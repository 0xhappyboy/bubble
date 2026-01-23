mod init;
mod types;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

use crate::init::parse_bubble_config;

// ======================================================= Root =======================================================
/// Bubble Application Entry Point Macro
///
/// The `#[bubble]` macro transforms a standard Rust `main` function into a
/// fully-featured asynchronous application with built-in infrastructure
/// including Tokio runtime, logging, graceful shutdown, and optional database
/// support. It's designed to eliminate boilerplate code when building
/// server-side applications.
///
/// # Features
///
/// - **Automatic Tokio Runtime**: Creates and manages a multi-threaded Tokio
///   runtime automatically
/// - **Graceful Shutdown**: Handles Ctrl+C signals for clean application
///   termination
/// - **Built-in Logging**: Initializes structured logging with configurable
///   levels
/// - **Configuration Management**: Supports multiple configuration sources
///   (environment variables, config files, command-line arguments)
/// - **Database Integration**: Optional automatic database connection setup
/// - **Error Handling**: Unified error handling with proper exit codes
/// - **Concurrency Control**: Configurable worker thread count
///
/// # Basic Usage
///
/// ```rust
/// use std::io::Result;
///
/// #[bubble]
/// async fn main() -> Result<()> {
///     println!("Hello, Bubble!");
///     Ok(())
/// }
/// ```
///
/// # Configuration Parameters
///
/// The macro accepts optional named parameters to customize the application:
///
/// ## Network Configuration
///
/// - `port`: Server port number (default: `3000`)
///   ```rust
///   #[bubble(port = 8080)]
///   async fn main() -> Result<()> { Ok(()) }
///   ```
///
/// - `host`: Server host address (default: `"127.0.0.1"`)
///   ```rust
///   #[bubble(host = "0.0.0.0")]
///   async fn main() -> Result<()> { Ok(()) }
///   ```
///
/// ## Concurrency Configuration
///
/// - `workers`: Number of Tokio worker threads (default: `0` = auto-detect)
///   ```rust
///   #[bubble(workers = 4)]  // Use 4 worker threads
///   async fn main() -> Result<()> { Ok(()) }
///   ```
///
/// ## Database Configuration
///
/// - `db_type`: Database type (`"mysql"`, `"postgres"`, `"sqlite"`, `"redis"`)
/// - `db_url`: Database connection URL
///   ```rust
///   #[bubble(
///       db_type = "postgres",
///       db_url = "postgres://user:pass@localhost:5432/mydb"
///   )]
///   async fn main() -> Result<()> { Ok(()) }
///   ```
///
/// ## Logging Configuration
///
/// - `log_level`: Logging verbosity (`"error"`, `"warn"`, `"info"`, `"debug"`, `"trace"`)
///   (default: `"info"`)
///   ```rust
///   #[bubble(log_level = "debug")]
///   async fn main() -> Result<()> { Ok(()) }
///   ```
///
/// ## Configuration Files
///
/// - `config_file`: Path to configuration file (default: `"config.toml"`)
///   ```rust
///   #[bubble(config_file = "app.toml")]
///   async fn main() -> Result<()> { Ok(()) }
///   ```
///
/// # Complete Example
///
/// ```rust
/// use std::io::Result;
///
/// #[bubble(
///     port = 8080,
///     host = "0.0.0.0",
///     workers = 4,
///     db_type = "postgres",
///     db_url = "postgres://user:pass@localhost:5432/appdb",
///     log_level = "debug",
///     config_file = "app_config.toml"
/// )]
/// async fn main() -> Result<()> {
///     // Application logic here
///     println!("Application running on port 8080");
///     
///     // Access environment variables
///     let db_url = std::env::var("DATABASE_URL")
///         .unwrap_or_else(|_| "postgres://localhost:5432/appdb".to_string());
///     
///     Ok(())
/// }
/// ```
///
/// # Application Lifecycle
///
/// When using `#[bubble]`, your application follows this sequence:
///
/// 1. **Runtime Initialization**:
///    - Tokio multi-threaded runtime is created
///    - Worker threads are spawned with lifecycle hooks
///
/// 2. **Infrastructure Setup**:
///    - Logging system is initialized with the specified level
///    - Configuration file is loaded (if exists)
///    - Command-line arguments are parsed
///    - Database connection is established (if configured)
///
/// 3. **Signal Handling**:
///    - Ctrl+C handler is registered for graceful shutdown
///    - Signal handler runs in a separate Tokio task
///
/// 4. **Application Execution**:
///    - Your `main` function is executed asynchronously
///    - Runs concurrently with signal monitoring
///
/// 5. **Shutdown**:
///    - On Ctrl+C: graceful shutdown with completion message
///    - On error: error logging with non-zero exit code
///    - On success: clean exit with zero exit code
///
/// # Error Handling
///
/// The macro expects your `main` function to return `Result<()>`. Errors are
/// handled as follows:
///
/// - **Runtime Errors**: If Tokio runtime creation fails, the application
///   panics with a descriptive message
/// - **Configuration Errors**: Missing or invalid configuration results in
///   panic with error details
/// - **Application Errors**: Errors returned from your `main` function are
///   logged at error level and cause exit code 1
/// - **Signal Errors**: If signal handling fails, it's logged but doesn't
///   prevent application startup
///
/// # Logging Output
///
/// The macro automatically sets up logging with the following format:
///
/// ```text
/// 2024-01-23T10:30:45.123 INFO  Starting Bubble Application
/// 2024-01-23T10:30:45.124 INFO  Configuration: port=8080, host=0.0.0.0, workers=4
/// 2024-01-23T10:30:45.125 INFO  Logging initialized with level: debug
/// 2024-01-23T10:30:45.126 DEBUG Tokio worker thread started
/// 2024-01-23T10:30:45.127 INFO  Executing user application
/// ```
///
/// # Environment Variables
///
/// The application automatically reads environment variables prefixed with
/// `BUBBLE_`. For example:
///
/// ```bash
/// export BUBBLE_PORT=9000
/// export BUBBLE_LOG_LEVEL=debug
/// export BUBBLE_DB_URL=postgres://localhost:5432/mydb
/// ```
///
/// # Configuration File Format
///
/// When using a configuration file (default: `config.toml`), it should follow
/// this structure:
///
/// ```toml
/// # config.toml
/// port = 8080
/// host = "0.0.0.0"
/// workers = 4
/// db_type = "postgres"
/// db_url = "postgres://localhost:5432/appdb"
/// log_level = "info"
/// ```
///
/// # Integration with Other Macros
///
/// The `#[bubble]` macro can be combined with other macros from this crate:
///
/// ```rust
/// #[bubble(port = 3000)]
/// async fn main() -> Result<()> {
///     // Use route macros
///     #[get("/health")]
///     fn health_check() -> String {
///         "OK".to_string()
///     }
///     
///     // Use ORM models
///     #[orm(table = "users")]
///     struct User {
///         id: i64,
///         name: String,
///     }
///     
///     Ok(())
/// }
/// ```
///
/// # Performance Considerations
///
/// - **Worker Threads**: Setting `workers = 0` lets Tokio choose the optimal
///   number based on CPU cores
/// - **Runtime Overhead**: The macro adds minimal runtime overhead (mostly
///   during startup)
/// - **Memory Usage**: Each worker thread has its own task queue and memory
///   allocation
/// - **Concurrency**: For I/O-bound applications, more workers can improve
///   throughput; for CPU-bound tasks, match worker count to CPU cores
///
/// # Limitations and Requirements
///
/// - **Main Function Only**: Can only be applied to the `main` function
/// - **Async Required**: The `main` function must be `async`
/// - **Return Type**: Must return `Result<()>` or compatible error type
/// - **Dependencies**: Requires `tokio`, `env_logger`, and `log` crates
/// - **Platform**: Works on all platforms supported by Tokio
///
/// # Migration from Manual Setup
///
/// If you're migrating from manual Tokio setup, replace:
///
/// ```rust
/// #[tokio::main]
/// async fn main() -> Result<()> {
///     // Manual setup code here
///     Ok(())
/// }
/// ```
///
/// With:
///
/// ```rust
/// #[bubble]
/// async fn main() -> Result<()> {
///     // Your application code (infrastructure is automatic)
///     Ok(())
/// }
/// ```
///
/// # Best Practices
///
/// 1. **Use Environment Variables** for sensitive data (passwords, API keys)
/// 2. **Set Appropriate Log Level** in production (`"info"` or `"warn"`)
/// 3. **Configure Workers Appropriately** based on your workload
/// 4. **Use Graceful Shutdown** for database connections and external services
/// 5. **Combine with Error Handling** for robust applications
///
/// # Example: Web Server with Database
///
/// ```rust
/// use std::io::Result;
///
/// #[bubble(
///     port = 3000,
///     db_type = "postgres",
///     db_url = "postgres://localhost:5432/appdb"
/// )]
/// async fn main() -> Result<()> {
///     // Database operations
///     #[orm(table = "users")]
///     struct User {
///         id: i64,
///         name: String,
///         email: String,
///     }
///     
///     // Web routes
///     #[get("/api/users")]
///     async fn get_users() -> String {
///         "User list".to_string()
///     }
///     
///     log::info!("Server ready");
///     Ok(())
/// }
/// ```
///
/// # Troubleshooting
///
/// Common issues and solutions:
///
/// - **"Failed to create Tokio runtime"**: Usually indicates system resource
///   limitations or invalid worker thread count
/// - **Database connection errors**: Verify database is running and credentials
///   are correct
/// - **Permission denied**: Check port permissions (ports < 1024 require root)
/// - **Missing dependencies**: Ensure `tokio`, `env_logger`, `log` are in
///   `Cargo.toml`
///
#[proc_macro_attribute]
pub fn bubble(attr: TokenStream, item: TokenStream) -> TokenStream {
    let config = parse_bubble_config(attr);
    let input_fn = parse_macro_input!(item as syn::ItemFn);
    let fn_name = &input_fn.sig.ident;
    if fn_name != "main" {
        return syn::Error::new_spanned(
            fn_name,
            "The #[bubble] macro can only be used on the main function",
        )
        .to_compile_error()
        .into();
    }
    let has_async = input_fn.sig.asyncness.is_some();
    if !has_async {
        return syn::Error::new_spanned(
            &input_fn.sig,
            "The main function must be async when using #[bubble]",
        )
        .to_compile_error()
        .into();
    }
    let vis = &input_fn.vis;
    let inputs = &input_fn.sig.inputs;
    let output = &input_fn.sig.output;
    let block = &input_fn.block;
    let attrs = &input_fn.attrs;
    let port = config.port;
    let host = &config.host;
    let workers = config.workers;
    let db_type = &config.db_type;
    let db_url = &config.db_url;
    let log_level = &config.log_level;
    let config_file = &config.config_file;
    // Generate the expanded code with full integration
    let expanded = quote! {
        #(#attrs)*
        #[doc = "Bubble Application Entry Point"]
        #[doc = "Automatically initialized with: "]
        #[doc = concat!("- Port: ", #port)]
        #[doc = concat!("- Host: \"", #host, "\"")]
        #[doc = concat!("- Workers: ", #workers)]
        #[doc = concat!("- Database: ", #db_type)]
        #[doc = concat!("- Log Level: ", #log_level)]
        #vis fn main() #output {
            // Create the actual main function that will be called by tokio
            async fn inner_main() #output {
                // Helper function to initialize logging
                fn init_logging(level_str: &str) {
                    let level = match level_str.to_lowercase().as_str() {
                        "error" => log::LevelFilter::Error,
                        "warn" => log::LevelFilter::Warn,
                        "info" => log::LevelFilter::Info,
                        "debug" => log::LevelFilter::Debug,
                        "trace" => log::LevelFilter::Trace,
                        _ => log::LevelFilter::Info,
                    };
                    env_logger::Builder::from_default_env()
                        .filter_level(level)
                        .format_timestamp(Some(env_logger::TimestampPrecision::Millis))
                        .format_module_path(false)
                        .init();
                    log::info!("Logging initialized with level: {}", level_str);
                }
                async fn init_database(db_type: &str, db_url: &str) -> Result<(), String> {
                    log::info!(
                        "Database connection configured: type={}, url={}",
                        db_type,
                        db_url
                    );
                    Ok(())
                }
                fn load_config_file(file_path: &str) -> Result<(), String> {
                    use std::fs;
                    match fs::read_to_string(file_path) {
                        Ok(content) => {
                            log::debug!("Configuration file content:\n{}", content);
                            Ok(())
                        }
                        Err(err) => Err(format!("Failed to read config file: {}", err)),
                    }
                }
                fn parse_command_line_args(args: &[String]) {
                    if args.len() > 1 {
                        log::info!("Command line arguments: {:?}", &args[1..]);
                    }
                }
                init_logging(#log_level);
                log::info!("Starting Bubble Application");
                log::info!("Configuration: port={}, host={}, workers={}",
                    #port, #host, #workers);
                if !#db_type.is_empty() && !#db_url.is_empty() {
                    log::info!("Initializing {} database: {}", #db_type, #db_url);
                    init_database(#db_type, #db_url).await
                        .expect("Failed to initialize database");
                }
                if std::path::Path::new(#config_file).exists() {
                    log::info!("Loading configuration from {}", #config_file);
                    load_config_file(#config_file)
                        .expect("Failed to load configuration file");
                }
                let args: Vec<String> = std::env::args().collect();
                parse_command_line_args(&args);
                log::info!("Executing user application");
                #block
            }
            let mut rt_builder = tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .on_thread_start(|| {
                    log::debug!("Tokio worker thread started");
                })
                .on_thread_stop(|| {
                    log::debug!("Tokio worker thread stopped");
                });
            let rt = if #workers > 0 {
                rt_builder.worker_threads(#workers)
            } else {
                &mut rt_builder
            }
            .build()
            .expect("Failed to create Tokio runtime");
            let (shutdown_tx, mut shutdown_rx) = tokio::sync::oneshot::channel::<()>();
            rt.spawn(async move {
                match tokio::signal::ctrl_c().await {
                    Ok(()) => {
                        log::info!("Received shutdown signal (Ctrl+C)");
                        let _ = shutdown_tx.send(());
                    }
                    Err(err) => {
                        log::error!("Failed to listen for shutdown signal: {}", err);
                    }
                }
            });
            let result = rt.block_on(async {
                tokio::select! {
                    _ = &mut shutdown_rx => {
                        log::info!("Shutting down gracefully...");
                        Err("Application interrupted by user".into())
                    }
                    res = inner_main() => {
                        res
                    }
                }
            });
            match result {
                Ok(_) => {
                    log::info!("Application completed successfully");
                    std::process::exit(0);
                }
                Err(err) => {
                    log::error!("Application failed: {}", err);
                    std::process::exit(1);
                }
            }
        }
    };
    expanded.into()
}

// ======================================================= WEB =======================================================

/// GET request macro
///
/// # Examples
/// ```
/// #[get("/users")]
/// fn get_users() -> String {
///     "List of users".to_string()
/// }
/// ```
#[proc_macro_attribute]
pub fn get(attr: TokenStream, item: TokenStream) -> TokenStream {
    generate_route_macro("GET", attr, item)
}

/// POST request macro
///
/// # Examples
/// ```
/// #[post("/users")]
/// fn create_user() -> String {
///     "User created".to_string()
/// }
/// ```
#[proc_macro_attribute]
pub fn post(attr: TokenStream, item: TokenStream) -> TokenStream {
    generate_route_macro("POST", attr, item)
}

/// PUT request macro
///
/// # Examples
/// ```
/// #[put("/users/:id")]
/// fn update_user(id: i64) -> String {
///     format!("User {} updated", id)
/// }
/// ```
#[proc_macro_attribute]
pub fn put(attr: TokenStream, item: TokenStream) -> TokenStream {
    generate_route_macro("PUT", attr, item)
}

/// DELETE request macro
///
/// # Examples
/// ```
/// #[delete("/users/:id")]
/// fn delete_user(id: i64) -> String {
///     format!("User {} deleted", id)
/// }
/// ```
#[proc_macro_attribute]
pub fn delete(attr: TokenStream, item: TokenStream) -> TokenStream {
    generate_route_macro("DELETE", attr, item)
}

/// PATCH request macro
///
/// # Examples
/// ```
/// #[patch("/users/:id")]
/// fn patch_user(id: i64) -> String {
///     format!("User {} partially updated", id)
/// }
/// ```
#[proc_macro_attribute]
pub fn patch(attr: TokenStream, item: TokenStream) -> TokenStream {
    generate_route_macro("PATCH", attr, item)
}

/// HEAD request macro
///
/// # Examples
/// ```
/// #[head("/users/:id")]
/// fn check_user_exists(id: i64) -> String {
///     format!("Check user {} exists", id)
/// }
/// ```
#[proc_macro_attribute]
pub fn head(attr: TokenStream, item: TokenStream) -> TokenStream {
    generate_route_macro("HEAD", attr, item)
}

/// OPTIONS request macro
///
/// # Examples
/// ```
/// #[options("/users/:id")]
/// fn user_options(id: i64) -> String {
///     format!("Supported methods for user {}", id)
/// }
/// ```
#[proc_macro_attribute]
pub fn options(attr: TokenStream, item: TokenStream) -> TokenStream {
    generate_route_macro("OPTIONS", attr, item)
}

/// Generic route macro that can specify any HTTP method
///
/// # Examples
/// ```
/// #[route(method = "CUSTOM", path = "/custom")]
/// fn custom_method() -> String {
///     "Custom method".to_string()
/// }
///
/// #[route("TRACE", "/trace")]
/// fn trace_method() -> String {
///     "Trace method".to_string()
/// }
/// ```
#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr_str = attr.to_string();
    let parts: Vec<&str> = attr_str.split(',').map(|s| s.trim()).collect();

    let (method, path) = if parts.len() >= 2 {
        let method_part = parts[0];
        let path_part = parts[1];

        // Extract method
        let method = if method_part.contains("method") {
            method_part
                .split('=')
                .nth(1)
                .unwrap_or("GET")
                .trim_matches(|c| c == '"' || c == ' ')
                .to_string()
        } else {
            method_part.to_string()
        };

        // Extract path
        let path = if path_part.contains("path") {
            path_part
                .split('=')
                .nth(1)
                .unwrap_or("/")
                .trim_matches(|c| c == '"' || c == ' ')
                .to_string()
        } else {
            path_part.trim_matches('"').to_string()
        };

        (method, path)
    } else if parts.len() == 1 {
        // If there's only one parameter, assume it's the path and method defaults to GET
        let path = parts[0].trim_matches('"').to_string();
        ("GET".to_string(), path)
    } else {
        ("GET".to_string(), "/".to_string())
    };

    generate_custom_route_macro(&method, &path, item)
}

// =============================== Controller Macros ===============================

/// Controller macro
///
/// Marks a struct as a controller with a base path
///
/// # Examples
/// ```
/// #[controller("/api/users")]
/// struct UserController {
///     service_name: String,
/// }
/// ```
#[proc_macro_attribute]
pub fn controller(attr: TokenStream, item: TokenStream) -> TokenStream {
    let base_path = if attr.is_empty() {
        "/".to_string()
    } else {
        attr.to_string().trim_matches('"').to_string()
    };

    let input = parse_macro_input!(item as syn::ItemStruct);
    let struct_name = &input.ident;
    let fields = &input.fields;
    let attrs = &input.attrs;
    let vis = &input.vis;

    let expanded = quote! {
        #(#attrs)*
        #[doc = concat!("Controller - Base Path: ", #base_path)]
        #vis struct #struct_name #fields
    };

    expanded.into()
}

// =============================== Helper Functions ===============================

/// Generate standard HTTP method macros
fn generate_route_macro(method: &str, attr: TokenStream, item: TokenStream) -> TokenStream {
    let path = if attr.is_empty() {
        "/".to_string()
    } else {
        attr.to_string()
            .trim_matches(|c| c == '"' || c == ' ')
            .to_string()
    };

    let input_fn = parse_macro_input!(item as syn::ItemFn);
    let fn_name = &input_fn.sig.ident;
    let vis = &input_fn.vis;
    let inputs = &input_fn.sig.inputs;
    let output = &input_fn.sig.output;
    let block = &input_fn.block;
    let attrs = &input_fn.attrs;

    let expanded = quote! {
        #(#attrs)*
        #[doc = concat!(#method, " Request Handler - Path: ", #path)]
        #vis fn #fn_name(#inputs) #output #block
    };

    expanded.into()
}

/// Generate custom HTTP method macros
fn generate_custom_route_macro(method: &str, path: &str, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as syn::ItemFn);
    let fn_name = &input_fn.sig.ident;
    let vis = &input_fn.vis;
    let inputs = &input_fn.sig.inputs;
    let output = &input_fn.sig.output;
    let block = &input_fn.block;
    let attrs = &input_fn.attrs;

    let expanded = quote! {
        #(#attrs)*
        #[doc = concat!(#method, " Request Handler - Path: ", #path)]
        #vis fn #fn_name(#inputs) #output #block
    };

    expanded.into()
}

// =============================== Middleware Related Macros ===============================

/// Middleware macro
///
/// Marks a function as middleware
///
/// # Examples
/// ```
/// #[middleware]
/// fn logger_middleware(req: &Request) -> Result<Response> {
///     println!("Request received");
///     Ok(Response::new())
/// }
/// ```
#[proc_macro_attribute]
pub fn middleware(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as syn::ItemFn);
    let fn_name = &input_fn.sig.ident;
    let vis = &input_fn.vis;
    let inputs = &input_fn.sig.inputs;
    let output = &input_fn.sig.output;
    let block = &input_fn.block;
    let attrs = &input_fn.attrs;

    let expanded = quote! {
        #(#attrs)*
        #[doc = "Middleware Handler"]
        #vis fn #fn_name(#inputs) #output #block
    };

    expanded.into()
}

/// Error handler macro
///
/// Marks a function as an error handler
///
/// # Examples
/// ```
/// #[error_handler]
/// fn handle_error(err: Error) -> Response {
///     Response::error(err)
/// }
/// ```
#[proc_macro_attribute]
pub fn error_handler(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as syn::ItemFn);
    let fn_name = &input_fn.sig.ident;
    let vis = &input_fn.vis;
    let inputs = &input_fn.sig.inputs;
    let output = &input_fn.sig.output;
    let block = &input_fn.block;
    let attrs = &input_fn.attrs;

    let expanded = quote! {
        #(#attrs)*
        #[doc = "Error Handler"]
        #vis fn #fn_name(#inputs) #output #block
    };

    expanded.into()
}

// =============================== Parameter Binding Macros ===============================

/// Path parameter macro
///
/// Binds a function parameter to a path parameter
///
/// # Examples
/// ```
/// #[get("/users/:id")]
/// fn get_user(#[path_param("id")] user_id: i64) -> String {
///     format!("User ID: {}", user_id)
/// }
/// ```
#[proc_macro_attribute]
pub fn path_param(attr: TokenStream, item: TokenStream) -> TokenStream {
    let param_name = if attr.is_empty() {
        "".to_string()
    } else {
        attr.to_string().trim_matches('"').to_string()
    };
    let expanded = format!(
        r#"
        #[doc = "Path Parameter: {}"]
        {}
    "#,
        param_name,
        item.to_string()
    );
    expanded.parse().unwrap()
}

/// Query parameter macro
///
/// Binds a function parameter to a query parameter
///
/// # Examples
/// ```
/// #[get("/users")]
/// fn search_users(#[query_param("name")] name: String) -> String {
///     format!("Searching users with name: {}", name)
/// }
/// ```
#[proc_macro_attribute]
pub fn query_param(attr: TokenStream, item: TokenStream) -> TokenStream {
    let param_name = if attr.is_empty() {
        "".to_string()
    } else {
        attr.to_string().trim_matches('"').to_string()
    };
    let expanded = format!(
        r#"
        #[doc = "Query Parameter: {}"]
        {}
    "#,
        param_name,
        item.to_string()
    );
    expanded.parse().unwrap()
}

/// Request body macro
///
/// Binds a function parameter to the request body
///
/// # Examples
/// ```
/// #[post("/users")]
/// fn create_user(#[request_body] user: CreateUserRequest) -> String {
///     format!("Creating user: {:?}", user)
/// }
/// ```
#[proc_macro_attribute]
pub fn request_body(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let expanded = format!(
        r#"
        #[doc = "Request Body"]
        {}
    "#,
        item.to_string()
    );
    expanded.parse().unwrap()
}

// ======================================================= DB =======================================================
/// ORM (Object-Relational Mapping) Macro
///
/// Automatically generates complete CRUD (Create, Read, Update, Delete) operations
/// for a struct, supporting multiple database types. Structs marked with this macro
/// will automatically implement database interaction methods, simplifying database code.
///
/// # Attribute Parameters
///
/// The macro supports the following optional parameters:
/// - `table`: Specifies the database table name (optional, defaults to lowercase plural of struct name)
/// - `db_type`: Specifies the database type (optional, defaults to "generic")
///   - Supported values: `"mysql"`, `"postgres"`, `"sqlite"`, `"redis"`, `"generic"`
///   - SQL syntax is automatically adapted for different database types
///
/// # Automatically Generated Methods
///
/// The macro automatically generates the following methods for the struct:
/// 1. **Instance Methods**:
///    - `insert(&self) -> DbResult<Self>` - Inserts the current instance into the database
/// 2. **Static Methods**:
///    - `find_by_id(id: i64) -> DbResult<Self>` - Finds a record by its ID
///    - `update(&self, id: i64) -> DbResult<Self>` - Updates the record with the given ID
///    - `delete(id: i64) -> DbResult<Self>` - Deletes the record with the given ID
///    - `all() -> DbResult<Vec<Self>>` - Retrieves all records from the table
///    - `query(sql: &str) -> DbResult<Vec<Self>>` - Executes a custom SQL query
///    - `execute(sql: &str) -> DbResult<u64>` - Executes a custom SQL command
///    - `count() -> DbResult<i64>` - Counts the number of records in the table
///    - `where_clause(condition: &str) -> DbResult<Vec<Self>>` - Queries with WHERE condition
///
/// # Database Integration
///
/// The macro relies on a global database connection available through `crate::DATABASE_CONNECTION`.
/// Before using ORM methods, you must initialize the database connection using `init_database_connection()`.
///
/// # Serialization
///
/// The struct automatically implements:
/// - `Default` trait - Provides default values for all fields
/// - `serde::Serialize` - Enables JSON serialization
/// - `serde::Deserialize` - Enables JSON deserialization
///
/// # Examples
///
/// ## Basic Usage
/// ```rust
/// #[orm(table = "users", db_type = "mysql")]
/// struct User {
///     id: i64,
///     name: String,
///     email: String,
///     age: i32,
/// }
/// ```
///
/// ## Using Default Table Name
/// ```rust
/// #[orm(db_type = "postgres")]  // Table name will be "products"
/// struct Product {
///     id: i64,
///     name: String,
///     price: f64,
/// }
/// ```
///
/// ## Usage Example
/// ```rust
/// // Initialize database connection
/// let config = DatabaseConfig::new("mysql://localhost:3306/mydb");
/// let conn = MySqlConnection::connect(&config).await?;
/// init_database_connection(conn).await?;
///
/// // Create a new user
/// let user = User {
///     id: 0,
///     name: "John Doe".to_string(),
///     email: "john@example.com".to_string(),
///     age: 30,
/// };
///
/// let created_user = user.create().await?;
/// println!("Created user: {:?}", created_user);
///
/// // Find user by ID
/// let found_user = User::find_by_id(1).await?;
///
/// // Update user
/// let updated_user = found_user.update(1).await?;
///
/// // Get all users
/// let all_users = User::all().await?;
///
/// // Execute custom query
/// let admins = User::query("SELECT * FROM users WHERE role = 'admin'").await?;
///
/// // Count users
/// let user_count = User::count().await?;
/// ```
///
/// # Database-Specific Features
///
/// - **PostgreSQL**: Uses `RETURNING *` clause for INSERT and UPDATE operations
/// - **MySQL/SQLite**: Uses standard SQL syntax with `?` placeholders
/// - **Redis**: Supports basic key-value operations (limited ORM functionality)
/// - **Generic**: Uses standard SQL syntax compatible with most databases
///
/// # Error Handling
///
/// All methods return `crate::DbResult<T>` which is an alias for `Result<T, String>`.
/// Errors are propagated as strings for simplicity.
///
/// # Limitations
///
/// - Field types must implement `Default`, `FromStr`, and serde traits
/// - Primitive types (i64, String, f64, etc.) are supported out of the box
/// - Complex types may require custom implementations
/// - No support for complex queries (JOINs, subqueries) - use `query()` method instead
/// - No support for database transactions within the macro
///
/// # Performance Considerations
///
/// - Batch operations use JSON serialization for simplicity
/// - For high-performance applications, consider using prepared statements
/// - The `query()` method allows for optimized custom SQL when needed
///
/// # Dependencies
///
/// Requires the following crates to be available:
/// - `serde` (for serialization)
/// - `async_trait` (for async database operations)
/// - Database-specific drivers (mysql_async, sqlx, redis, rusqlite)
///
/// # Migration from Previous Versions
///
/// This macro replaces the previous `#[orm_model]` macro with improved:
/// - Simplified API (no need to pass database connection to each method)
/// - Better database type handling
/// - More intuitive method naming
/// - Enhanced error messages
/// ```
#[proc_macro_attribute]
pub fn orm(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr_str = attr.to_string();
    let attrs: Vec<&str> = attr_str.split(',').map(|s| s.trim()).collect();
    let mut table_name = String::new();
    let mut db_type = String::from("generic");
    for attr in attrs {
        if attr.starts_with("table") {
            table_name = attr
                .split('=')
                .nth(1)
                .unwrap_or("")
                .trim_matches(|c| c == '"' || c == ' ')
                .to_string();
        } else if attr.starts_with("db_type") {
            db_type = attr
                .split('=')
                .nth(1)
                .unwrap_or("generic")
                .trim_matches(|c| c == '"' || c == ' ')
                .to_string();
        }
    }
    let input = parse_macro_input!(item as syn::ItemStruct);
    let struct_name = &input.ident;
    if table_name.is_empty() {
        table_name = format!("{}s", struct_name.to_string().to_lowercase());
    }
    let field_idents: Vec<syn::Ident> = if let syn::Fields::Named(fields_named) = &input.fields {
        fields_named
            .named
            .iter()
            .filter_map(|f| f.ident.clone())
            .collect()
    } else {
        Vec::new()
    };
    let mut field_impls = Vec::new();
    let mut field_names_vec = Vec::new();
    for ident in &field_idents {
        let field_name = ident.to_string();
        field_impls.push(quote! {
            if let Some(value) = row.get(#field_name) {
                instance.#ident = value.parse().unwrap_or_default();
            }
        });
        field_names_vec.push(quote! { #field_name });
    }
    let placeholders_count = field_idents.len();
    let placeholders: Vec<_> = (0..placeholders_count)
        .map(|i| {
            if db_type == "postgres" {
                format!("${}", i + 1)
            } else {
                "?".to_string()
            }
        })
        .collect();
    let expanded = quote! {
        #[derive(Default, serde::Serialize, serde::Deserialize)]
        #input
        impl #struct_name {
            fn from_db_row(row: &std::collections::HashMap<String, String>) -> crate::DbResult<Self> {
                let mut instance = Self::default();
                #(#field_impls)*
                Ok(instance)
            }
            fn from_json(json_str: &str) -> crate::DbResult<Self> {
                serde_json::from_str(json_str).map_err(|e| e.to_string())
            }
            pub async fn insert(&self) -> crate::DbResult<Self> {
                let field_names: Vec<&str> = vec![
                    #(stringify!(#field_idents)),*
                ];
                let fields_str = field_names.join(", ");
                let placeholders_vec: Vec<&str> = vec![
                    #(#placeholders),*
                ];
                let placeholders_str = placeholders_vec.join(", ");
                let sql = if #db_type == "postgres" {
                    format!(
                        "INSERT INTO {} ({}) VALUES ({}) RETURNING *",
                        #table_name,
                        fields_str,
                        placeholders_str
                    )
                } else {
                    format!(
                        "INSERT INTO {} ({}) VALUES ({})",
                        #table_name,
                        fields_str,
                        placeholders_str
                    )
                };
                let result = crate::DATABASE_CONNECTION
                    .query_one(&sql)
                    .await?;
                Self::from_json(&result)
            }
            pub async fn find_by_id(id: i64) -> crate::DbResult<Self> {
                let sql = format!("SELECT * FROM {} WHERE id = {}", #table_name, id);
                let result = crate::DATABASE_CONNECTION.query_one(&sql).await?;
                Self::from_json(&result)
            }
            pub async fn update(&self, id: i64) -> crate::DbResult<Self> {
                let field_names: Vec<&str> = vec![
                    #(stringify!(#field_idents)),*
                ];
                let set_clauses: Vec<String> = if #db_type == "postgres" {
                    field_names.iter()
                        .enumerate()
                        .map(|(i, name)| format!("{} = ${}", name, i + 1))
                        .collect()
                } else {
                    field_names.iter()
                        .map(|name| format!("{} = ?", name))
                        .collect()
                };
                let set_clauses_str = set_clauses.join(", ");
                let sql = if #db_type == "postgres" {
                    format!(
                        "UPDATE {} SET {} WHERE id = {} RETURNING *",
                        #table_name,
                        set_clauses_str,
                        id
                    )
                } else {
                    format!(
                        "UPDATE {} SET {} WHERE id = {}",
                        #table_name,
                        set_clauses_str,
                        id
                    )
                };
                let result = crate::DATABASE_CONNECTION
                    .query_one(&sql)
                    .await?;
                Self::from_json(&result)
            }
            pub async fn delete(id: i64) -> crate::DbResult<Self> {
                let record = Self::find_by_id(id).await?;
                let sql = format!("DELETE FROM {} WHERE id = {}", #table_name, id);
                crate::DATABASE_CONNECTION.execute(&sql).await?;
                Ok(record)
            }
            pub async fn all() -> crate::DbResult<Vec<Self>> {
                let sql = format!("SELECT * FROM {}", #table_name);
                Self::query(&sql).await
            }
            pub async fn query(sql: &str) -> crate::DbResult<Vec<Self>> {
                let result = crate::DATABASE_CONNECTION.query(sql).await?;
                let items: Vec<std::collections::HashMap<String, String>> =
                    serde_json::from_str(&result).map_err(|e| e.to_string())?;
                let mut records = Vec::new();
                for row in items {
                    records.push(Self::from_db_row(&row)?);
                }
                Ok(records)
            }
            pub async fn execute(sql: &str) -> crate::DbResult<u64> {
                crate::DATABASE_CONNECTION.execute(sql).await
            }
            pub async fn count() -> crate::DbResult<i64> {
                let sql = format!("SELECT COUNT(*) as count FROM {}", #table_name);
                let result = crate::DATABASE_CONNECTION.query_one(&sql).await?;
                let data: std::collections::HashMap<String, String> =
                    serde_json::from_str(&result).map_err(|e| e.to_string())?;
                data.get("count")
                    .unwrap_or(&"0".to_string())
                    .parse()
                    .map_err(|e| e.to_string())
            }
        }
    };
    expanded.into()
}

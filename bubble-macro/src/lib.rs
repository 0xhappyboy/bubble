mod types;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

// =============================== Root ===============================

// =============================== WEB ===============================

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

// =============================== DB ===============================
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

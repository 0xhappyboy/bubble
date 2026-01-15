use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

// =============================== MVC ===============================

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

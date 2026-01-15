use std::collections::HashMap;
use std::fmt::{Debug, Display};

/// HTTP Request structure
#[derive(Debug, Clone, Default)]
pub struct Request {
    /// HTTP method
    pub method: HttpMethod,
    /// Request path
    pub path: String,
    /// Query parameters
    pub query_params: HashMap<String, String>,
    /// Path parameters
    pub path_params: HashMap<String, String>,
    /// Request headers
    pub headers: HashMap<String, String>,
    /// Request body (raw bytes)
    pub body: Vec<u8>,
    /// Request context
    pub context: Context,
}

/// HTTP Response structure
#[derive(Debug, Clone, Default)]
pub struct Response {
    /// HTTP status code
    pub status: HttpStatus,
    /// Response headers
    pub headers: HashMap<String, String>,
    /// Response body
    pub body: ResponseBody,
    /// Response metadata
    pub metadata: ResponseMetadata,
}

/// Response body enum supporting multiple formats
#[derive(Debug, Clone)]
pub enum ResponseBody {
    /// Text response
    Text(String),
    /// JSON response
    Json(serde_json::Value),
    /// Binary data
    Binary(Vec<u8>),
    /// Empty response
    Empty,
}

impl Default for ResponseBody {
    fn default() -> Self {
        ResponseBody::Empty
    }
}

/// Response metadata
#[derive(Debug, Clone, Default)]
pub struct ResponseMetadata {
    /// Response duration in milliseconds
    pub duration: u64,
    /// Whether response is cached
    pub cached: bool,
    /// Additional metadata
    pub extra: HashMap<String, String>,
}

/// HTTP method enumeration
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum HttpMethod {
    #[default]
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    HEAD,
    OPTIONS,
    TRACE,
    CONNECT,
    CUSTOM(String),
}

/// HTTP status code wrapper
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct HttpStatus {
    /// Status code number
    pub code: u16,
    /// Status message
    pub message: String,
}

/// Request context for passing contextual information
#[derive(Debug, Clone, Default)]
pub struct Context {
    /// Unique request identifier
    pub request_id: String,
    /// User session information
    pub session: Option<Session>,
    /// Authentication information
    pub auth: Option<AuthInfo>,
    /// Locale information
    pub locale: String,
    /// Custom context data
    pub data: HashMap<String, String>,
}

/// User session information
#[derive(Debug, Clone)]
pub struct Session {
    /// Session ID
    pub id: String,
    /// User ID
    pub user_id: String,
    /// Session creation timestamp
    pub created_at: u64,
    /// Session expiration timestamp
    pub expires_at: u64,
    /// Session data storage
    pub data: HashMap<String, String>,
}

/// Authentication information
#[derive(Debug, Clone)]
pub struct AuthInfo {
    /// Authenticated user ID
    pub user_id: String,
    /// User roles
    pub roles: Vec<String>,
    /// User permissions
    pub permissions: Vec<String>,
    /// Authentication token
    pub token: String,
}

/// Error type for framework operations
#[derive(Debug, Clone)]
pub struct Error {
    /// Error code
    pub code: String,
    /// Error message
    pub message: String,
    /// Error details
    pub details: Option<HashMap<String, String>>,
}

/// Route configuration
#[derive(Debug, Clone)]
pub struct Route {
    /// HTTP method
    pub method: HttpMethod,
    /// Route path pattern
    pub path: String,
    /// Handler function name
    pub handler: String,
    /// Middleware chain
    pub middleware: Vec<String>,
}

/// Application configuration
#[derive(Debug, Clone, Default)]
pub struct AppConfig {
    /// Server host address
    pub host: String,
    /// Server port
    pub port: u16,
    /// Database connection URL
    pub database_url: String,
    /// Redis connection URL
    pub redis_url: String,
    /// JWT secret key
    pub jwt_secret: String,
    /// CORS configuration
    pub cors: CorsConfig,
}

/// CORS configuration
#[derive(Debug, Clone, Default)]
pub struct CorsConfig {
    /// Allowed origins
    pub allowed_origins: Vec<String>,
    /// Allowed methods
    pub allowed_methods: Vec<String>,
    /// Allowed headers
    pub allowed_headers: Vec<String>,
    /// Allow credentials
    pub allow_credentials: bool,
}

/// Middleware trait definition
pub trait Middleware: Send + Sync {
    /// Process request before handler
    fn pre_process(&self, request: &mut Request) -> Result<(), Error>;
    /// Process response after handler
    fn post_process(&self, response: &mut Response) -> Result<(), Error>;
}

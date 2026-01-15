// Core framework system types (completely web-independent)
use std::any::Any;
use std::collections::HashMap;
use std::error::Error as StdError;
use std::fmt::{Debug, Display};
use std::sync::Arc;

/// Result type alias for the entire framework
pub type FrameworkResult<T> = Result<T, FrameworkError>;

/// Generic configuration container that can hold any type
#[derive(Debug, Clone)]
pub struct Config {
    /// Unique configuration identifier
    pub id: String,
    /// Configuration data as key-value pairs
    pub values: HashMap<String, ConfigValue>,
    /// Configuration metadata
    pub metadata: ConfigMetadata,
}

/// Configuration value that can be of different types
#[derive(Debug, Clone)]
pub enum ConfigValue {
    /// String value
    String(String),
    /// Integer value
    Int(i64),
    /// Float value
    Float(f64),
    /// Boolean value
    Bool(bool),
    /// List of values
    List(Vec<ConfigValue>),
    /// Nested configuration
    Nested(Config),
}

/// Configuration metadata
#[derive(Debug, Clone)]
pub struct ConfigMetadata {
    /// Configuration source (file, env, database, etc.)
    pub source: String,
    /// Last updated timestamp
    pub last_updated: u64,
    /// Whether configuration is required
    pub required: bool,
    /// Configuration description
    pub description: String,
}

/// Extension trait for adding functionality to types
pub trait Extension: Send + Sync {
    /// Unique extension identifier
    fn id(&self) -> &str;

    /// Extension metadata
    fn metadata(&self) -> ExtensionMetadata;

    /// Extension lifecycle hook
    fn on_register(&self) -> FrameworkResult<()> {
        Ok(())
    }
}

/// Extension metadata
#[derive(Debug, Clone)]
pub struct ExtensionMetadata {
    /// Extension name
    pub name: String,
    /// Extension version
    pub version: String,
    /// Extension author
    pub author: String,
    /// Extension dependencies
    pub dependencies: Vec<String>,
    /// Whether extension is enabled by default
    pub enabled_by_default: bool,
}

/// Service abstraction for dependency injection
pub trait Service: Send + Sync {
    /// Service identifier
    fn service_id(&self) -> &str;

    /// Initialize service
    fn init(&mut self, config: &Config) -> FrameworkResult<()>;

    /// Start service
    fn start(&mut self) -> FrameworkResult<()>;

    /// Stop service
    fn stop(&mut self) -> FrameworkResult<()>;

    /// Get service status
    fn status(&self) -> ServiceStatus;
}

/// Service status enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceStatus {
    /// Service is stopped
    Stopped,
    /// Service is starting
    Starting,
    /// Service is running
    Running,
    /// Service is stopping
    Stopping,
    /// Service encountered an error
    Error,
    /// Service is in maintenance mode
    Maintenance,
}

/// Event abstraction for event-driven architecture
pub trait Event: Send + Sync {
    /// Event name
    fn event_name(&self) -> &str;

    /// Event payload
    fn payload(&self) -> &dyn Any;

    /// Event metadata
    fn metadata(&self) -> EventMetadata;
}

/// Event metadata
#[derive(Debug, Clone)]
pub struct EventMetadata {
    /// Unique event ID
    pub id: String,
    /// Event timestamp
    pub timestamp: u64,
    /// Event source
    pub source: String,
    /// Event correlation ID
    pub correlation_id: Option<String>,
    /// Event priority
    pub priority: EventPriority,
}

/// Event priority levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventPriority {
    /// Low priority event
    Low,
    /// Normal priority event
    Normal,
    /// High priority event
    High,
    /// Critical priority event
    Critical,
}

/// Event handler trait
pub trait EventHandler<E: Event>: Send + Sync {
    /// Handle an event
    fn handle(&self, event: Arc<E>) -> FrameworkResult<()>;
}

/// Framework error type
#[derive(Debug, Clone)]
pub struct FrameworkError {
    /// Error code
    pub code: String,
    /// Error message
    pub message: String,
    /// Error severity
    pub severity: ErrorSeverity,
    /// Stack trace (if available)
    pub stack_trace: Option<String>,
    /// Related errors
    pub causes: Vec<FrameworkError>,
    /// Additional error context
    pub context: HashMap<String, String>,
}

/// Error severity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorSeverity {
    /// Informational, not really an error
    Info,
    /// Warning, something unexpected but not critical
    Warning,
    /// Error, something went wrong
    Error,
    /// Critical error, system may be unstable
    Critical,
    /// Fatal error, system cannot continue
    Fatal,
}

/// Module descriptor for framework modules
#[derive(Debug, Clone)]
pub struct ModuleDescriptor {
    /// Module name
    pub name: String,
    /// Module version
    pub version: String,
    /// Module description
    pub description: String,
    /// Module author
    pub author: String,
    /// Module dependencies
    pub dependencies: Vec<Dependency>,
    /// Module exports
    pub exports: Vec<String>,
    /// Module configuration schema
    pub config_schema: Option<ConfigSchema>,
}

/// Dependency description
#[derive(Debug, Clone)]
pub struct Dependency {
    /// Dependency name
    pub name: String,
    /// Minimum version
    pub min_version: String,
    /// Maximum version
    pub max_version: Option<String>,
    /// Whether dependency is required
    pub required: bool,
}

/// Configuration schema for validation
#[derive(Debug, Clone)]
pub struct ConfigSchema {
    /// JSON schema definition
    pub schema: String,
    /// Default configuration values
    pub defaults: HashMap<String, ConfigValue>,
    /// Whether configuration can be updated at runtime
    pub runtime_updatable: bool,
}

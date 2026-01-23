use proc_macro::TokenStream;

/// Configuration for the bubble macro
pub(crate) struct BubbleConfig {
    pub(crate) port: u16,
    pub(crate) host: String,
    pub(crate) workers: usize,
    pub(crate) db_type: String,
    pub(crate) db_url: String,
    pub(crate) log_level: String,
    pub(crate) config_file: String,
}

impl Default for BubbleConfig {
    fn default() -> Self {
        Self {
            port: 3000,
            host: "127.0.0.1".to_string(),
            workers: 0, // 0 means use Tokio default
            db_type: "".to_string(),
            db_url: "".to_string(),
            log_level: "info".to_string(),
            config_file: "config.toml".to_string(),
        }
    }
}

/// Parse configuration from attribute tokens
pub(crate) fn parse_bubble_config(attr: TokenStream) -> BubbleConfig {
    let mut config = BubbleConfig::default();
    let attr_str = attr.to_string();
    if attr_str.is_empty() {
        return config;
    }
    let parts: Vec<&str> = attr_str.split(',').map(|s| s.trim()).collect();
    for part in parts {
        if part.contains('=') {
            let mut kv = part.split('=');
            let key = kv.next().unwrap_or("").trim();
            let value = kv.next().unwrap_or("").trim().trim_matches('"');
            match key {
                "port" => {
                    if let Ok(port) = value.parse() {
                        config.port = port;
                    }
                }
                "host" => config.host = value.to_string(),
                "workers" => {
                    if let Ok(workers) = value.parse() {
                        config.workers = workers;
                    }
                }
                "db_type" => config.db_type = value.to_string(),
                "db_url" => config.db_url = value.to_string(),
                "log_level" => config.log_level = value.to_string(),
                "config_file" => config.config_file = value.to_string(),
                _ => {}
            }
        }
    }
    config
}

/// Helper function to initialize logging
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

/// Helper function to initialize database connection
async fn init_database(db_type: &str, db_url: &str) -> Result<(), String> {
    // This would be implemented based on your database setup
    // For now, just log the configuration
    log::info!(
        "Database connection configured: type={}, url={}",
        db_type,
        db_url
    );
    Ok(())
}

/// Helper function to load configuration file
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

/// Helper function to parse command line arguments
fn parse_command_line_args(args: &[String]) {
    if args.len() > 1 {
        log::info!("Command line arguments: {:?}", &args[1..]);
    }
}

use mysql::serde_json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DbError {
    #[error("Invalid configuration: {0}")]
    Config(String),

    #[error("Connection failed: {0}")]
    Connection(String),

    #[error("Connection pool error: {0}")]
    Pool(String),

    #[error("Query execution failed: {0}")]
    Query(String),

    #[error("Transaction error: {0}")]
    Transaction(String),

    #[error("Row not found")]
    RowNotFound,

    #[error("Invalid data type: {0}")]
    Type(String),

    #[error("MySQL error: {0}")]
    MySql(String),

    #[error("PostgreSQL error: {0}")]
    Postgres(String),

    #[error("SQLite error: {0}")]
    Sqlite(String),

    #[error("Redis error: {0}")]
    RedisError(String),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Operation timeout: {0}")]
    Timeout(String),

    #[error("Database error: {0}")]
    Other(String),

    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),

    #[error(transparent)]
    MySqlAsync(#[from] mysql_async::Error),

    #[error(transparent)]
    Rusqlite(#[from] rusqlite::Error),

    #[error(transparent)]
    Redis(#[from] redis::RedisError),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
}

pub type DbResult<T> = Result<T, DbError>;

impl DbError {
    pub fn is_connection_error(&self) -> bool {
        matches!(
            self,
            DbError::Connection(_) | DbError::Pool(_) | DbError::Timeout(_)
        )
    }

    pub fn is_constraint_violation(&self) -> bool {
        match self {
            DbError::MySql(err) => err.contains("constraint"),
            DbError::Postgres(err) => err.contains("constraint"),
            DbError::Sqlite(err) => err.contains("constraint"),
            DbError::RedisError(err) => err.contains("constraint"),
            DbError::Redis(err) => err.to_string().contains("constraint"),
            DbError::Sqlx(err) => match err {
                sqlx::Error::Database(db_err) => db_err.message().contains("constraint"),
                _ => false,
            },
            DbError::MySqlAsync(err) => err.to_string().contains("constraint"),
            DbError::Rusqlite(err) => err.to_string().contains("constraint"),
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DatabaseType {
    MySql,
    Postgres,
    Sqlite,
    Redis,
}

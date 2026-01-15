pub mod config;
pub mod mysql;
pub mod postgres;
pub mod redis;
pub mod sqlite;
pub mod types;

use async_trait::async_trait;
use serde::Serialize;
use std::fmt::Debug;

pub use config::{DatabaseConfig, DatabaseType};

pub type DbResult<T> = Result<T, String>;

#[async_trait]
pub trait DatabaseConnection: Send + Sync + Debug {
    async fn execute(&self, sql: &str) -> DbResult<u64>;
    async fn query(&self, sql: &str) -> DbResult<String>;
    async fn query_one(&self, sql: &str) -> DbResult<String>;
    async fn insert_batch(&self, table: &str, json_data: &str) -> DbResult<u64>;
}

pub fn to_sql_value<T: Serialize>(value: &T) -> DbResult<String> {
    let json = serde_json::to_value(value).map_err(|e| e.to_string())?;
    match json {
        serde_json::Value::String(s) => Ok(format!("'{}'", s.replace("'", "''"))),
        serde_json::Value::Number(n) => Ok(n.to_string()),
        serde_json::Value::Bool(b) => Ok(if b { "1".to_string() } else { "0".to_string() }),
        serde_json::Value::Null => Ok("NULL".to_string()),
        _ => Ok(format!("'{}'", json.to_string().replace("'", "''"))),
    }
}

#[derive(Debug)]
pub enum DbConnection {
    MySql(mysql::MySqlConnection),
    Postgres(postgres::PostgresConnection),
    Sqlite(sqlite::SqliteConnection),
    Redis(redis::RedisConnection),
}

#[async_trait]
impl DatabaseConnection for DbConnection {
    async fn execute(&self, sql: &str) -> DbResult<u64> {
        match self {
            DbConnection::MySql(conn) => conn.execute(sql).await,
            DbConnection::Postgres(conn) => conn.execute(sql).await,
            DbConnection::Sqlite(conn) => conn.execute(sql).await,
            DbConnection::Redis(conn) => conn.execute(sql).await,
        }
    }

    async fn query(&self, sql: &str) -> DbResult<String> {
        match self {
            DbConnection::MySql(conn) => conn.query(sql).await,
            DbConnection::Postgres(conn) => conn.query(sql).await,
            DbConnection::Sqlite(conn) => conn.query(sql).await,
            DbConnection::Redis(conn) => conn.query(sql).await,
        }
    }

    async fn query_one(&self, sql: &str) -> DbResult<String> {
        match self {
            DbConnection::MySql(conn) => conn.query_one(sql).await,
            DbConnection::Postgres(conn) => conn.query_one(sql).await,
            DbConnection::Sqlite(conn) => conn.query_one(sql).await,
            DbConnection::Redis(conn) => conn.query_one(sql).await,
        }
    }

    async fn insert_batch(&self, table: &str, json_data: &str) -> DbResult<u64> {
        match self {
            DbConnection::MySql(conn) => conn.insert_batch(table, json_data).await,
            DbConnection::Postgres(conn) => conn.insert_batch(table, json_data).await,
            DbConnection::Sqlite(conn) => conn.insert_batch(table, json_data).await,
            DbConnection::Redis(conn) => conn.insert_batch(table, json_data).await,
        }
    }
}

pub async fn connect(config: &DatabaseConfig) -> DbResult<DbConnection> {
    match config.database_type {
        DatabaseType::MySql => {
            let conn = mysql::MySqlConnection::connect(config).await?;
            Ok(DbConnection::MySql(conn))
        }
        DatabaseType::Postgres => {
            let conn = postgres::PostgresConnection::connect(config).await?;
            Ok(DbConnection::Postgres(conn))
        }
        DatabaseType::Sqlite => {
            let conn = sqlite::SqliteConnection::connect(config).await?;
            Ok(DbConnection::Sqlite(conn))
        }
        DatabaseType::Redis => {
            let conn = redis::RedisConnection::connect(config).await?;
            Ok(DbConnection::Redis(conn))
        }
    }
}

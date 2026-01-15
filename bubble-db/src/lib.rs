pub mod config;
pub mod mysql;
pub mod postgres;
pub mod redis;
pub mod sqlite;
pub mod types;

use async_trait::async_trait;
use serde::{Serialize, de::DeserializeOwned};
use std::fmt::Debug;

pub use config::{DatabaseConfig, DatabaseType, PoolConfig, SslConfig};
pub use mysql::{MySqlConnection, MySqlPool};
pub use postgres::{PostgresConnection, PostgresPool};
pub use redis::{RedisConnection, RedisPool};
pub use sqlite::{SqliteConnection, SqlitePool};

use crate::types::DbResult;

#[async_trait]
pub trait DatabaseConnection: Send + Sync + Debug {
    async fn execute(&self, sql: &str, params: &[&dyn ToSql]) -> DbResult<u64>;

    async fn query<T>(&self, sql: &str, params: &[&dyn ToSql]) -> DbResult<Vec<T>>
    where
        T: DeserializeOwned + Send + Sync;

    async fn query_one<T>(&self, sql: &str, params: &[&dyn ToSql]) -> DbResult<T>
    where
        T: DeserializeOwned + Send + Sync;

    async fn query_scalar<T>(&self, sql: &str, params: &[&dyn ToSql]) -> DbResult<T>
    where
        T: DeserializeOwned + Send + Sync;

    async fn begin_transaction(&self) -> DbResult<Box<dyn Transaction>>;

    async fn insert_batch<T: Serialize + Send + Sync>(
        &self,
        table: &str,
        records: &[T],
    ) -> DbResult<u64>;

    fn connection_info(&self) -> ConnectionInfo;
}

#[async_trait]
pub trait Transaction: Send + Sync {
    async fn commit(self: Box<Self>) -> DbResult<()>;

    async fn rollback(self: Box<Self>) -> DbResult<()>;

    async fn execute(&mut self, sql: &str, params: &[&dyn ToSql]) -> DbResult<u64>;

    async fn query<T>(&self, sql: &str, params: &[&dyn ToSql]) -> DbResult<Vec<T>>
    where
        T: DeserializeOwned + Send + Sync;

    async fn savepoint(&mut self, name: &str) -> DbResult<()>;
    async fn rollback_to_savepoint(&mut self, name: &str) -> DbResult<()>;
    async fn release_savepoint(&mut self, name: &str) -> DbResult<()>;
}

pub trait ToSql: Sync {
    fn to_sql(&self) -> String;
}

impl ToSql for i32 {
    fn to_sql(&self) -> String {
        self.to_string()
    }
}

impl ToSql for i64 {
    fn to_sql(&self) -> String {
        self.to_string()
    }
}

impl ToSql for f64 {
    fn to_sql(&self) -> String {
        self.to_string()
    }
}

impl ToSql for String {
    fn to_sql(&self) -> String {
        format!("'{}'", self.replace("'", "''"))
    }
}

impl ToSql for &str {
    fn to_sql(&self) -> String {
        format!("'{}'", self.replace("'", "''"))
    }
}

impl ToSql for bool {
    fn to_sql(&self) -> String {
        if *self { "TRUE" } else { "FALSE" }.to_string()
    }
}

#[derive(Debug, Clone)]
pub struct ConnectionInfo {
    pub db_type: DatabaseType,
    pub host: String,
    pub port: u16,
    pub database: String,
    pub connected_at: chrono::DateTime<chrono::Utc>,
}

pub struct DatabaseFactory;

impl DatabaseFactory {
    pub async fn create(
        config: &crate::config::DatabaseConfig,
    ) -> DbResult<Box<dyn DatabaseConnection>> {
        match config.database_type {
            crate::config::DatabaseType::MySql => {
                use crate::mysql::MySqlConnection;
                let conn = MySqlConnection::connect(config).await?;
                Ok(Box::new(conn))
            }
            crate::config::DatabaseType::Postgres => {
                use crate::postgres::PostgresConnection;
                let conn = PostgresConnection::connect(config).await?;
                Ok(Box::new(conn))
            }
            crate::config::DatabaseType::Sqlite => {
                use crate::sqlite::SqliteConnection;
                let conn = SqliteConnection::connect(config).await?;
                Ok(Box::new(conn))
            }
            crate::config::DatabaseType::Redis => {
                use crate::redis::RedisConnection;
                let conn = RedisConnection::connect(config).await?;
                Ok(Box::new(conn))
            }
        }
    }

    pub async fn create_pool(
        config: &crate::config::DatabaseConfig,
    ) -> DbResult<Box<dyn ConnectionPool>> {
        match config.database_type {
            crate::config::DatabaseType::MySql => {
                use crate::mysql::MySqlPool;
                let pool = MySqlPool::new(config).await?;
                Ok(Box::new(pool))
            }
            crate::config::DatabaseType::Postgres => {
                use crate::postgres::PostgresPool;
                let pool = PostgresPool::new(config).await?;
                Ok(Box::new(pool))
            }
            crate::config::DatabaseType::Sqlite => {
                use crate::sqlite::SqlitePool;
                let pool = SqlitePool::new(config).await?;
                Ok(Box::new(pool))
            }
            crate::config::DatabaseType::Redis => {
                use crate::redis::RedisPool;
                let pool = RedisPool::new(config).await?;
                Ok(Box::new(pool))
            }
        }
    }
}

#[async_trait]
pub trait ConnectionPool: Send + Sync {
    async fn get(&self) -> DbResult<Box<dyn DatabaseConnection>>;
    fn status(&self) -> PoolStatus;
}

#[derive(Debug, Clone)]
pub struct PoolStatus {
    pub max_size: u32,
    pub size: u32,
    pub available: u32,
    pub waiting: u32,
}

pub async fn create_connection(config: &DatabaseConfig) -> DbResult<Box<dyn DatabaseConnection>> {
    DatabaseFactory::create(config).await
}

pub async fn create_pool(config: &DatabaseConfig) -> DbResult<Box<dyn ConnectionPool>> {
    DatabaseFactory::create_pool(config).await
}

pub struct DbHealth {
    pub connections: Vec<ConnectionInfo>,
    pub pools: Vec<PoolStatus>,
}

pub async fn health_check(pool: &dyn ConnectionPool) -> DbResult<bool> {
    let conn = pool.get().await?;
    let info = conn.connection_info();
    Ok(!info.host.is_empty())
}

pub fn db_test() {
    println!("123")
}

use crate::{DatabaseConfig, DatabaseConnection, DbResult};
use async_trait::async_trait;
use redis::{Client, Commands};
use std::collections::HashMap;

#[derive(Debug)]
pub struct RedisConnection {
    client: Client,
}

impl RedisConnection {
    pub async fn connect(config: &DatabaseConfig) -> DbResult<Self> {
        let client = Client::open(config.connection_string()).map_err(|e| e.to_string())?;
        Ok(Self { client })
    }

    fn get_connection(&self) -> DbResult<redis::Connection> {
        self.client.get_connection().map_err(|e| e.to_string())
    }
}

#[async_trait]
impl DatabaseConnection for RedisConnection {
    async fn execute(&self, sql: &str) -> DbResult<u64> {
        let mut conn = self.get_connection()?;
        let parts: Vec<&str> = sql.split_whitespace().collect();
        if parts.is_empty() {
            return Ok(0);
        }
        match parts[0].to_uppercase().as_str() {
            "SET" if parts.len() >= 3 => {
                let key = parts[1];
                let value = parts[2..].join(" ");
                let _: () = redis::cmd("SET")
                    .arg(key)
                    .arg(value)
                    .query(&mut conn)
                    .map_err(|e| e.to_string())?;
                Ok(1)
            }
            "DEL" if parts.len() >= 2 => {
                let keys = &parts[1..];
                let count: u64 = redis::cmd("DEL")
                    .arg(keys)
                    .query(&mut conn)
                    .map_err(|e| e.to_string())?;
                Ok(count)
            }
            "HSET" if parts.len() >= 4 => {
                let key = parts[1];
                let field = parts[2];
                let value = parts[3..].join(" ");
                let _: () = redis::cmd("HSET")
                    .arg(key)
                    .arg(field)
                    .arg(value)
                    .query(&mut conn)
                    .map_err(|e| e.to_string())?;
                Ok(1)
            }
            _ => Err("Unsupported Redis command".to_string()),
        }
    }

    async fn query(&self, sql: &str) -> DbResult<String> {
        let mut conn = self.get_connection()?;
        let parts: Vec<&str> = sql.split_whitespace().collect();
        match parts[0].to_uppercase().as_str() {
            "GET" if parts.len() == 2 => {
                let value: Option<String> = conn.get(parts[1]).map_err(|e| e.to_string())?;

                let result = if let Some(val) = value {
                    serde_json::json!({ "value": val })
                } else {
                    serde_json::json!([])
                };

                serde_json::to_string(&result).map_err(|e| e.to_string())
            }
            "HGETALL" if parts.len() == 2 => {
                let map: HashMap<String, String> =
                    conn.hgetall(parts[1]).map_err(|e| e.to_string())?;

                serde_json::to_string(&map).map_err(|e| e.to_string())
            }
            _ => Err("Unsupported Redis query".to_string()),
        }
    }

    async fn query_one(&self, sql: &str) -> DbResult<String> {
        let mut conn = self.get_connection()?;
        let parts: Vec<&str> = sql.split_whitespace().collect();
        match parts[0].to_uppercase().as_str() {
            "GET" if parts.len() == 2 => {
                let value: Option<String> = conn.get(parts[1]).map_err(|e| e.to_string())?;

                if let Some(val) = value {
                    serde_json::to_string(&serde_json::json!({ "value": val }))
                        .map_err(|e| e.to_string())
                } else {
                    Err("No data found".to_string())
                }
            }
            "HGETALL" if parts.len() == 2 => {
                let map: HashMap<String, String> =
                    conn.hgetall(parts[1]).map_err(|e| e.to_string())?;

                serde_json::to_string(&map).map_err(|e| e.to_string())
            }
            _ => Err("Unsupported Redis query".to_string()),
        }
    }

    async fn insert_batch(&self, table: &str, json_data: &str) -> DbResult<u64> {
        let items: Vec<serde_json::Value> = serde_json::from_str(json_data)
            .map_err(|e| format!("Failed to parse JSON data: {}", e))?;
        let mut conn = self.get_connection()?;
        let mut count = 0;
        for (i, item) in items.iter().enumerate() {
            let value = crate::to_sql_value(item)?;
            let key = format!("{}:{}", table, i);
            let _: () = conn.set(&key, value).map_err(|e| e.to_string())?;
            count += 1;
        }
        Ok(count)
    }
}

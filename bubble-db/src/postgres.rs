use crate::{DatabaseConfig, DatabaseConnection, DbResult};
use async_trait::async_trait;
use sqlx::{Column, Pool, Postgres, Row, postgres::PgPool};
use std::collections::HashMap;

#[derive(Debug)]
pub struct PostgresConnection {
    pool: Pool<Postgres>,
}

impl PostgresConnection {
    pub async fn connect(config: &DatabaseConfig) -> DbResult<Self> {
        let pool = PgPool::connect(&config.connection_string())
            .await
            .map_err(|e| e.to_string())?;

        Ok(Self { pool })
    }
}

#[async_trait]
impl DatabaseConnection for PostgresConnection {
    async fn execute(&self, sql: &str) -> DbResult<u64> {
        let result = sqlx::query(sql)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(result.rows_affected())
    }

    async fn query(&self, sql: &str) -> DbResult<String> {
        let rows = sqlx::query(sql)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())?;
        let mut results = Vec::new();
        for row in rows {
            let mut map = HashMap::new();
            let columns = row.columns();
            for (i, column) in columns.iter().enumerate() {
                let name = column.name().to_string();
                let value: String = row.try_get(i).unwrap_or_default();
                map.insert(name, value);
            }
            results.push(map);
        }
        serde_json::to_string(&results).map_err(|e| e.to_string())
    }

    async fn query_one(&self, sql: &str) -> DbResult<String> {
        let row = sqlx::query(sql)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| e.to_string())?;
        let mut map = HashMap::new();
        let columns = row.columns();
        for (i, column) in columns.iter().enumerate() {
            let name = column.name().to_string();
            let value: String = row.try_get(i).unwrap_or_default();
            map.insert(name, value);
        }
        serde_json::to_string(&map).map_err(|e| e.to_string())
    }

    async fn insert_batch(&self, table: &str, json_data: &str) -> DbResult<u64> {
        let items: Vec<serde_json::Value> = serde_json::from_str(json_data)
            .map_err(|e| format!("Failed to parse JSON data: {}", e))?;
        if items.is_empty() {
            return Ok(0);
        }
        let mut sql = String::new();
        sql.push_str(&format!("INSERT INTO {} VALUES ", table));
        for (i, item) in items.iter().enumerate() {
            if i > 0 {
                sql.push_str(", ");
            }
            let value = crate::to_sql_value(item)?;
            sql.push_str(&format!("({})", value));
        }
        self.execute(&sql).await
    }
}

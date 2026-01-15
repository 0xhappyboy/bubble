use crate::{DatabaseConfig, DatabaseConnection, DbResult};
use async_trait::async_trait;
use rusqlite::{Connection, Row};
use std::collections::HashMap;
use tokio::sync::Mutex;

#[derive(Debug)]
pub struct SqliteConnection {
    conn: Mutex<Connection>,
}

impl SqliteConnection {
    pub async fn connect(config: &DatabaseConfig) -> DbResult<Self> {
        let conn = Connection::open(&config.database).map_err(|e| e.to_string())?;
        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    fn row_to_map(row: &Row) -> DbResult<HashMap<String, String>> {
        let mut map = HashMap::new();
        for (i, column) in row.as_ref().column_names().iter().enumerate() {
            let name = column.to_string();
            let value: String = row.get(i).unwrap_or_default();
            map.insert(name, value);
        }
        Ok(map)
    }
}

#[async_trait]
impl DatabaseConnection for SqliteConnection {
    async fn execute(&self, sql: &str) -> DbResult<u64> {
        let mut conn = self.conn.lock().await;
        conn.execute(sql, [])
            .map(|n| n as u64)
            .map_err(|e| e.to_string())
    }

    async fn query(&self, sql: &str) -> DbResult<String> {
        let conn = self.conn.lock().await;
        let mut stmt = conn.prepare(sql).map_err(|e| e.to_string())?;
        let rows = stmt.query([]).map_err(|e| e.to_string())?;
        let mut results = Vec::new();
        let mut rows_iter = rows;
        while let Some(row) = rows_iter.next().map_err(|e| e.to_string())? {
            let map = Self::row_to_map(&row)?;
            results.push(map);
        }
        serde_json::to_string(&results).map_err(|e| e.to_string())
    }

    async fn query_one(&self, sql: &str) -> DbResult<String> {
        let conn = self.conn.lock().await;
        let mut stmt = conn.prepare(sql).map_err(|e| e.to_string())?;
        let mut rows = stmt.query([]).map_err(|e| e.to_string())?;

        if let Some(row) = rows.next().map_err(|e| e.to_string())? {
            let map = Self::row_to_map(&row)?;
            serde_json::to_string(&map).map_err(|e| e.to_string())
        } else {
            Err("No rows found".to_string())
        }
    }

    async fn insert_batch(&self, table: &str, json_data: &str) -> DbResult<u64> {
        let items: Vec<serde_json::Value> = serde_json::from_str(json_data)
            .map_err(|e| format!("Failed to parse JSON data: {}", e))?;

        if items.is_empty() {
            return Ok(0);
        }
        let mut conn = self.conn.lock().await;
        let tx = conn.transaction().map_err(|e| e.to_string())?;
        for item in items.iter() {
            let value = crate::to_sql_value(item)?;
            let sql = format!("INSERT INTO {} VALUES ({})", table, value);
            tx.execute(&sql, []).map_err(|e| e.to_string())?;
        }
        tx.commit().map_err(|e| e.to_string())?;
        Ok(items.len() as u64)
    }
}

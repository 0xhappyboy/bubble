use crate::{DatabaseConfig, DatabaseConnection, DbResult};
use async_trait::async_trait;
use mysql_async::{Conn, prelude::Queryable};
use std::collections::HashMap;
use tokio::sync::Mutex;

#[derive(Debug)]
pub struct MySqlConnection {
    conn: Mutex<Conn>,
}

impl MySqlConnection {
    pub async fn connect(config: &DatabaseConfig) -> DbResult<Self> {
        let conn = Conn::new(
            mysql_async::Opts::from_url(&config.connection_string()).map_err(|e| e.to_string())?,
        )
        .await
        .map_err(|e| e.to_string())?;

        Ok(Self {
            conn: Mutex::new(conn),
        })
    }
}

#[async_trait]
impl DatabaseConnection for MySqlConnection {
    async fn execute(&self, sql: &str) -> DbResult<u64> {
        let mut conn = self.conn.lock().await;
        conn.query_drop(sql).await.map_err(|e| e.to_string())?;
        let result = conn
            .query_iter("SELECT ROW_COUNT()")
            .await
            .map_err(|e| e.to_string())?;
        let rows = result
            .map_and_drop(|row| row)
            .await
            .map_err(|e| e.to_string())?;

        if let Some(row) = rows.first() {
            let affected: i64 = row.get(0).unwrap_or(0);
            Ok(affected.max(0) as u64)
        } else {
            Ok(0)
        }
    }

    async fn query(&self, sql: &str) -> DbResult<String> {
        let mut conn = self.conn.lock().await;
        let result = conn.query_iter(sql).await.map_err(|e| e.to_string())?;
        let rows = result
            .map_and_drop(|row| row)
            .await
            .map_err(|e| e.to_string())?;
        let mut results = Vec::new();
        for row in rows {
            let mut map = HashMap::new();
            for (i, column) in row.columns_ref().iter().enumerate() {
                let name = column.name_str().to_string();
                let opt_value: Option<mysql_async::Value> = row.get(i);
                let value = match opt_value {
                    Some(mysql_async::Value::Int(i)) => i.to_string(),
                    Some(mysql_async::Value::UInt(u)) => u.to_string(),
                    Some(mysql_async::Value::Float(f)) => f.to_string(),
                    Some(mysql_async::Value::Double(d)) => d.to_string(),
                    Some(mysql_async::Value::Bytes(bytes)) => {
                        String::from_utf8_lossy(&bytes).to_string()
                    }
                    Some(mysql_async::Value::Date(
                        year,
                        month,
                        day,
                        hour,
                        minute,
                        second,
                        micro,
                    )) => {
                        format!(
                            "{}-{:02}-{:02} {:02}:{:02}:{:02}.{:06}",
                            year as i32, month, day, hour, minute, second, micro
                        )
                    }
                    Some(mysql_async::Value::Time(neg, days, hours, minutes, seconds, micros)) => {
                        let total = (days as i64 * 86400
                            + hours as i64 * 3600
                            + minutes as i64 * 60
                            + seconds as i64) as i64;
                        let total = if neg { -total } else { total };
                        format!(
                            "{} days {}:{:02}:{:02}.{:06}",
                            days, hours, minutes, seconds, micros
                        )
                    }
                    None | Some(mysql_async::Value::NULL) => "".to_string(),
                };
                map.insert(name, value);
            }
            results.push(map);
        }
        serde_json::to_string(&results).map_err(|e| e.to_string())
    }

    async fn query_one(&self, sql: &str) -> DbResult<String> {
        let mut conn = self.conn.lock().await;
        let result = conn.query_iter(sql).await.map_err(|e| e.to_string())?;
        let rows = result
            .map_and_drop(|row| row)
            .await
            .map_err(|e| e.to_string())?;
        if let Some(row) = rows.first() {
            let mut map = HashMap::new();
            for (i, column) in row.columns_ref().iter().enumerate() {
                let name = column.name_str().to_string();
                let opt_value: Option<mysql_async::Value> = row.get(i);
                let value = match opt_value {
                    Some(mysql_async::Value::Int(i)) => i.to_string(),
                    Some(mysql_async::Value::UInt(u)) => u.to_string(),
                    Some(mysql_async::Value::Float(f)) => f.to_string(),
                    Some(mysql_async::Value::Double(d)) => d.to_string(),
                    Some(mysql_async::Value::Bytes(bytes)) => {
                        String::from_utf8_lossy(&bytes).to_string()
                    }
                    Some(mysql_async::Value::Date(
                        year,
                        month,
                        day,
                        hour,
                        minute,
                        second,
                        micro,
                    )) => {
                        format!(
                            "{}-{:02}-{:02} {:02}:{:02}:{:02}.{:06}",
                            year as i32, month, day, hour, minute, second, micro
                        )
                    }
                    Some(mysql_async::Value::Time(neg, days, hours, minutes, seconds, micros)) => {
                        let total = (days as i64 * 86400
                            + hours as i64 * 3600
                            + minutes as i64 * 60
                            + seconds as i64) as i64;
                        let total = if neg { -total } else { total };
                        format!(
                            "{} days {}:{:02}:{:02}.{:06}",
                            days, hours, minutes, seconds, micros
                        )
                    }
                    None | Some(mysql_async::Value::NULL) => "".to_string(),
                };
                map.insert(name, value);
            }
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
        let mut count = 0;
        conn.query_drop("START TRANSACTION")
            .await
            .map_err(|e| e.to_string())?;
        for item in items {
            let value = crate::to_sql_value(&item)?;
            let sql = format!("INSERT INTO {} VALUES ({})", table, value);
            conn.query_drop(&sql).await.map_err(|e| e.to_string())?;
            count += 1;
        }
        conn.query_drop("COMMIT").await.map_err(|e| e.to_string())?;
        Ok(count)
    }
}

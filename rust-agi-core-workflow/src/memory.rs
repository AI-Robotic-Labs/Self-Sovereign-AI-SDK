use anyhow::Result;
use rusqlite::{params, Connection};
use serde_json::Value;
use chrono::Utc;

pub struct Memory {
    conn: Connection,
}

impl Memory {
    pub fn new(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS memories (
                id INTEGER PRIMARY KEY,
                tstamp TEXT NOT NULL,
                kind TEXT NOT NULL,
                content TEXT NOT NULL
            );",
        )?;
        Ok(Self { conn })
    }

    pub fn store(&self, kind: &str, content: &Value) -> Result<()> {
        let t = Utc::now().to_rfc3339();
        self.conn.execute(
            "INSERT INTO memories (tstamp, kind, content) VALUES (?1, ?2, ?3)",
            params![t, kind, content.to_string()],
        )?;
        Ok(())
    }

    pub fn recent(&self, n: usize) -> Result<Vec<Value>> {
        let mut stmt = self.conn.prepare(
            "SELECT content FROM memories ORDER BY id DESC LIMIT ?1"
        )?;
        let rows = stmt.query_map([n as i64], |r| r.get::<_, String>(0))?;
        let mut out = vec![];
        for row in rows {
            let s: String = row?;
            if let Ok(v) = serde_json::from_str::<Value>(&s) {
                out.push(v);
            }
        }
        Ok(out)
    }
}

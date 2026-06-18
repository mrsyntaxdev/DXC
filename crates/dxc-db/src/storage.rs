use dxc_core::DxcError;
use rusqlite::{params, Connection};
use crate::HistoryEntry;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn open(path: &str) -> Result<Self, DxcError> {
        if let Some(parent) = std::path::Path::new(path).parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| DxcError::Database(e.to_string()))?;
        }
        let conn = Connection::open(path)
            .map_err(|e| DxcError::Database(e.to_string()))?;
        let db = Self { conn };
        db.initialize()?;
        Ok(db)
    }

    fn initialize(&self) -> Result<(), DxcError> {
        self.conn
            .execute_batch(
                "CREATE TABLE IF NOT EXISTS history (
                    id          INTEGER PRIMARY KEY AUTOINCREMENT,
                    url         TEXT NOT NULL,
                    title       TEXT,
                    media_type  TEXT NOT NULL,
                    provider    TEXT NOT NULL,
                    file_path   TEXT,
                    created_at  TEXT NOT NULL DEFAULT (datetime('now')),
                    success     INTEGER NOT NULL DEFAULT 0
                );

                CREATE TABLE IF NOT EXISTS config (
                    key   TEXT PRIMARY KEY,
                    value TEXT NOT NULL
                );",
            )
            .map_err(|e| DxcError::Database(e.to_string()))?;
        Ok(())
    }

    pub fn insert_history(
        &self,
        url: &str,
        title: Option<&str>,
        media_type: &str,
        provider: &str,
        file_path: Option<&str>,
        success: bool,
    ) -> Result<(), DxcError> {
        self.conn
            .execute(
                "INSERT INTO history (url, title, media_type, provider, file_path, success)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                params![url, title, media_type, provider, file_path, success as i32],
            )
            .map_err(|e| DxcError::Database(e.to_string()))?;
        Ok(())
    }

    pub fn get_history(&self, limit: usize) -> Result<Vec<HistoryEntry>, DxcError> {
        let mut stmt = self.conn
            .prepare(
                "SELECT id, url, title, media_type, provider, file_path, created_at, success
                 FROM history ORDER BY id DESC LIMIT ?1",
            )
            .map_err(|e| DxcError::Database(e.to_string()))?;

        let rows = stmt
            .query_map(params![limit as i64], |row| {
                Ok(HistoryEntry {
                    id: row.get(0)?,
                    url: row.get(1)?,
                    title: row.get(2)?,
                    media_type: row.get(3)?,
                    provider: row.get(4)?,
                    file_path: row.get(5)?,
                    created_at: row.get(6)?,
                    success: row.get::<_, i32>(7)? != 0,
                })
            })
            .map_err(|e| DxcError::Database(e.to_string()))?;

        let mut entries = Vec::new();
        for row in rows {
            entries.push(row.map_err(|e| DxcError::Database(e.to_string()))?);
        }
        Ok(entries)
    }
}

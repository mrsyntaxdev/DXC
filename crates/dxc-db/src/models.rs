use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub id: i64,
    pub url: String,
    pub title: Option<String>,
    pub media_type: String,
    pub provider: String,
    pub file_path: Option<String>,
    pub created_at: String,
    pub success: bool,
}

use std::fmt;
use std::time::SystemTime;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateLog{
    pub protocol_id: String,
    pub created_at: SystemTime,
    pub last_update_at: SystemTime,
    pub start_block: u64,
    pub end_block: u64
}

// Implementacja Default
impl Default for UpdateLog {
    fn default() -> Self {
        UpdateLog {
            protocol_id: String::new(),
            created_at: SystemTime::now(),
            last_update_at: SystemTime::now(),
            start_block: 0,
            end_block: 0,
        }
    }
}

impl fmt::Display for UpdateLog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let created_at = DateTime::<Utc>::from(self.created_at);
        let last_update_at = DateTime::<Utc>::from(self.last_update_at);

        write!(
            f,
            "UpdateLog {{ protocol_id: {}, created_at: {}, last_update_at: {}, start_block: {}, end_block: {} }}",
            self.protocol_id,
            created_at.format("%Y-%m-%d %H:%M:%S"),
            last_update_at.format("%Y-%m-%d %H:%M:%S"),
            self.start_block,
            self.end_block,
        )
    }
}
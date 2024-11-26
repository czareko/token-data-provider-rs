use serde::{Deserialize, Serialize};
use crate::domain::entities::types::date_time::DateTime;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdateLog{
    pub protocol_id: String,
    pub created_at: DateTime,
    pub last_update_at: DateTime,
    pub start_block: i64,
    pub end_block: i64
}
use serde::{Deserialize, Serialize};
use crate::domain::entities::types::date_time::DateTime;

#[derive(Debug, Serialize, Deserialize,Clone, Default)]
pub struct Token{
    pub address: String,
    pub protocol_id: String,
    pub symbol: String,
    pub name: String,
    pub decimals: String,
    pub retrieved_at: DateTime,
    pub updated_at: DateTime,
    pub active: bool
}
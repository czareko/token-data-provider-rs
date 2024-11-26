use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Protocol{
    pub id: String,
    pub chain_id: String,
    pub dex_id: String
}
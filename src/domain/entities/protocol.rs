use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Protocol{
    pub chain_id: String,
    pub protocol_id: String
}
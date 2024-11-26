use std::time::SystemTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct Token{
    pub address: String,
    pub protocol_id: String,
    pub symbol: String,
    pub name: String,
    pub decimals: String,
    pub retrieved_at: SystemTime,
    pub updated_at: SystemTime,
    pub active: bool
}

impl Default for Token {
    fn default() -> Self {
        Token {
            address: String::new(),
            protocol_id: String::new(),
            symbol: String::new(),
            name: String::new(),
            decimals: String::new(),
            retrieved_at: SystemTime::now(),
            updated_at: SystemTime::now(),
            active: true,
        }
    }
}
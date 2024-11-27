use std::collections::HashMap;
use std::time::SystemTime;
use serde::{Deserialize, Serialize};
use crate::domain::entities::token_pair::TokenPair;

#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct Token{
    pub address: String,
    pub protocol_id: String,
    pub symbol: String,
    pub name: String,
    pub decimals: String,
    pub retrieved_at: SystemTime,
    pub updated_at: SystemTime,
    pub pairs: HashMap<String, TokenPair>,
    pub active_pairs: HashMap<String, TokenPair>,
    pub swaps: i64,
    pub high_risk: bool
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
            pairs: HashMap::new(),
            active_pairs: HashMap::new(),
            swaps: 0,
            high_risk: false,
        }
    }
}
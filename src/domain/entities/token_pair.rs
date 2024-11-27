use std::fmt;
use std::time::SystemTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct TokenPair{
    pub token_pair_address: String,
    pub protocol_id: String,
    pub base_address: String,
    pub base_reserve: u128,
    pub quote_address: String,
    pub quote_reserve: u128,
    pub reserve_block: u32,
    pub swaps: i64,
    pub retrieved_at: SystemTime,
    pub updated_at: SystemTime
}

impl Default for TokenPair {
    fn default() -> Self {
        TokenPair {
            token_pair_address: String::new(),
            protocol_id: String::new(),
            base_address: String::new(),
            base_reserve: 0,
            quote_address: String::new(),
            quote_reserve: 0,
            reserve_block: 0,
            swaps: 0,
            retrieved_at: SystemTime::now(),
            updated_at: SystemTime::now(),
        }
    }
}

impl fmt::Display for TokenPair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "TokenPair {{
            token_pair_address: {},
            protocol_id: {},
            base_address: {},
            base_reserve: {},
            quote_address: {},
            quote_reserve: {},
            reserve_block: {},
            swaps: {},
            retrieved_at: {:?},
            updated_at: {:?}
        }}",
            self.token_pair_address,
            self.protocol_id,
            self.base_address,
            self.base_reserve,
            self.quote_address,
            self.quote_reserve,
            self.reserve_block,
            self.swaps,
            self.retrieved_at,
            self.updated_at
        )
    }
}
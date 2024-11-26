use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize,Clone, Default)]
pub struct TokenPair{
    pub token_pair_id: String,
    pub protocol_id: String,
    pub base_address: String,
    pub quote_address: String,
    //TODO will be way more here
}
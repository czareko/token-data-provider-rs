use std::fmt;
use ethers::types::{Address, Bytes, H256, U256, U64};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwapLog {
    pub address: Address,
    pub topics: Vec<H256>,
    pub data: Bytes,
    pub block_hash: Option<H256>,
    pub block_number: Option<U64>,
    pub transaction_hash: Option<H256>,
    pub transaction_index: Option<U64>,
    pub log_index: Option<U256>,
    pub removed: Option<bool>,
}

impl fmt::Display for SwapLog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "SwapLog {{ address: {}, block_number: {:?}, transaction_hash: {:?}, log_index: {:?} }}",
            self.address,
            self.block_number,
            self.transaction_hash,
            self.log_index
        )
    }
}
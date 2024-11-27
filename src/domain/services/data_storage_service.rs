use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use once_cell::sync::Lazy;
use crate::domain::entities::protocol::Protocol;
use crate::domain::entities::token::Token;
use crate::domain::entities::token_pair::TokenPair;
use crate::domain::entities::update_log::UpdateLog;
use log;
use crate::domain::entities::swap_log::SwapLog;

pub static DATA_STORAGE: Lazy<Arc<Mutex<DataStorage>>> = Lazy::new(|| {
    Arc::new(Mutex::new(DataStorage::default()))
});

pub struct DataStorage {
    pub tokens: HashMap<String, Token>,
    pub protocols: HashMap<String, Protocol>,
    pub token_pairs: HashMap<String, TokenPair>,
    pub update_logs: HashMap<String, UpdateLog>,
    pub swap_logs: HashMap<String, Vec<SwapLog>>,
}

impl Default for DataStorage {
    fn default() -> Self {
        DataStorage {
            tokens: HashMap::new(),
            protocols: HashMap::new(),
            token_pairs: HashMap::new(),
            update_logs: HashMap::new(),
            swap_logs: HashMap::new(),
        }
    }
}

pub struct DataStorageService;

#[allow(dead_code)]
pub trait DataStorageTrait {
    fn get_instance() -> Arc<Mutex<DataStorage>>;
    fn init(&self);

    fn add_token(&self, key: String, token: Token);
    fn get_token(&self, key: String) -> Option<Token>;
    fn get_tokens(&self) -> HashMap<String, Token>;
    fn get_tokens_size(&self) -> i64;

    fn add_protocol(&self, key: String, protocol: Protocol);
    fn get_protocol(&self, key: String) -> Option<Protocol>;
    fn get_protocols(&self) -> HashMap<String, Protocol>;
    fn get_protocols_size(&self) -> i64;

    fn add_token_pair(&self, key: String, token_pair: TokenPair);
    fn get_token_pair(&self, key: String)-> Option<TokenPair>;
    fn get_token_pairs(&self) -> HashMap<String, TokenPair>;
    fn get_token_pairs_size(&self) -> i64;
    fn get_token_pairs_by_address(&self, address: String) -> HashMap<String, TokenPair>;
    fn get_non_zero_reserve_token_pairs_by_address(&self, address: String) -> HashMap<String, TokenPair>;
    fn get_total_swaps_for_address(&self, address: String) -> i64;

    fn add_update_log(&self, key: String, update_log: UpdateLog);
    fn get_update_log(&self, key: String) -> Option<UpdateLog>;
    fn get_update_logs(&self) -> HashMap<String, UpdateLog>;
    fn get_update_logs_size(&self) -> i64;

    fn add_swap_log(&self, key: String, swap_log: SwapLog);
    fn get_swap_logs(&self, key: String) -> Option<Vec<SwapLog>>;
    fn get_all_swap_logs(&self) -> HashMap<String, Vec<SwapLog>>;
    fn get_swap_logs_size(&self, key: String) -> i64;
    fn get_total_swap_logs_size(&self) -> i64;
}

impl DataStorageTrait for DataStorageService {
    fn get_instance() -> Arc<Mutex<DataStorage>> {
        Arc::clone(&DATA_STORAGE)
    }

    fn init(&self) {
        let protocol = Protocol {
            id: "BASE_UNISWAP_V2".to_string(),
            chain_id: "BASE".to_string(),
            dex_id: "UNISWAP_V2".to_string(),
        };
        Self.add_protocol(protocol.id.clone(), protocol);

        log::info!("Data Storage initialized");
    }

    // TOKEN
    fn add_token(&self, key: String, token: Token) {
        let mut storage = DATA_STORAGE.lock().unwrap();
        storage.tokens.insert(key, token);
    }

    fn get_token(&self, key: String) -> Option<Token> {
        let storage = DATA_STORAGE.lock().unwrap();
        storage.tokens.get(&key).cloned()
    }

    fn get_tokens(&self) -> HashMap<String, Token> {
        let storage = DATA_STORAGE.lock().unwrap();
        storage.tokens.clone()
    }

    fn get_tokens_size(&self) -> i64 {
        let storage = DATA_STORAGE.lock().unwrap();
        storage.tokens.len() as i64
    }

    // PROTOCOL
    fn add_protocol(&self, key: String, protocol: Protocol) {
        let mut storage = DATA_STORAGE.lock().unwrap();
        storage.protocols.insert(key, protocol);
    }

    fn get_protocol(&self, key: String) -> Option<Protocol> {
        let storage = DATA_STORAGE.lock().unwrap();
        storage.protocols.get(&key).cloned()
    }


    fn get_protocols(&self) -> HashMap<String, Protocol> {
        let storage = DATA_STORAGE.lock().unwrap();
        storage.protocols.clone()
    }

    fn get_protocols_size(&self) -> i64 {
        let storage = DATA_STORAGE.lock().unwrap();
        storage.protocols.len() as i64
    }

    // TOKEN PAIRS
    fn add_token_pair(&self, key: String, token_pair: TokenPair) {
        let mut storage = DATA_STORAGE.lock().unwrap();
        storage.token_pairs.insert(key, token_pair);
    }

    fn get_token_pair(&self, key: String) -> Option<TokenPair> {
        let storage = DATA_STORAGE.lock().unwrap();
        storage.token_pairs.get(&key).cloned()
    }

    fn get_token_pairs(&self) -> HashMap<String, TokenPair> {
        let storage = DATA_STORAGE.lock().unwrap();
        storage.token_pairs.clone()
    }

    fn get_token_pairs_size(&self) -> i64 {
        let storage = DATA_STORAGE.lock().unwrap();
        storage.token_pairs.len() as i64
    }

    fn get_token_pairs_by_address(&self, address: String) -> HashMap<String, TokenPair> {
        let storage = DATA_STORAGE.lock().unwrap();
        storage
            .token_pairs
            .iter()
            .filter(|(_, pair)| pair.base_address == address || pair.quote_address == address)
            .map(|(key, pair)| (key.clone(), pair.clone()))
            .collect()
    }

    fn get_non_zero_reserve_token_pairs_by_address(&self, address: String) -> HashMap<String, TokenPair> {
        let storage = DATA_STORAGE.lock().unwrap();
        storage
            .token_pairs
            .iter()
            .filter(|(_, pair)| {
                (pair.base_address == address || pair.quote_address == address)
                    && pair.base_reserve > 0
                    && pair.quote_reserve > 0
            })
            .map(|(key, pair)| (key.clone(), pair.clone()))
            .collect()
    }

    fn get_total_swaps_for_address(&self, address: String) -> i64 {
        let storage = DATA_STORAGE.lock().unwrap();
        storage
            .token_pairs
            .values()
            .filter(|pair| pair.base_address == address || pair.quote_address == address)
            .map(|pair| pair.swaps)
            .sum()
    }


    // UPDATE LOGS
    fn add_update_log(&self, key: String, update_log: UpdateLog) {
        let mut storage = DATA_STORAGE.lock().unwrap();
        storage.update_logs.insert(key, update_log);
    }

    fn get_update_log(&self, key: String) -> Option<UpdateLog> {
        let storage = DATA_STORAGE.lock().unwrap();
        storage.update_logs.get(&key).cloned()
    }

    fn get_update_logs(&self) -> HashMap<String, UpdateLog> {
        let storage = DATA_STORAGE.lock().unwrap();
        storage.update_logs.clone()
    }

    fn get_update_logs_size(&self) -> i64 {
        let storage = DATA_STORAGE.lock().unwrap();
        storage.update_logs.len() as i64
    }

    //SWAP LOGS

    fn add_swap_log(&self, key: String, swap_log: SwapLog) {
        let mut storage = DATA_STORAGE.lock().unwrap();
        storage
            .swap_logs
            .entry(key)
            .or_insert_with(Vec::new)
            .push(swap_log);
    }

    fn get_swap_logs(&self, key: String) -> Option<Vec<SwapLog>> {
        let storage = DATA_STORAGE.lock().unwrap();
        storage.swap_logs.get(&key).cloned()
    }

    fn get_all_swap_logs(&self) -> HashMap<String, Vec<SwapLog>> {
        let storage = DATA_STORAGE.lock().unwrap();
        storage.swap_logs.clone()
    }

    fn get_swap_logs_size(&self, key: String) -> i64 {
        let storage = DATA_STORAGE.lock().unwrap();
        storage
            .swap_logs
            .get(&key)
            .map_or(0, |logs| logs.len() as i64)
    }

    fn get_total_swap_logs_size(&self) -> i64 {
        let storage = DATA_STORAGE.lock().unwrap();
        storage.swap_logs.values().map(|logs| logs.len() as i64).sum()
    }
}
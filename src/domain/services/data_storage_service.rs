use std::sync::{Arc, Mutex};
use once_cell::sync::Lazy;
use crate::domain::entities::protocol::Protocol;
use crate::domain::entities::token::Token;
use crate::domain::entities::token_pair::TokenPair;
use crate::domain::entities::update_log::UpdateLog;
use log;

pub static DATA_STORAGE: Lazy<Arc<Mutex<DataStorage>>> = Lazy::new(|| {
    Arc::new(Mutex::new(DataStorage::default()))
});

pub struct DataStorage {
    pub tokens: Option<Vec<Token>>,
    pub protocols: Option<Vec<Protocol>>,
    pub token_pairs: Option<Vec<TokenPair>>,
    pub update_logs: Option<Vec<UpdateLog>>
}

impl Default for DataStorage {
    fn default() -> Self {
        DataStorage {
            tokens: Some(Vec::new()),
            protocols: Some(Vec::new()),
            token_pairs: Some(Vec::new()),
            update_logs: Some(Vec::new()),
        }
    }
}

pub struct DataStorageService;

#[allow(dead_code)]
pub trait DataStorageTrait {
    fn get_instance() -> Arc<Mutex<DataStorage>>;
    fn init(&self);

    fn add_token(&self, token: Token);
    fn get_tokens(&self) -> Option<Vec<Token>>;
    fn get_tokens_size(&self) -> i64;

    fn add_protocol(&self, protocol: Protocol);
    fn get_protocols(&self) -> Option<Vec<Protocol>>;
    fn get_protocols_size(&self) -> i64;

    fn add_token_pair(&self, token_pair: TokenPair);
    fn get_token_pairs(&self) -> Option<Vec<TokenPair>>;
    fn get_token_pairs_size(&self) -> i64;

    fn add_update_log(&self, update_log: UpdateLog);
    fn get_update_logs(&self) -> Option<Vec<UpdateLog>>;
    fn get_update_logs_size(&self) -> i64;
}

impl DataStorageTrait for DataStorageService{
    // Pobierz instancję DataStorage
    fn get_instance() -> Arc<Mutex<DataStorage>> {
        Arc::clone(&DATA_STORAGE)
    }

    fn init(&self) {

        //PROTOCOLS

        let protocol = Protocol {
          id: "BASE_UNISWAP_V2".to_string(),
          chain_id: "BASE".to_string(),
          dex_id: "UNISWAP_V2".to_string()
        };
        Self.add_protocol(protocol);

        log::info!("Data Storage initialized")
    }


    //TOKEN

    fn add_token(&self, token: Token) {
        let mut storage = DATA_STORAGE.lock().unwrap();
        if let Some(tokens) = &mut storage.tokens {
            tokens.push(token);
        }
    }

    fn get_tokens(&self) -> Option<Vec<Token>> {
        let storage = DATA_STORAGE.lock().unwrap();
        storage.tokens.clone()
    }

    fn get_tokens_size(&self) -> i64 {
        let storage = DATA_STORAGE.lock().unwrap();
        if let Some(tokens) = &storage.tokens {
            tokens.len() as i64
        } else {
            0
        }
    }

    // PROTOCOL

    fn add_protocol(&self, protocol: Protocol) {
        let mut storage = DATA_STORAGE.lock().unwrap();
        if let Some(protocols) = &mut storage.protocols {
            protocols.push(protocol);
        }
    }

    fn get_protocols(&self) -> Option<Vec<Protocol>> {
        let storage = DATA_STORAGE.lock().unwrap();
        storage.protocols.clone()
    }

    fn get_protocols_size(&self) -> i64 {
        let storage = DATA_STORAGE.lock().unwrap();
        if let Some(protocols) = &storage.protocols {
            protocols.len() as i64
        } else {
            0
        }
    }


    //TOKEN PAIRS

    fn add_token_pair(&self, token_pair: TokenPair) {
        let mut storage = DATA_STORAGE.lock().unwrap();
        if let Some(token_pairs) = &mut storage.token_pairs {
            token_pairs.push(token_pair);
        }
    }

    fn get_token_pairs(&self) -> Option<Vec<TokenPair>> {
        let storage = DATA_STORAGE.lock().unwrap();
        storage.token_pairs.clone()
    }

    fn get_token_pairs_size(&self) -> i64 {
        let storage = DATA_STORAGE.lock().unwrap();
        if let Some(token_pairs) = &storage.token_pairs {
            token_pairs.len() as i64
        } else {
            0
        }
    }

    //UPDATE_LOGS

    fn add_update_log(&self, update_log: UpdateLog) {
        let mut storage = DATA_STORAGE.lock().unwrap();
        if let Some(update_logs) = &mut storage.update_logs {
            update_logs.push(update_log);
        }
    }

    fn get_update_logs(&self) -> Option<Vec<UpdateLog>> {
        let storage = DATA_STORAGE.lock().unwrap();
        storage.update_logs.clone()
    }

    fn get_update_logs_size(&self) -> i64 {
        let storage = DATA_STORAGE.lock().unwrap();
        if let Some(update_logs) = &storage.update_logs {
            update_logs.len() as i64
        } else {
            0
        }
    }
}
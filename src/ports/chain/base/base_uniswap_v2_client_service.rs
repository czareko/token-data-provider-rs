use async_trait::async_trait;
use crate::config::CONFIG;
use log;

#[derive(Debug)]
pub struct BaseUniswapV2ClientService;

#[async_trait]
pub trait BaseUniswapV2ClientServiceTrait: Send + Sync{
    async fn start_chain_connection(&self);
}

#[async_trait]
impl BaseUniswapV2ClientServiceTrait for BaseUniswapV2ClientService {
    async fn start_chain_connection(&self) {
        log::info!("Start Base Network connection ...");
        log::info!("RPC Endpoint: {}",CONFIG.default.chain_base_rpc_url);
    }
}

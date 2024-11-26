mod domain;
mod config;
mod ports;

use std::env;
use std::sync::Arc;
use crate::config::CONFIG;
use crate::ports::chain::base::base_uniswap_v2_client_service::{BaseUniswapV2ClientService, BaseUniswapV2ClientServiceTrait};

#[tokio::main]
async fn main() {
    //Default info logs level
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }
    env_logger::init();
    log::info!("Starting application...");

    let base_uniswap_v2_client_service = Arc::new(BaseUniswapV2ClientService);
    base_uniswap_v2_client_service.start_chain_connection().await;

    log::info!("HTTP SERVER PORT::{}",CONFIG.default.server_port_http);

}

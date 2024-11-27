mod domain;
mod config;
mod ports;
mod adapters;

use std::env;
use std::sync::Arc;
use crate::config::CONFIG;
use crate::ports::chain::base::base_uniswap_v2_client_service::{BaseUniswapV2ClientService, BaseUniswapV2ClientServiceTrait};
use crate::adapters::token_api::create_token_rest_api;
use crate::ports::chain::base::base_uniswap_v2_swap_sync::{BaseUniswapV2SwapSynchronizer, BaseUniswapV2SwapSynchronizerTrait};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //Default info logs level
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }
    env_logger::init();
    log::info!("Starting application...");


    //base_uniswap_v2_client_service.init_chain_data_sync();

    //log::info!("HTTP SERVER PORT::{}",CONFIG.default.server_port_http);

    let base_uniswap_v2_swap_sync_handle = tokio::spawn(async{
        let base_uniswap_v2_swap_sync = Arc::new(BaseUniswapV2SwapSynchronizer);
        let _ = base_uniswap_v2_swap_sync.synchronize_swaps().await;
    });

    let base_uniswap_v2_handle = tokio::spawn(async {
        log::info!("Starting Base Newtork Uniswap V2 Sync service ...");
        let base_uniswap_v2_client_service = Arc::new(BaseUniswapV2ClientService);
        let _ = base_uniswap_v2_client_service.init_chain_data_sync().await;
    });

    let rest_api_handle = tokio::spawn(async {
        log::info!("Starting REST API server on port {} ...", CONFIG.default.server_port_http);
        let api = create_token_rest_api();
        warp::serve(api).run(([0, 0, 0, 0], CONFIG.default.server_port_http)).await;
    });

    if let Err(e) = tokio::try_join!(
        base_uniswap_v2_handle,
        base_uniswap_v2_swap_sync_handle,
        rest_api_handle
    ) {
        log::error!("Error occurred while joining tasks: {:?}", e);
    }

    Ok(())
}

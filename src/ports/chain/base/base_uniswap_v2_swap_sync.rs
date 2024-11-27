use std::error::Error;
use std::sync::Arc;
use std::time::Duration;
use async_trait::async_trait;
use ethers::middleware::Middleware;
use ethers::prelude::{Filter, Http, Provider, H256};
use once_cell::sync::Lazy;
use tokio::sync::Mutex;
use tokio::time::sleep;
use crate::config::CONFIG;
use crate::domain::entities::swap_log::SwapLog;
use crate::domain::services::data_storage_service::{DataStorageService, DataStorageTrait};

pub static PROTOCOL_ID: &str = "BASE_UNISWAP_V2";
pub static SWAP_SYNC_IN_PROGRESS: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));
pub static LAST_PROCESSED_BLOCK: Lazy<Mutex<u64>> = Lazy::new(|| Mutex::new(0));
#[derive(Debug)]
pub struct BaseUniswapV2SwapSynchronizer;

#[async_trait]
pub trait BaseUniswapV2SwapSynchronizerTrait: Send + Sync{
    async fn synchronize_swaps(&self) -> Result<(), Box<dyn std::error::Error>>;
    async fn extract_swap_events(
        data_service: Arc<DataStorageService>,
        provider: Arc<Provider<Http>>,
        from_block: u64,
        to_block: u64, step: usize
    ) -> Result<u64, Box<dyn Error>>;
    async fn get_processing_status(&self) -> bool;
    async fn get_last_processed_block(&self) -> u64;
    async fn is_block_processed_or_newer(&self, block: u64) -> bool;
}

#[async_trait]
impl BaseUniswapV2SwapSynchronizerTrait for BaseUniswapV2SwapSynchronizer{
    async fn synchronize_swaps(&self) -> Result<(), Box<dyn std::error::Error>> {

        let rpc_url = CONFIG.default.chain_base_rpc_url.clone();
        let data_service = Arc::new(DataStorageService);
        let provider = Arc::new(Provider::<Http>::try_from(rpc_url)?);
        let step = 1000;
        let mut from_block = 22800000;

        loop{
            log::info!("Swap event refresh ....  from block: {}",from_block);
            {
                let mut in_progress = SWAP_SYNC_IN_PROGRESS.lock().await;
                if *in_progress {
                    log::warn!("Synchronization already in progress. Skipping...");
                    sleep(Duration::from_secs(CONFIG.default.data_refresh_interval.clone())).await;
                    continue;
                }
                *in_progress = true;
            }
            //set in progress flag
            let latest_block = provider.get_block_number().await?.as_u64();
            log::info!("Swap event refresh to block {}",latest_block);
            if latest_block > from_block{
                Self::extract_swap_events(data_service.clone(),provider.clone(),from_block,latest_block,step).await?;
                from_block = latest_block;
            }
            else{
                log::info!("No new blocks to process");
                sleep(Duration::from_secs(CONFIG.default.data_refresh_interval.clone())).await;
            }
            {
                let mut in_progress = SWAP_SYNC_IN_PROGRESS.lock().await;
                *in_progress = false;
            }
            sleep(Duration::from_secs(CONFIG.default.data_refresh_interval.clone())).await;
        }
    }

    async fn extract_swap_events(
        data_service: Arc<DataStorageService>,
        provider: Arc<Provider<Http>>,
        from_block: u64,
        to_block: u64,
        step: usize,
    ) -> Result<u64, Box<dyn Error>> {
        let swap_event_signature: H256 = "0xd78ad95fa46c994b6551d0da85fc275fe613ce37657fb8d5e3d130840159d822"
            .parse()
            .unwrap();

        for block_start in (from_block..=to_block).step_by(step) {
            let block_end = std::cmp::min(block_start as usize + step - 1, to_block as usize);
            let mut attempts = 0;
            let max_attempts = 5;

            loop {
                attempts += 1;

                match provider.get_logs(
                    &Filter::new()
                        .topic0(swap_event_signature)
                        .from_block(block_start)
                        .to_block(block_end),
                ).await {
                    Ok(logs) => {
                        for log in logs {
                            let swap_log = SwapLog {
                                address: log.address,
                                topics: log.topics.clone(),
                                data: log.data.clone(),
                                block_hash: log.block_hash,
                                block_number: log.block_number,
                                transaction_hash: log.transaction_hash,
                                transaction_index: log.transaction_index,
                                log_index: log.log_index,
                                removed: log.removed,
                            };
                            data_service.add_swap_log(swap_log.address.to_string(), swap_log);
                        }
                        let mut last_block = LAST_PROCESSED_BLOCK.lock().await;
                        *last_block = block_end as u64;
                        log::info!("Swap sync: processed blocks: {} from {} : data set size: {} ",
                            block_end, to_block, data_service.get_all_swap_logs().len() );
                        break;
                    }
                    Err(e) if attempts < max_attempts => {
                        log::warn!(
                        "Error fetching logs between blocks {} and {}: {}. Retrying... ({}/{})",
                        block_start,
                        block_end,
                        e,
                        attempts,
                        max_attempts
                    );
                        sleep(Duration::from_secs(2_u64.pow(attempts))).await;
                    }
                    Err(e) => {
                        log::error!(
                        "Failed to fetch logs between blocks {} and {} after {} attempts: {}",
                        block_start,
                        block_end,
                        attempts,
                        e
                    );
                        return Err(e.into());
                    }
                }
            }
        }

        Ok(to_block)
    }

    async fn get_processing_status(&self) -> bool {
        let in_progress = SWAP_SYNC_IN_PROGRESS.lock().await;
        *in_progress
    }

    async fn get_last_processed_block(&self) -> u64 {
        let last_block = LAST_PROCESSED_BLOCK.lock().await;
        *last_block
    }

    async fn is_block_processed_or_newer(&self, block: u64) -> bool {
        let last_block = LAST_PROCESSED_BLOCK.lock().await;
        *last_block >= block
    }
}
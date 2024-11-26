use std::sync::Arc;
use std::time::SystemTime;
use ethers::contract::abigen;
use ethers::providers::{Http, Provider};
use async_trait::async_trait;
use ethers::types::{Address, U256};
use crate::config::CONFIG;
use log;
use tokio::time::{sleep, Duration};
use crate::domain::entities::token::Token;
use crate::domain::entities::types::date_time::DateTime;
use crate::domain::services::data_storage_service::{DataStorageService, DataStorageTrait};

abigen!(
    UniswapV2Factory,
    r#"[
        function allPairsLength() external view returns (uint256)
        function allPairs(uint256) external view returns (address)
    ]"#,
);

abigen!(
    UniswapV2Pair,
    r#"[
        function token0() external view returns (address)
        function token1() external view returns (address)
    ]"#,
);

abigen!(
    ERC20,
    r#"[
        function name() external view returns (string)
        function symbol() external view returns (string)
        function decimals() external view returns (uint8)
    ]"#,
);

#[derive(Debug)]
pub struct BaseUniswapV2ClientService;

#[async_trait]
pub trait BaseUniswapV2ClientServiceTrait: Send + Sync{
    async fn init_chain_data_sync(&self) -> Result<(), Box<dyn std::error::Error>>;
    async fn init_token_list(&self, data_service: Arc<DataStorageService>) -> Result<(), Box<dyn std::error::Error>>;
    async fn fetch_token_details(
        address: Address,provider: Arc<Provider<Http>>
        ) -> Result<Token, Box<dyn std::error::Error>>;
}

#[async_trait]
impl BaseUniswapV2ClientServiceTrait for BaseUniswapV2ClientService {
    async fn init_chain_data_sync(&self) -> Result<(), Box<dyn std::error::Error>> {

        let data_service = Arc::new(DataStorageService);

        log::info!("Start Base Network connection ...");
        log::info!("RPC Endpoint: {}",CONFIG.default.chain_base_rpc_url);

        data_service.init();

        loop{
            if data_service.get_tokens_size() == 0{
                let _ = Self::init_token_list(&self,data_service.clone()).await;
            }
            else{
                log::info!("Lista tokenów: {}", data_service.get_tokens_size())
            }
            log::info!("Base Network UniswapV2 - data refresh");
            sleep(Duration::from_secs(CONFIG.default.data_refresh_interval.clone())).await;
        }
    }

    async fn init_token_list(&self, data_service: Arc<DataStorageService>) -> Result<(), Box<dyn std::error::Error>>{
        log::info!("Initial full token load");
        let factory_address: Address = CONFIG.default.chain_base_uniswap_v2_factory_address.clone().parse()?;

        let rpc_url = CONFIG.default.chain_base_rpc_url.clone();
        let provider = Arc::new(Provider::<Http>::try_from(rpc_url)?);

        let factory = UniswapV2Factory::new(factory_address, provider.clone());

        let pair_count: U256 = factory.all_pairs_length().call().await?;
        log::info!("Full pair list size: {}", pair_count);

        let mut token_addresses = vec![];

        for i in 0..pair_count.as_u64() {
            let pair_address: Address = factory.all_pairs(U256::from(i)).call().await?;
            let pair = UniswapV2Pair::new(pair_address, provider.clone());

            let token0: Address = pair.token_0().call().await?;
            let token1: Address = pair.token_1().call().await?;

            token_addresses.push(token0);
            token_addresses.push(token1);

            log::info!("Pair {}: Token0: {}, Token1: {}", i, token0, token1);
        }

        token_addresses.sort();
        token_addresses.dedup();

        log::info!("\nUnique tokens on BASE Network:");

        for token_address in token_addresses{
            match Self::fetch_token_details(token_address, provider.clone()).await {
                Ok(token) => {
                    data_service.add_token(token);
                }
                Err(e) => log::error!("Fetch error for token {}: {:?}", token_address, e),
            }
        }
        Ok(())
    }

    async fn fetch_token_details(address: Address, provider: Arc<Provider<Http>>
        ) -> Result<Token, Box<dyn std::error::Error>> {
        let token = ERC20::new(address, provider);

        let name: String = token.name().call().await.unwrap_or_else(|_| "No data".to_string());
        let symbol: String = token.symbol().call().await.unwrap_or_else(|_| "No data".to_string());
        let decimals: u8 = token.decimals().call().await.unwrap_or(0);

        log::info!("Token: {} ({}) | Address: {} | Decimals: {}", name, symbol, address, decimals);

        let token_object = Token {
            address: format!("{:?}", address),
            protocol_id: "BASE_UNISWAP_V2".to_string(),
            symbol,
            name,
            decimals: decimals.to_string(),
            retrieved_at: DateTime(SystemTime::now()),
            updated_at: DateTime(SystemTime::now()),
            active: true,
        };

        Ok(token_object)
    }
}

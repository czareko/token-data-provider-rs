use std::env;
use config::{Config, File};
use serde::Deserialize;
use std::error::Error;
use std::sync::Arc;
use once_cell::sync::Lazy;

#[derive(Debug, Deserialize)]
pub struct DefaultConfig {
    pub server_port_http: u16,
    pub data_refresh_interval: u64,
    pub chain_base_rpc_url: String,
    pub chain_base_uniswap_v2_factory_address: String,
    pub chain_base_uniswap_v3_factory_address: String,
}

enum EnvVar {
    ServerPortHTTP,
    DataRefreshInterval,
    ChainBaseRPCURL,
    ChainBaseUniswapV2FactoryAddress,
    ChainBaseUniswapV3FactoryAddress
}

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub default: DefaultConfig,
}

// Lazy static configuration loading
pub static CONFIG: Lazy<Arc<AppConfig>> = Lazy::new(|| {
    log::info!("Configuration loading ...");
    match load_config_from_env_or_file() {
        Ok(config) => Arc::new(config),
        Err(e) => {
            log::error!("Failed to load configuration: {:?}", e);
            panic!("Failed to load configuration: {:?}", e);
        }
    }
});

impl EnvVar {
    // Returns the environment variable name as a &str
    fn as_str(&self) -> &str {
        match self {
            EnvVar::ServerPortHTTP => "SERVER_PORT_HTTP",
            EnvVar::DataRefreshInterval => "DATA_REFRESH_INTERVAL",
            EnvVar::ChainBaseRPCURL => "CHAIN_BASE_RPC_URL",
            EnvVar::ChainBaseUniswapV2FactoryAddress => "CHAIN_BASE_UNISWAP_V2_FACTORY_ADDRESS",
            EnvVar::ChainBaseUniswapV3FactoryAddress => "CHAIN_BASE_UNISWAP_V3_FACTORY_ADDRESS"
        }
    }

    // Fetches the value from the environment, attempts to parse it into the desired type, or returns the default value
    fn get_value<T: std::str::FromStr + Clone>(&self, default: &T) -> T
    where
        T::Err: std::fmt::Debug,
    {
        env::var(self.as_str())
            .ok()
            .and_then(|val| val.parse::<T>().ok()) // Try to parse the value to type T
            .unwrap_or_else(|| default.clone()) // Clone the default value if parsing fails
    }
}

// Loads the configuration from a file (config.toml)
pub fn load_config() -> Result<AppConfig, Box<dyn Error>> {
    let mut settings = Config::default();

    // Load configuration from file
    settings.merge(File::with_name("resources/config.toml"))?;

    // Parse the configuration into the AppConfig structure
    let app_config: AppConfig = settings.try_into()?;

    Ok(app_config)
}

// Load the configuration from environment variables, overriding values from the file if present
pub fn load_config_from_env_or_file() -> Result<AppConfig, Box<dyn Error>> {
    // First, load the configuration from the file
    let mut config = load_config()?;

    // Override with environment variables if they exist
    config.default.server_port_http = EnvVar::ServerPortHTTP
        .get_value(&config.default.server_port_http); // u16 for server_port_http

    config.default.data_refresh_interval = EnvVar::DataRefreshInterval
        .get_value(&config.default.data_refresh_interval); // u16 refresh data interval in seconds

    // Override with environment variables if they exist
    config.default.chain_base_rpc_url = EnvVar::ChainBaseRPCURL
        .get_value(&config.default.chain_base_rpc_url); // String for chain RPC URL

    config.default.chain_base_uniswap_v2_factory_address = EnvVar::ChainBaseUniswapV2FactoryAddress
        .get_value(&config.default.chain_base_uniswap_v2_factory_address); // String for Factory Address

    config.default.chain_base_uniswap_v3_factory_address = EnvVar::ChainBaseUniswapV3FactoryAddress
        .get_value(&config.default.chain_base_uniswap_v3_factory_address); // String for Factory V3 address

    log::info!("Config loaded: {:?}",config);

    Ok(config)
}
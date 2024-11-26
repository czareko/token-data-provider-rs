use ethers::contract::abigen;
use ethers::providers::{Http, Provider};
use ethers::types::{Address, U256};
use std::sync::Arc;

abigen!(
    // Definicja kontraktu Uniswap Factory
    UniswapFactory,
    r#"[
        function allPairsLength() external view returns (uint256)
        function allPairs(uint256) external view returns (address)
    ]"#,
);

abigen!(
    // Definicja kontraktu Uniswap Pair (LP Token)
    UniswapPair,
    r#"[
        function token0() external view returns (address)
        function token1() external view returns (address)
    ]"#,
);

abigen!(
    // Definicja ERC-20
    ERC20,
    r#"[
        function name() external view returns (string)
        function symbol() external view returns (string)
        function decimals() external view returns (uint8)
    ]"#,
);

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Adres kontraktu Factory w sieci BASE
    let factory_address: Address = "0x04C9f118d21e8B767D2e50C946f0cC9F6C367300".parse()?;

    // Ustawienie dostawcy RPC (sieć BASE) z `Arc`
    let rpc_url = "https://mainnet.base.org"; // Twój URL RPC
    let provider = Arc::new(Provider::<Http>::try_from(rpc_url)?);

    // Podłącz do Factory
    let factory = UniswapFactory::new(factory_address, provider.clone());

    // Pobierz liczbę par w Uniswap
    let pair_count: U256 = factory.all_pairs_length().call().await?;
    println!("Liczba par na Uniswap: {}", pair_count);

    let mut token_addresses = vec![];

    // Iteruj po wszystkich parach
    for i in 0..pair_count.as_u64() {
        let pair_address: Address = factory.all_pairs(U256::from(i)).call().await?;
        let pair = UniswapPair::new(pair_address, provider.clone());

        // Pobierz adresy tokenów z pary
        let token0: Address = pair.token_0().call().await?;
        let token1: Address = pair.token_1().call().await?;

        // Dodaj tokeny do listy
        token_addresses.push(token0);
        token_addresses.push(token1);

        println!("Para {}: Token0: {}, Token1: {}", i, token0, token1);
    }

    // Usuń duplikaty tokenów
    token_addresses.sort();
    token_addresses.dedup();

    println!("\nUnikalne tokeny w sieci BASE:");

    // Pobierz szczegóły każdego tokena
    for token_address in token_addresses {
        match fetch_token_details(token_address, provider.clone()).await {
            Ok(_) => {}
            Err(e) => println!("Błąd przy pobieraniu danych dla {}: {:?}", token_address, e),
        }
    }

    Ok(())
}

async fn fetch_token_details(
    address: Address,
    provider: Arc<Provider<Http>>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Podłącz do kontraktu ERC-20
    let token = ERC20::new(address, provider);

    // Pobierz nazwę, symbol i liczbę miejsc dziesiętnych
    let name: String = token.name().call().await.unwrap_or_else(|_| "Brak danych".to_string());
    let symbol: String = token.symbol().call().await.unwrap_or_else(|_| "Brak danych".to_string());
    let decimals: u8 = token.decimals().call().await.unwrap_or(0);

    println!("Token: {} ({}) | Address: {} | Decimals: {}", name, symbol, address, decimals);
    Ok(())
}
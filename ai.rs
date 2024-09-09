use reqwest::Client;
use solana_sdk::transaction::Transaction;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct DexPriceInfo {
    symbol: String,
    price: f64,
}

pub async fn get_dex_price_info(token: &str) -> Result<DexPriceInfo, Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = format!("https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies=usd", token);
    
    let response = client.get(&url).send().await?.json::<serde_json::Value>().await?;
    let price = response[token]["usd"].as_f64().unwrap();
    
    Ok(DexPriceInfo {
        symbol: token.to_string(),
        price,
    })
}

pub async fn check_sandwich_opportunity(tx: &Transaction) -> bool {
    let dex_price_info = get_dex_price_info("solana").await.unwrap();
    
    // Example: Simple slippage check
    let slippage_threshold = 0.01; // 1% slippage threshold
    let mempool_price = 50.0; // Mock transaction price
    
    let slippage = (mempool_price - dex_price_info.price).abs() / dex_price_info.price;
    slippage > slippage_threshold
}

pub async fn check_front_run_opportunity(tx: &Transaction) -> bool {
    let dex_price_info = get_dex_price_info("solana").await.unwrap();
    
    let mempool_price = 50.0; // Mock transaction price
    let threshold = 0.01; // 1% profit threshold
    
    let price_difference = (mempool_price - dex_price_info.price).abs();
    price_difference > threshold
}

// mod api;

// #[tokio::main]
// async fn main() {
//     // main::main_class::run()

//     let response = api::hyperliquid::Hyperliquid::new()
//         .get_funding_rates()
//         .await;
//     println!("response = {:?}", response.unwrap());
// }

use ethers::contract::{Contract, ContractFactory};
use ethers::prelude::*;
use ethers::providers::{Http, Provider};
use ethers::types;
use ethers::types::{Address, U256};
use ethers::utils::Anvil;
use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use tokio;

#[derive(Deserialize)]
struct Base {
    chain_id: u64,
    rpc_url: String,
}

#[derive(Deserialize)]
struct MarketMetaData {
    market_name: String,
    symbol: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Generate the contract from the abi
    abigen!(PerpsMarketProxy, "src/abi/PerpsMarketProxy.json",);

    let symbols: [(u128, &str); 32] = [
        (200, "BTC"),
        (100, "ETH"),
        (300, "SNX"),
        (400, "SOL"),
        (600, "W"),
        (500, "WIF"),
        (1600, "ARB"),
        (900, "AVAX"),
        (1800, "BNB"),
        (1400, "BONK"),
        (800, "DOGE"),
        (700, "ENA"),
        (1500, "FTM"),
        (1700, "MATIC"),
        (1000, "OP"),
        (1100, "ORDI"),
        (1200, "PEPE"),
        (1300, "RUNE"),
        (2600, "ARKM"),
        (3200, "AXL"),
        (2900, "BOME"),
        (3000, "ETHFI"),
        (2700, "GALA"),
        (2200, "GMX"),
        (2100, "INJ"),
        (1900, "LINK"),
        (2000, "PENDLE"),
        (3100, "STX"),
        (2400, "SUI"),
        (2800, "TAO"),
        (2300, "TIA"),
        (2500, "TON"),
    ];

    let base = Base {
        chain_id: 8453,
        rpc_url: "https://mainnet.base.org".to_string(),
    };

    let provider = Provider::<Http>::try_from(base.rpc_url)?;

    let perps_market_proxy_address: Address =
        "0x0A2AF931eFFd34b81ebcc57E3d3c9B1E1dE1C9Ce".parse()?;

    // let perps_market_proxy_abi = serde_json::from_str(include_str!("PerpsMarketProxy.json"))?;

    // Create a contract instance
    // let contract = Contract::new(perps_market_proxy_address, perps_market_proxy_abi, provider.clone());

    let contract = PerpsMarketProxy::new(perps_market_proxy_address, provider.clone().into());

    // Call the getMarkets function to get market IDs
    // let market_ids = contract.get_markets().call().await?;

    let mut funding_rates: HashMap<String, f64> = HashMap::new();

    for symbol in symbols {
        let market = contract.get_market_summary(symbol.0).call().await?;
        // Convert funding rate from wei to eth and multiply by 100 to convert to percentage and divide by 24 to get hourly
        let fr = convert_to_hourly_percent(wei_to_eth(market.current_funding_rate).unwrap());
        funding_rates.insert(symbol.1.to_string(), fr);
    }

    println!("FR = {:?}", funding_rates);

    Ok(())
}

fn wei_to_eth(value: I256) -> Result<f64, Box<dyn Error>> {
    let wei_value = value.to_string().parse::<f64>()?;
    let decimals = 18;
    let conversion_factor = 10u64.pow(decimals).to_string().parse::<f64>()?;
    Ok(wei_value / conversion_factor)
}

fn convert_to_hourly_percent(value: f64) -> f64 {
    value * 100.0 / 24.0
}

//TODO:
// Convert market IDS to some tickers
// Create a mapping from tickers to funding rates
// Convert funding rates to proper percentages

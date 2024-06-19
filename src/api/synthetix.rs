use ethers::prelude::*;
use ethers::providers::{Http, Provider};
use ethers::types::Address;
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;

// Generate the contract from the abi
abigen!(PerpsMarketProxy, "src/abi/PerpsMarketProxy.json",);

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

struct FundingRate {
    current_funding_rate: f64,
}

impl FundingRate {
    fn new(value: I256) -> Self {
        Self { 
            current_funding_rate: value.to_string().parse::<f64>().unwrap(),
        }
    }

    fn convert_wei_to_eth(&mut self) {
        let decimals = 18;
        let conversion_factor = 10u64.pow(decimals).to_string().parse::<f64>().unwrap();
        self.current_funding_rate /= conversion_factor;
    }
    
    fn convert_to_hourly_percent(&mut self) {
        self.current_funding_rate *= 100.0 / 24.0;
    }
}

pub struct Synthetix<'a> {
    symbols: [(u128, &'a str); 32],
}

impl<'a> Synthetix<'a> {
    fn new() -> Self {
        Self {
            symbols: [
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
            ],
        }
    }

    fn setup(&self) -> Result<PerpsMarketProxy<Provider<Http>>, Box<dyn Error>> {
        let base = Base {
            chain_id: 8453,
            rpc_url: "https://mainnet.base.org".to_string(),
        };
        let provider = Provider::<Http>::try_from(base.rpc_url)?;
        let perps_market_proxy_address: Address = "0x0A2AF931eFFd34b81ebcc57E3d3c9B1E1dE1C9Ce".parse()?;
        let contract = PerpsMarketProxy::new(perps_market_proxy_address, provider.clone().into());

        // Return the Perps Market Proxy contract
        Ok(contract)
    }

    pub async fn get_funding_rates(&self) -> Result<HashMap<String, f64>, Box<dyn Error>> {
        let contract = self.setup()?;

        let mut funding_rates: HashMap<String, f64> = HashMap::new();

        for symbol in self.symbols {
            let market = contract.get_market_summary(symbol.0).call().await?;
            let mut fr = FundingRate::new(market.current_funding_rate);
            fr.convert_wei_to_eth();
            fr.convert_to_hourly_percent();
            funding_rates.insert(symbol.1.to_string(), fr.current_funding_rate);
        }

        Ok(funding_rates)
    }
}

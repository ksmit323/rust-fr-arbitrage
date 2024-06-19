// src/api/hyperliquid.rs

use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;

const URL_BASE: &str = " https://api.hyperliquid.xyz/info";

#[derive(Serialize)]
struct RequestBody<'a> {
    r#type: &'a str,
}

#[derive(Deserialize, Debug)]
struct AssetInfo {
    name: String,
}

#[derive(Deserialize, Debug)]
struct AssetContext {
    funding: String,
}

pub struct Hyperliquid {
    url: String,
}

impl Hyperliquid {
    pub fn new() -> Self {
        Self {
            url: URL_BASE.to_string(),
        }
    }

    async fn get_market_data(&self) -> Result<Value, Box<dyn Error>> {
        // Headers for the request
        // let headers = [("Content-Type", "application/json")];

        // Request body data
        let body = RequestBody {
            r#type: "metaAndAssetCtxs",
        };
        // Send request to API
        let client = Client::new();
        let response = client
            .post(&self.url)
            // .headers(headers.into())
            .json(&body)
            .send()
            .await?;

        // Parse response JSON
        let json: Value = response.json().await?;

        Ok(json)
    }

    pub async fn get_funding_rates(&self) -> Result<HashMap<String, f64>, Box<dyn Error>> {
        // Get market data
        let market_data = self.get_market_data().await?;

        // Parse asset info and context
        let asset_info: Vec<AssetInfo> =
            serde_json::from_value(market_data[0]["universe"].clone())?;
        let asset_context: Vec<AssetContext> = serde_json::from_value(market_data[1].clone())?;

        // Initialize Hashmap
        let mut funding_rates: HashMap<String, f64> = HashMap::new();

        // Iterate over both lists, assuming their indexes are aligned
        for (asset, context) in asset_info.iter().zip(asset_context.iter()) {
            let symbol = asset.name.clone();
            let funding_rate: f64 = context.funding.parse::<f64>()? * 100.0; // Convert 1hr rate to percentage
            funding_rates.insert(symbol, funding_rate);
        }

        Ok(funding_rates)
    }
}

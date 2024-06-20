mod hyperliquid;
mod master_caller;
mod synthetix;

use polars::prelude::*;
use std::collections::HashMap;
use std::error::Error;
use std::env;

use master_caller::MasterCaller;

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn Error>> {

    let master_caller = MasterCaller::new();
    let funding_rates = master_caller.get_funding_rates().await?;
    
    // Convert funding_rates to DataFrame table
    let display_rates_table = build_funding_rate_table(funding_rates)?;

    // Print the entire DataFrame
    println!("{}", display_rates_table.to_string());
    
    Ok(())
}

fn build_funding_rate_table(
    funding_rates: HashMap<String, HashMap<String, f64>>,
) -> Result<DataFrame> {
    let mut symbols: Vec<String> = Vec::new();
    let mut hyperliquid_rates: Vec<f64> = Vec::new();
    let mut synthetix_rates: Vec<f64> = Vec::new();
    
    for (symbol, rates) in funding_rates.iter() {
        symbols.push(symbol.clone());
        hyperliquid_rates.push(*rates.get("Hyperliquid").unwrap_or(&f64::NAN));
        synthetix_rates.push(*rates.get("Synthetix").unwrap_or(&f64::NAN));
    }

    let difference_rates: Vec<f64> = hyperliquid_rates
        .iter()
        .zip(&synthetix_rates)
        .map(|(h_rate, s_rate)| (h_rate - s_rate).abs())
        .collect();
    
    let df = df!(
        "Symbol" => symbols,
        "Hyperliquid" => hyperliquid_rates,
        "Synthetix" => synthetix_rates,
        "Difference" => difference_rates
    )?;
        
    // Filter out rows with NaN in the Difference column
    let mask = df.column("Difference")?.f64()?.is_not_nan();
    let df = df.filter(&mask)?;
    
    // Sort DataFrame by Difference column in descending order
    let sorted_df = df.sort(["Difference"], true)?;
    
    // Get top rows
    let top_df = sorted_df.head(Some(10));
    
    Ok(top_df)
}

// // Target about $0.43/hr
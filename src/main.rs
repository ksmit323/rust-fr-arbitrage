mod hyperliquid;
mod master_caller;
mod synthetix;

use master_caller::MasterCaller;
use polars::prelude::*;
use std::collections::HashMap;
use std::error::Error;

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn Error>> {
    let master_caller = MasterCaller::new();
    let funding_rates = master_caller.get_funding_rates().await?;

    let display_rate_table = build_funding_rate_table(funding_rates)?;
    println!("{:?}", display_rate_table);

    Ok(())
}

fn build_funding_rate_table(
    funding_rates: HashMap<String, HashMap<String, f64>>,
) -> Result<DataFrame> {
    // Collect all exchanges
    let mut exchanges_set = std::collections::HashSet::new();
    for exchange_rates in funding_rates.values() {
        for exchange in exchange_rates.keys() {
            exchanges_set.insert(exchange.clone());
        }
    }
    let exchanges: Vec<String> = exchanges_set.into_iter().collect();

    // Create initial DataFrame with symbol column
    let mut df = DataFrame::new(vec![Series::new(
        "Symbol",
        funding_rates.keys().cloned().collect::<Vec<String>>(),
    )])?;

    // Add a column for each exchange
    for exchange in &exchanges {
        let mut rates = Vec::new();
        for symbol in funding_rates.keys() {
            let rate = funding_rates
                .get(symbol)
                .and_then(|r| r.get(exchange))
                .cloned()
                .unwrap_or(f64::NAN);
            rates.push(rate);
        }
        df = df.hstack(&[Series::new(exchange, rates)])?;
    }

    // Add a "Difference" column for the absolute difference between the two exchanges
    let exchange1 = &exchanges[0];
    let exchange2 = &exchanges[1];
    let exchange1_series = df.column(exchange1)?;
    let exchange2_series = df.column(exchange2)?;

    let difference: Vec<f64> = exchange1_series
        .f64()?
        .into_iter()
        .zip(exchange2_series.f64()?)
        .map(|(rate1, rate2)| match (rate1, rate2) {
            (Some(r1), Some(r2)) => (r1 - r2).abs(),
            _ => f64::NAN,
        })
        .collect();

    df = df.hstack(&[Series::new("Difference", difference)])?;

    // Filter out rows where the Difference is NaN
    let diff_series = df.column("Difference")?.f64()?;
    let mask = diff_series
        .into_iter()
        .map(|opt| opt.is_some() && !opt.unwrap().is_nan())
        .collect::<Vec<bool>>();

    let mask_series = BooleanChunked::from_slice("mask", &mask);
    df = df.filter(&mask_series)?;

    // Sort the DF by by the Diffence column in descending order
    df = df.sort(["Difference"], true)?;

    // Get the top rows
    let top_df = df.head(Some(10));

    Ok(top_df)
}


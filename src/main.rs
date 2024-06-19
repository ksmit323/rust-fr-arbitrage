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
    todo!("Build funding rate table")
}

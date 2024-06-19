mod hyperliquid;
mod synthetix;
mod master_caller;

use master_caller::MasterCaller;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let master_caller = MasterCaller::new();
    let funding_rates = master_caller.get_funding_rates().await?;

    for (symbol, rates) in funding_rates {
        println!("{}: {:?}", symbol, rates);
    }

    Ok(())
}

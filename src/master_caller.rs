use crate::hyperliquid::*;
use crate::synthetix::*;

use std::collections::HashMap;
use std::error::Error;

pub struct MasterCaller;

impl MasterCaller {
    pub fn new() -> Self {
        MasterCaller
    }

    pub async fn get_funding_rates(
        &self,
    ) -> Result<HashMap<String, HashMap<String, f64>>, Box<dyn Error>> {
        /*
        Returns:
            HashMap: A mapping where keys are symbols and values are mappings of DEX names and rates.
            i.e.    # "BTC":
                        # Synthetix:   0.006
                        # Hyperliquid: 0.001
         */

        let hyperliquid = Hyperliquid::new();
        let synthetix = Synthetix::new();

        let (hyperliquid_rates_result, synthetix_rates_result) = tokio::join!(
            hyperliquid.get_funding_rates(),
            synthetix.get_funding_rates()
        );

        // Handle potential errors from both API calls
        let hyperliquid_rates = hyperliquid_rates_result?;
        let synthetix_rates = synthetix_rates_result?;

        let mut funding_rates: HashMap<String, HashMap<String, f64>> = HashMap::new();

        // Insert Hyperliquid rates into the funding_rates map
        for (symbol, rate) in hyperliquid_rates {
            funding_rates
                .entry(symbol.clone())
                .or_insert_with(HashMap::new)
                .insert("Hyperliquid".to_string(), rate);
        }

        // Insert Synthetix rates into the funding_rates map
        for (symbol, rate) in synthetix_rates {
            funding_rates
                .entry(symbol.clone())
                .or_insert_with(HashMap::new)
                .insert("Synthetix".to_string(), rate);
        }

        Ok(funding_rates)
    }
}

mod api;

#[tokio::main]
async fn main() {
    // main::main_class::run()

    let response = api::hyperliquid::Hyperliquid::new()
        .get_funding_rates()
        .await;
    println!("response = {:?}", response.unwrap());
}

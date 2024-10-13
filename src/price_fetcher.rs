// src/price_fetcher.rs
use reqwest::Error;
use serde::Deserialize;
use tokio::time::{self, Duration};

#[derive(Deserialize, Debug)]
pub struct PriceResponse {
    bitcoin: BtcPrice,
}

#[derive(Deserialize, Debug)]
pub struct BtcPrice {
    usd: f64,
}

pub async fn fetch_bitcoin_price() -> Result<(), Error> {
    let client = reqwest::Client::new();
    let url = "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd";

    loop {
        let resp = client.get(url).send().await?.json::<PriceResponse>().await?;
        println!("Current Bitcoin Price in USD: ${}", resp.bitcoin.usd);

        // 延迟下一次请求
        time::sleep(Duration::from_secs(60)).await;
    }
}

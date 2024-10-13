// src/volume_fetcher.rs
use reqwest::{Client, Error};
use serde::Deserialize;
use tokio::time::{self, Duration};

#[derive(Deserialize, Debug)]
struct MarketData {
    total_volume: serde_json::Value,
}

#[derive(Deserialize, Debug)]
struct CoinGeckoResponse {
    market_data: MarketData,
}

pub async fn fetch_bitcoin_volume() -> Result<(), Error> {
    let client = Client::new();
    let url = "https://api.coingecko.com/api/v3/coins/bitcoin";

    loop {
        let response = client.get(url).send().await?;

        if response.status().is_success() {
            let data: CoinGeckoResponse = response.json().await?;
            let volume = &data.market_data.total_volume["usd"];

            println!("Current Bitcoin Volume: {:?}", volume);
        } else {
            println!("Failed to fetch data");
        }

        // 延迟下一次请求
        time::sleep(Duration::from_secs(60)).await;
    }
}

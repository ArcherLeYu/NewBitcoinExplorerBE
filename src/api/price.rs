// src/price.rs
use crate::db::Database; // 确保正确导入Database
use reqwest::{Client, Error};
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

pub async fn fetch_bitcoin_price(db: Database) -> Result<(), Error> {
    let client = Client::new();
    let url = "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd";

    loop {
        let resp = client.get(url).send().await?.json::<PriceResponse>().await?;
        println!("Current Bitcoin Price in USD: ${}", resp.bitcoin.usd);

        // 插入价格到数据库
        if let Err(e) = db.insert_bitcoin_price(resp.bitcoin.usd) {
            eprintln!("Failed to insert bitcoin price into database: {:?}", e);
        }

        // 延迟下一次请求
        time::sleep(Duration::from_secs(600)).await;
    }
}

use reqwest::Client;
use serde::Deserialize;
use crate::models::BlockSummary;
use crate::errors::AppError;

#[derive(Deserialize, Debug)]
struct LatestBlockResponse {
    hash: String,
    height: u64,
    time: u64, // The timestamp of the latest block
}

pub async fn fetch_latest_block() -> Result<BlockSummary, AppError> {
    let client = Client::new();
    let url = "https://blockchain.info/latestblock";

    let response = client.get(url)
        .send()
        .await?
        .json::<LatestBlockResponse>()
        .await?;

    // Convert the response to your existing BlockSummary model
    let block_summary = BlockSummary {
        height: response.height,
        hash: response.hash,
    };

    Ok(block_summary)
}

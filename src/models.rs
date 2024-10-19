// models.rs
use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Serialize, Debug)]
pub struct BlockSummary {
    pub height: u64,
    pub hash: String,
}


#[derive(Deserialize, Serialize, Debug)]
pub struct PriceList {
    pub prices: Vec<f64>,
}

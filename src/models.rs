// models.rs
use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize,Serialize,Debug)]
pub struct BlockchainInfo {
    pub automatic_pruning: bool,
    pub bestblockhash: String,
    pub blocks: u64,
    pub chain: String,
    pub chainwork: String,
    pub difficulty: f64,
    pub headers: u64,
    pub initialblockdownload: bool,
    pub mediantime: u64,
    pub prune_target_size: u64,
    pub pruned: bool,
    pub pruneheight: u64,
    pub size_on_disk: u64,
    pub time: u64,
    pub verificationprogress: f64,
    pub warnings: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct BlockInfo {
    pub hash: String,
    pub confirmations: u64,
    pub size: u64,
    pub height: u64,
    pub version: u64,
    pub version_hex: Option<String>,  // 修改为可选字段
    pub merkleroot: String,
    pub time: u64,
    pub mediantime: u64,
    pub nonce: u64,
    pub bits: String,
    pub difficulty: f64,
    pub chainwork: String,
    pub n_tx: Option<u64>,
    pub previousblockhash: Option<String>,
    pub nextblockhash: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct BlockSummary {
    pub height: u64,
    pub hash: String,
}


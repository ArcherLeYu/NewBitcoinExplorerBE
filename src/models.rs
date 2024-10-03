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

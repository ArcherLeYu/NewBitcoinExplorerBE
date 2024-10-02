//main.rs
mod config;
mod rpc_client;
mod db;
mod models;
mod utils;
mod errors;
mod retryable;

use crate::errors::AppError; // 导入AppError
use tokio::time::{self, Duration};

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let config = config::Config::new();
    let database = db::Database::new(&config.db_connection_string)?;

    let mut interval = time::interval(Duration::from_secs(10)); // 每10秒运行一次

    loop{
        interval.tick().await;
        let blockchain_info = retryable::fetch_blockchain_info_with_retry(&config).await?;
        println!("Blockchain Info: {:?}", blockchain_info);

        let hash_exists = database.check_block_hash_exists(&blockchain_info.bestblockhash)?;

        if !hash_exists {
            // 如果不存在，插入新的区块信息
            println!("新区块链hash {:?}", blockchain_info.bestblockhash);
            database.insert_block_info(&blockchain_info)?;
        } else {
            // 如果存在，可以选择打印信息或执行其他逻辑
            println!("没有新区块生成.旧区块链hash：{}", blockchain_info.bestblockhash);
        }
    }


    Ok(())
}


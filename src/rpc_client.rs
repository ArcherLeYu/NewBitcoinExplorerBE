// rpc_client.rs
use reqwest::Client;
use serde_json::json;
use crate::models::BlockchainInfo;
use crate::config::Config;
use crate::errors::AppError;  // 引入自定义错误类型

pub async fn get_blockchain_info(config: &Config) -> Result<BlockchainInfo, AppError> {
    let client = Client::new();
    let res = client.post(&config.rpc_url)
        .basic_auth(&config.rpc_user, Some(&config.rpc_password))
        .json(&json!({"jsonrpc": "1.0", "id": "curltest", "method": "getblockchaininfo", "params": []}))
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    let info = serde_json::from_value::<BlockchainInfo>(res["result"].clone())
        .map_err(|e| AppError::Json(e))?;

    Ok(info)
}

// pub async fn get_block_count(config: &Config) -> Result<BlockchainInfo, AppError> {
//     let client = Client::new();
//     let res = client.post(&config.rpc_url)
//         .basic_auth(&config.rpc_user, Some(&config.rpc_password))
//         .json(&json!({"jsonrpc": "1.0", "id": "curltest", "method": "getblockcount", "params": []}))
//         .send()
//         .await?
//         .json::<serde_json::Value>()
//         .await?;
//
//     let info = serde_json::from_value::<BlockchainInfo>(res["result"].clone())
//         .map_err(|e| AppError::Json(e))?;
//
//     Ok(info)
// }
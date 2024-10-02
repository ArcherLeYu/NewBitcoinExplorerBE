// retryable.rs
use crate::config::Config;
use crate::rpc_client;
use crate::errors::AppError;
use crate::models::BlockchainInfo;
use tokio::time::{self, Duration};

pub async fn fetch_blockchain_info_with_retry(config: &Config) -> Result<BlockchainInfo, AppError> {
    let retry_interval = Duration::from_secs(2); // 重试间隔2秒
    let max_retries = 3; // 最大重试次数
    let mut attempts = 0;

    loop {
        match rpc_client::get_blockchain_info(config).await {
            Ok(info) => return Ok(info), // 成功获取数据，返回结果
            Err(e) if attempts < max_retries => {
                eprintln!("Attempt {} failed: {}. Retrying...", attempts + 1, e);
                attempts += 1;
                time::sleep(retry_interval).await; // 等待2秒
            },
            Err(e) => {
                eprintln!("Failed to retrieve blockchain information after {} attempts: {}", max_retries, e);
                return Err(e); // 重试次数耗尽，返回错误
            }
        }
    }
}

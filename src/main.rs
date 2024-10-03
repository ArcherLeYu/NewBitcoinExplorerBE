//main.rs
mod config;
mod rpc_client;
mod db;
mod models;
mod utils;
mod errors;
mod retryable;
mod api;

use actix_web::rt::spawn; // 导入 Actix 的运行时 spawn 函数
use actix_web::{web, App, HttpServer};// 导入其他需要的 Actix 组件
use crate::errors::AppError; // 导入 AppError
use tokio::time::{self, Duration};
use std::sync::Arc;


#[actix_web::main]
async fn main() -> Result<(), AppError> {
    // 创建配置对象
    let config = config::Config::new();

    // 创建数据库连接池
    let database = Arc::new(db::Database::new(&config.db_connection_string)?);

    // 使用 Tokio 并发来定时从 Bitcoin Core 获取区块链数据
    let database_clone = Arc::clone(&database);
    tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(10)); // 每10秒运行一次

        loop {
            interval.tick().await;  // 每隔10秒触发一次
            match retryable::fetch_blockchain_info_with_retry(&config).await {
                Ok(blockchain_info) => {
                    println!("Blockchain Info: {:?}", blockchain_info);

                    // 检查区块哈希是否存在
                    let hash_exists = database_clone.check_block_hash_exists(&blockchain_info.bestblockhash).unwrap_or(false);

                    if !hash_exists {
                        // 如果不存在，插入新的区块信息
                        println!("新区块链hash {:?}", blockchain_info.bestblockhash);
                        if let Err(e) = database_clone.insert_block_info(&blockchain_info) {
                            eprintln!("Failed to insert block info: {:?}", e);
                        }
                    } else {
                        // 如果存在，可以选择打印信息或执行其他逻辑
                        println!("没有新区块生成.旧区块链hash：{}", blockchain_info.bestblockhash);
                    }
                }
                Err(e) => {
                    eprintln!("Failed to fetch blockchain info: {:?}", e);
                }
            }
        }
    });

    // 使用 Actix Web 的运行时来启动 HTTP 服务器
    let api_database = Arc::clone(&database);
    spawn(async move {
        if let Err(e) = HttpServer::new(move || {
            App::new()
                .app_data(web::Data::new(Arc::clone(&api_database)))
                .route("/blockchain-info", web::get().to(api::handlers::get_blockchain_info))
        })
            .bind("127.0.0.1:8080")?
            .run()
            .await
        {
            eprintln!("Failed to start HTTP server: {:?}", e);
        }
        Ok::<(), std::io::Error>(())
    });

    loop {
        tokio::time::sleep(std::time::Duration::from_secs(60)).await;
    }
}


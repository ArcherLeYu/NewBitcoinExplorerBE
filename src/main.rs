// main.rs
mod config;
mod rpc_client;
mod db;
mod models;
mod utils;
mod errors;
mod retryable;
mod api;

use env_logger;
use actix_web::{HttpResponse, Responder};
use actix_web::{web, App, HttpServer}; // 导入其他需要的 Actix 组件
use crate::errors::AppError; // 导入 AppError
use tokio::time::{self, Duration};
use std::sync::Arc;
use actix_web::middleware::Logger;

#[actix_web::main]
async fn main() -> Result<(), AppError> {
    // 初始化日志记录器
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    // 创建配置对象
    let config = config::Config::new();

    // 创建数据库连接池
    let database = Arc::new(db::Database::new(&config.db_connection_string)?);

    // 使用 Tokio 并发来定时从 Bitcoin Core 获取区块链数据
    let database_clone = Arc::clone(&database);
    tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(600)); // 每600秒（10分钟）运行一次

        loop {
            interval.tick().await; // 每隔10分钟触发一次
            match rpc_client::get_blockchain_info(&config).await {
                Ok(blockchain_info) => {
                    // 检查区块哈希是否存在
                    let hash_exists = database_clone
                        .check_block_hash_exists(&blockchain_info.bestblockhash)
                        .unwrap_or_else(|e| {
                            eprintln!("Failed to check block hash existence: {:?}", e);
                            false // 返回默认值
                        });

                    if !hash_exists {
                        // 如果不存在，插入新的区块信息
                        println!("新区块链hash {:?}", blockchain_info.bestblockhash);
                        if let Err(e) = database_clone.insert_block_info(&blockchain_info) {
                            eprintln!("Failed to insert block info: {:?}", e);
                        }
                    } else {
                        // 如果存在，可以选择打印信息或执行其他逻辑
                    }
                }
                Err(e) => {
                    eprintln!("Failed to fetch blockchain info: {:?}", e);
                }
            }
        }
    });

    // 使用 Actix Web 的运行时来启动 HTTP 服务器
    println!("Starting HTTP server on http://127.0.0.1:8081...");
    HttpServer::new(move || {
        let app = App::new()
            .wrap(Logger::default()) // 添加日志记录中间件
            .app_data(web::Data::new(Arc::clone(&database)))
            .route("/", web::get().to(root_handler)) // 处理根路径请求
            .route("/blockchain-info", web::get().to(api::handlers::get_blockchain_info));

        app
    })
        .bind("127.0.0.1:8081")?
        .run()
        .await?;


    Ok(())
}
async fn root_handler() -> impl Responder {
    HttpResponse::Ok().body("Welcome to Bitcoin Explorer API. Use /blockchain-info to get the latest blockchain information.")
}

mod config;
mod rpc_client;
mod db;
mod models;
mod utils;
mod errors;
mod api;

use env_logger;
use actix_web::{web, App, HttpServer}; // 导入 Actix 组件
use crate::errors::AppError; // 导入 AppError
use tokio::time::{self, Duration};
use api::handlers::get_blockchain_info; // 导入 get_blockchain_info 函数
use actix_cors::Cors;

#[actix_web::main]
async fn main() -> Result<(), AppError> {
    // 初始化日志记录器
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    // 创建配置对象
    let config = config::Config::new();

    // 任务 1：定时从 Bitcoin Core 获取区块链数据
    // 直接创建数据库连接池实例并在每个循环中使用它
    let config_clone = config.clone();  // 克隆配置以供异步任务使用
    tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(600)); // 每600秒（10分钟）运行一次

        loop {
            interval.tick().await; // 每隔10分钟触发一次
            match rpc_client::get_blockchain_info(&config_clone).await {
                Ok(blockchain_info) => {
                    // 创建一个新的数据库连接池
                    let database = db::Database::new(&config_clone.db_connection_string).unwrap();

                    // 检查区块哈希是否存在
                    let hash_exists = database
                        .check_block_hash_exists(&blockchain_info.bestblockhash)
                        .unwrap_or_else(|e| {
                            eprintln!("Failed to check block hash existence: {:?}", e);
                            false // 返回默认值
                        });

                    if !hash_exists {
                        // 如果不存在，插入新的区块信息
                        println!("新区块链hash {:?}", blockchain_info.bestblockhash);
                        if let Err(e) = database.insert_block_info(&blockchain_info) {
                            eprintln!("Failed to insert block info: {:?}", e);
                        }
                    } else {
                        // 如果存在，可以选择打印信息或执行其他逻辑
                        println!("No update");
                    }
                }
                Err(e) => {
                    eprintln!("Failed to fetch blockchain info: {:?}", e);
                }
            }
        }
    });

    // 任务 2：提供 HTTP API 服务器，供前端获取最新的 blockchain 信息
    let database = db::Database::new(&config.db_connection_string)?;
    let database_data = web::Data::new(database);

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive()) // 添加 CORS 中间件，允许所有跨域请求
            .app_data(database_data.clone()) // 将包装好的数据库对象传递给应用程序
            .route("/blockchain-info", web::get().to(get_blockchain_info))
    })
        .bind("127.0.0.1:8081")?
        .run()
        .await?;

    Ok(())
}

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
use api::handlers::{get_blockchain_info, get_bitcoin_price, get_bitcoin_volume}; // 导入 get_blockchain_info 函数
use actix_cors::Cors;
use api::{price, volume};
use crate::api::handlers::{get_block_detail_by_height, get_blocks_summary, get_latest_10_prices};

#[actix_web::main]
async fn main() -> Result<(), AppError> {
    // 初始化日志记录器
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    // 创建配置对象
    let config = config::Config::new();
    let database = db::Database::new(&config.db_connection_string)?;

    // 克隆数据库连接以供异步任务使用
    let db_for_price = database.clone();
    let db_for_volume = database.clone();

    // 任务 1：定时从 Bitcoin Core 获取区块链数据
    // 直接创建数据库连接池实例并在每个循环中使用它
    let config_clone = config.clone();  // 克隆配置以供异步任务使用
    tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(600)); // 每600秒（10分钟）运行一次

        loop {
            interval.tick().await; //
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
                        if let Err(e) = database.insert_blockchain_info(&blockchain_info) {
                            eprintln!("Failed to insert block info: {:?}", e);
                        }

                        // Fetch the detailed block information using the new block hash
                        match rpc_client::get_block(&config_clone, &blockchain_info.bestblockhash).await {
                            Ok(block_info) => {
                                //println!("Fetched block info: {:?}", block_info);
                                // 尝试将获取的区块详细信息插入数据库
                                if let Err(e) = database.insert_blockinfo(&block_info) {
                                    eprintln!("Failed to insert block details: {:?}", e);
                                }
                            },
                            Err(e) => {
                                eprintln!("Failed to fetch block info: {:?}", e);
                            }
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

    // 任务 2：实时获取 Bitcoin 价格并保存到数据库
    tokio::spawn(async move {
        if let Err(e) = price::fetch_bitcoin_price(db_for_price).await {
            eprintln!("Error fetching bitcoin price: {:?}", e);
        }
    });

    // 任务 3：实时获取 Bitcoin 交易量并保存到数据库
    tokio::spawn(async move {
        if let Err(e) = volume::fetch_bitcoin_volume(db_for_volume).await {
            eprintln!("Error fetching bitcoin volume: {:?}", e);
        }
    });


    // 提供 HTTP API 服务器，供前端获取最新的 blockchain 信息
    let database = db::Database::new(&config.db_connection_string)?;
    let database_data = web::Data::new(database);

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive()) // 添加 CORS 中间件，允许所有跨域请求
            .app_data(database_data.clone()) // 将包装好的数据库对象传递给应用程序
            .route("/blockchain-info", web::get().to(get_blockchain_info))
            .route("/latest-price", web::get().to(get_bitcoin_price))
            .route("/latest-volume", web::get().to(get_bitcoin_volume))
            .route("/block-detail/{height}", web::get().to(get_block_detail_by_height))
            .route("/blocks-summary", web::get().to(get_blocks_summary))
            .route("/latest-10-prices", web::get().to(get_latest_10_prices)) // Register this route
    })
        .bind("0.0.0.0:8081")?
        .run()
        .await?;

    Ok(())
}

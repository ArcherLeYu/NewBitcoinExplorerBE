use actix_web::{get, web, HttpResponse, Responder};
use actix_web::web::Path;
use mysql::params;
use mysql::prelude::*; // 导入 Queryable trait
use crate::db::Database;
use crate::models::{BlockInfo, BlockchainInfo, PriceList};
use crate::models::BlockSummary;
use serde::Serialize;


pub async fn get_blockchain_info(database: web::Data<Database>) -> impl Responder {
    log::info!("Received request to get blockchain info");

    // 从数据库获取最新的区块链信息
    let mut conn = match database.get_conn() {
        Ok(conn) => {
            log::info!("Database connection established successfully");
            conn
        },
        Err(e) => {
            log::error!("Failed to get DB connection: {:?}", e);
            return HttpResponse::InternalServerError().body(format!("Failed to get DB connection: {:?}", e));
        }
    };

    log::info!("Executing query to fetch latest blockchain info");
    let query = r"SELECT
                    automatic_pruning, bestblockhash, blocks, chain, chainwork,
                    difficulty, headers, initialblockdownload, mediantime, prune_target_size,
                    pruned, pruneheight, size_on_disk, time, verificationprogress, warnings
                  FROM blockchaininfo
                  ORDER BY time DESC
                  LIMIT 1";

    // 执行查询并获取结果
    let result: Option<mysql::Row> = conn.exec_first(query, ()).unwrap_or_else(|e| {
        log::error!("Failed to execute query: {:?}", e);
        None
    });

    let blockchain_info = match result {
        Some(row) => BlockchainInfo {
            automatic_pruning: row.get("automatic_pruning").unwrap_or(false),
            bestblockhash: row.get("bestblockhash").unwrap_or_default(),
            blocks: row.get("blocks").unwrap_or(0),
            chain: row.get("chain").unwrap_or_default(),
            chainwork: row.get("chainwork").unwrap_or_default(),
            difficulty: row.get("difficulty").unwrap_or(0.0),
            headers: row.get("headers").unwrap_or(0),
            initialblockdownload: row.get("initialblockdownload").unwrap_or(false),
            mediantime: row.get("mediantime").unwrap_or(0),
            prune_target_size: row.get("prune_target_size").unwrap_or(0),
            pruned: row.get("pruned").unwrap_or(false),
            pruneheight: row.get("pruneheight").unwrap_or(0),
            size_on_disk: row.get("size_on_disk").unwrap_or(0),
            time: row.get("time").unwrap_or(0),
            verificationprogress: row.get("verificationprogress").unwrap_or(0.0),
            warnings: row.get("warnings").unwrap_or_default(),
        },
        None => {
            log::warn!("No blockchain info found in the database");
            return HttpResponse::NotFound().body("No blockchain info found");
        }
    };

    log::info!("Successfully fetched blockchain info: {:?}", blockchain_info);
    HttpResponse::Ok().json(blockchain_info)
}

// 获取最新的 Bitcoin 价格
pub async fn get_bitcoin_price(data: web::Data<Database>) -> HttpResponse {
    match data.get_latest_price() {
        Ok(price) => HttpResponse::Ok().json(price),
        Err(_) => HttpResponse::InternalServerError().json("Failed to fetch price"),
    }
}

// 获取最新的 Bitcoin 交易量
pub async fn get_bitcoin_volume(data: web::Data<Database>) -> HttpResponse {
    match data.get_latest_volume() {
        Ok(volume) => HttpResponse::Ok().json(volume),
        Err(_) => HttpResponse::InternalServerError().json("Failed to fetch volume"),
    }
}

// 获取指定bitcoin block的详细信息
pub async fn get_block_detail_by_height(database: web::Data<Database>, path: Path<u64>) -> impl Responder {
    let block_height = path.into_inner();  // Extract the height directly from path
    log::info!("Received request to get block detail for height: {}", block_height);

    // 从数据库获取指定的区块信息
    let mut conn = match database.get_conn() {
        Ok(conn) => {
            log::info!("Database connection established successfully");
            conn
        },
        Err(e) => {
            log::error!("Failed to get DB connection: {:?}", e);
            return HttpResponse::InternalServerError().body(format!("Failed to get DB connection: {:?}", e));
        }
    };

    let query = r"SELECT
                    hash, confirmations, size, height, version, version_hex, merkleroot,
                    time, mediantime, nonce, bits, difficulty, chainwork, n_tx,
                    previousblockhash, nextblockhash
                  FROM blockinfo
                  WHERE height = :height
                  LIMIT 1";

    // 执行查询并获取结果
    let result: Option<mysql::Row> = conn.exec_first(query, params! {
        "height" => block_height
    }).unwrap_or_else(|e| {
        log::error!("Failed to execute query: {:?}", e);
        None
    });

    let block_info = match result {
        Some(row) => BlockInfo {
            hash: row.get("hash").unwrap_or_default(),
            confirmations: row.get("confirmations").unwrap_or_default(),
            size: row.get("size").unwrap_or_default(),
            height: row.get("height").unwrap_or_default(),
            version: row.get("version").unwrap_or_default(),
            version_hex: row.get("version_hex").unwrap_or_default(),
            merkleroot: row.get("merkleroot").unwrap_or_default(),
            time: row.get("time").unwrap_or_default(),
            mediantime: row.get("mediantime").unwrap_or_default(),
            nonce: row.get("nonce").unwrap_or_default(),
            bits: row.get("bits").unwrap_or_default(),
            difficulty: row.get("difficulty").unwrap_or_default(),
            chainwork: row.get("chainwork").unwrap_or_default(),
            n_tx: row.get("n_tx").unwrap_or_default(),
            previousblockhash: row.get("previousblockhash").unwrap_or_default(),
            nextblockhash: row.get("nextblockhash").unwrap_or_default(),
        },
        None => {
            log::warn!("Block with height {} not found in the database", block_height);
            return HttpResponse::NotFound().body(format!("Block with height {} not found", block_height));
        }
    };

    log::info!("Successfully fetched block detail: {:?}", block_info);
    HttpResponse::Ok().json(block_info)
}

// 获取所有区块的摘要信息
pub async fn get_blocks_summary(database: web::Data<Database>) -> impl Responder {
    log::info!("Received request to get blocks summary");

    match database.get_blocks_summary() {
        Ok(blocks) => {
            // 日志输出每个区块的摘要信息
            for block in &blocks {
                log::info!("Block height: {}, hash: {}", block.height, block.hash);
            }
            HttpResponse::Ok().json(blocks) // 正确返回区块摘要信息
        },
        Err(e) => {
            log::error!("Failed to fetch blocks summary: {:?}", e);
            HttpResponse::InternalServerError().body(format!("Failed to fetch blocks summary: {:?}", e))
        }
    }
}


pub async fn get_latest_10_prices(db: web::Data<Database>) -> impl Responder {
    match db.get_latest_10_prices() {
        Ok(prices) => {
            HttpResponse::Ok().json(PriceList { prices }) // Respond with the list of prices.
        },
        Err(_) => {
            HttpResponse::InternalServerError().json("Failed to fetch the latest 10 prices")
        }
    }
}

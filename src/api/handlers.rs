use actix_web::{web, HttpResponse, Responder};
use mysql::prelude::*; // 导入 Queryable trait
use crate::db::Database;
use crate::models::BlockchainInfo;

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

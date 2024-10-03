use actix_web::{web, HttpResponse, Responder};
use mysql::prelude::*; // 导入 Queryable trait
use mysql::Row;
use crate::db::Database;
use crate::models::BlockchainInfo;

pub async fn get_blockchain_info(database: web::Data<Database>) -> impl Responder {
    let mut conn = match database.get_conn() {
        Ok(conn) => conn,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Failed to get DB connection: {:?}", e)),
    };

    let query = r"SELECT
                    automatic_pruning, bestblockhash, blocks, chain, chainwork,
                    difficulty, headers, initialblockdownload, mediantime, prune_target_size,
                    pruned, pruneheight, size_on_disk, time, verificationprogress, warnings
                  FROM Blocks
                  ORDER BY time DESC
                  LIMIT 1";

    // 执行查询并获取结果
    let result: Option<Row> = conn.exec_first(query, ()).unwrap();

    // 手动从 Row 中提取每个字段的值并构造 BlockchainInfo 结构体
    let blockchain_info = match result {
        Some(row) => {
            let automatic_pruning: bool = row.get("automatic_pruning").unwrap_or(false);
            let bestblockhash: String = row.get("bestblockhash").unwrap_or_default();
            let blocks: u64 = row.get("blocks").unwrap_or(0);
            let chain: String = row.get("chain").unwrap_or_default();
            let chainwork: String = row.get("chainwork").unwrap_or_default();
            let difficulty: f64 = row.get("difficulty").unwrap_or(0.0);
            let headers: u64 = row.get("headers").unwrap_or(0);
            let initialblockdownload: bool = row.get("initialblockdownload").unwrap_or(false);
            let mediantime: u64 = row.get("mediantime").unwrap_or(0);
            let prune_target_size: u64 = row.get("prune_target_size").unwrap_or(0);
            let pruned: bool = row.get("pruned").unwrap_or(false);
            let pruneheight: u64 = row.get("pruneheight").unwrap_or(0);
            let size_on_disk: u64 = row.get("size_on_disk").unwrap_or(0);
            let time: u64 = row.get("time").unwrap_or(0);
            let verificationprogress: f64 = row.get("verificationprogress").unwrap_or(0.0);
            let warnings: String = row.get("warnings").unwrap_or_default();

            BlockchainInfo {
                automatic_pruning,
                bestblockhash,
                blocks,
                chain,
                chainwork,
                difficulty,
                headers,
                initialblockdownload,
                mediantime,
                prune_target_size,
                pruned,
                pruneheight,
                size_on_disk,
                time,
                verificationprogress,
                warnings,
            }
        }
        None => return HttpResponse::NotFound().body("No blockchain info found"),
    };

    HttpResponse::Ok().json(blockchain_info)
}

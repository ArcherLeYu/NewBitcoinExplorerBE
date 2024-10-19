use actix_web::{get, web, HttpResponse, Responder};
use actix_web::web::Path;
use mysql::params;
use mysql::prelude::*; // 导入 Queryable trait
use crate::db::Database;
use crate::models::{PriceList};
use crate::models::BlockSummary;
use serde::Serialize;


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

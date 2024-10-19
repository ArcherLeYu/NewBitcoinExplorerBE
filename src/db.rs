// db.rs
use mysql::*;
use mysql::prelude::*;
use crate::errors::AppError;  // 确保引入了自定义错误类型
use crate::models::{BlockInfo, BlockSummary, BlockchainInfo}; // 确保引入了 BlockchainInfo
use tokio::time::sleep;
use std::time::Duration;


#[derive(Clone)]
pub struct Database {
    pool: Pool,
}

impl Database {
    // 修改函数返回类型为 Result<Database, AppError>
    pub fn new(url: &str) -> Result<Database, AppError> {
        // 使用 map_err 来转换错误类型
        let pool = Pool::new(url).map_err(AppError::Database)?;
        Ok(Database { pool })
    }

    // 添加一个函数来检查特定的 bestblockhash 是否已经存在
    pub fn check_block_hash_exists(&self, hash: &str) -> Result<bool, AppError> {
        let mut conn = self.pool.get_conn().map_err(AppError::Database)?;
        let exists: Option<u8> = conn.exec_first(
            "SELECT 1 FROM blockchaininfo WHERE bestblockhash = ?",
            (hash,)
        ).map_err(AppError::Database)?;
        Ok(exists.is_some())
    }

    pub fn insert_blockchain_info(&self, info: &BlockchainInfo) -> Result<(), AppError> {
        let mut conn = self.pool.get_conn().map_err(AppError::Database)?;
        let stmt = r"INSERT INTO blockchaininfo (automatic_pruning, bestblockhash, blocks, chain, chainwork, difficulty, headers, initialblockdownload, mediantime, prune_target_size, pruned, pruneheight, size_on_disk, time, verificationprogress, warnings) VALUES (:automatic_pruning, :bestblockhash, :blocks, :chain, :chainwork, :difficulty, :headers, :initialblockdownload, :mediantime, :prune_target_size, :pruned, :pruneheight, :size_on_disk, :time, :verificationprogress, :warnings)";
        let result = conn.exec_drop(
            stmt,
            params! {
            "automatic_pruning" => info.automatic_pruning,
            "bestblockhash" => &info.bestblockhash,
            "blocks" => info.blocks,
            "chain" => &info.chain,
            "chainwork" => &info.chainwork,
            "difficulty" => info.difficulty,
            "headers" => info.headers,
            "initialblockdownload" => info.initialblockdownload,
            "mediantime" => info.mediantime,
            "prune_target_size" => info.prune_target_size,
            "pruned" => info.pruned,
            "pruneheight" => info.pruneheight,
            "size_on_disk" => info.size_on_disk,
            "time" => info.time,
            "verificationprogress" => info.verificationprogress,
            "warnings" => &info.warnings,
        },
        );

        match result {
            Ok(_) => Ok(()),
            Err(e) => {
                eprintln!("Failed to insert block info: {}", e);
                Err(AppError::Database(e))
            }
        }
    }

    // 获取数据库连接的方法
    pub fn get_conn(&self) -> Result<PooledConn, mysql::Error> {
        match self.pool.get_conn() {
            Ok(conn) => {
                log::info!("Successfully established a database connection");
                Ok(conn)
            }
            Err(e) => {
                log::error!("Failed to establish a database connection: {:?}", e);
                Err(e)
            }
        }
    }

    pub fn insert_blockinfo(&self, block_info: &BlockInfo) -> Result<(), AppError> {
        let mut conn = self.pool.get_conn().map_err(AppError::Database)?;
        let stmt = r"INSERT INTO blockinfo (hash, confirmations, size, height, version, version_hex, merkleroot, time, mediantime, nonce, bits, difficulty, chainwork, n_tx, previousblockhash, nextblockhash) VALUES (:hash, :confirmations, :size, :height, :version, :version_hex, :merkleroot, :time, :mediantime, :nonce, :bits, :difficulty, :chainwork, :n_tx, :previousblockhash, :nextblockhash)";
        let result = conn.exec_drop(
            stmt,
            params! {
            "hash" => &block_info.hash,
            "confirmations" => block_info.confirmations,
            "size" => block_info.size,
            "height" => block_info.height,
            "version" => block_info.version,
            "version_hex" => &block_info.version_hex,  // Ensure that this handles Option correctly
            "merkleroot" => &block_info.merkleroot,
            "time" => block_info.time,
            "mediantime" => block_info.mediantime,
            "nonce" => block_info.nonce,
            "bits" => &block_info.bits,
            "difficulty" => block_info.difficulty,
            "chainwork" => &block_info.chainwork,
            "n_tx" => block_info.n_tx,
            "previousblockhash" => &block_info.previousblockhash,
            "nextblockhash" => &block_info.nextblockhash,
        },
        );

        match result {
            Ok(_) => Ok(()),
            Err(e) => {
                eprintln!("Failed to insert block info: {}", e);
                Err(AppError::Database(e))
            }
        }
    }

    // 添加价格到数据库
    pub fn insert_bitcoin_price(&self, price: f64) -> Result<(), AppError> {
        let mut conn = self.pool.get_conn().map_err(AppError::Database)?;
        let result = conn.exec_drop(
            "INSERT INTO bitcoin_prices (price, timestamp) VALUES (:price, CURRENT_TIMESTAMP)",
            params! {
                "price" => price,
            },
        );
        match result {
            Ok(_) => Ok(()),
            Err(e) => {
                eprintln!("Failed to insert bitcoin price: {}", e);
                Err(AppError::Database(e))
            }
        }
    }

    // 添加交易量到数据库
    pub fn insert_bitcoin_volume(&self, volume: f64) -> Result<(), AppError> {
        let mut conn = self.pool.get_conn().map_err(AppError::Database)?;
        let result = conn.exec_drop(
            "INSERT INTO bitcoin_volumes (volume, timestamp) VALUES (:volume, CURRENT_TIMESTAMP)",
            params! {
                "volume" => volume,
            },
        );
        match result {
            Ok(_) => Ok(()),
            Err(e) => {
                eprintln!("Failed to insert bitcoin volume: {}", e);
                Err(AppError::Database(e))
            }
        }
    }

    // 获取最新的价格信息
    pub fn get_latest_price(&self) -> Result<Option<f64>, AppError> {
        let mut conn = self.pool.get_conn()?;
        match conn.exec_first::<f64, _, _>(
            "SELECT price FROM bitcoin_prices ORDER BY id DESC LIMIT 1",
            (),
        ) {
            Ok(result) => {
                log::info!("Successfully fetched latest price: {:?}", result);
                Ok(result)
            },
            Err(e) => {
                log::error!("Failed to fetch latest price: {:?}", e);
                Err(AppError::Database(e))
            }
        }
    }

    // 获取最新的交易量信息
    pub fn get_latest_volume(&self) -> Result<Option<f64>, AppError> {
        let mut conn = self.pool.get_conn()?;
        match conn.exec_first::<f64, _, _>(
            "SELECT volume FROM bitcoin_volumes ORDER BY id DESC LIMIT 1",
            (),
        ) {
            Ok(result) => {
                log::info!("Successfully fetched latest volume: {:?}", result);
                Ok(result)
            },
            Err(e) => {
                log::error!("Failed to fetch latest volume: {:?}", e);
                Err(AppError::Database(e))
            }
        }
    }

    //获取数据库中block height和 hash
    pub fn get_blocks_summary(&self) -> Result<Vec<BlockSummary>, AppError> {
        let mut conn = self.pool.get_conn().map_err(AppError::Database)?;
        let blocks = conn.query_map(
            "SELECT height, hash FROM blockinfo ORDER BY height DESC",
            |(height, hash)| BlockSummary { height, hash },
        ).map_err(AppError::Database)?;
        Ok(blocks)
    }

}

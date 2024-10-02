// db.rs
use mysql::*;
use mysql::prelude::*;
use crate::errors::AppError;  // 确保引入了自定义错误类型
use crate::models::BlockchainInfo; // 确保引入了 BlockchainInfo


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
            "SELECT 1 FROM Blocks WHERE bestblockhash = ?",
            (hash,)
        ).map_err(AppError::Database)?;
        Ok(exists.is_some())
    }

    pub fn insert_block_info(&self, info: &BlockchainInfo) -> Result<(), AppError> {
        let mut conn = self.pool.get_conn().map_err(AppError::Database)?;
        let stmt = r"INSERT INTO Blocks (automatic_pruning, bestblockhash, blocks, chain, chainwork, difficulty, headers, initialblockdownload, mediantime, prune_target_size, pruned, pruneheight, size_on_disk, time, verificationprogress, warnings) VALUES (:automatic_pruning, :bestblockhash, :blocks, :chain, :chainwork, :difficulty, :headers, :initialblockdownload, :mediantime, :prune_target_size, :pruned, :pruneheight, :size_on_disk, :time, :verificationprogress, :warnings)";
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

}

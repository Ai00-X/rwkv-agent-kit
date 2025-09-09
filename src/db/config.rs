//! 数据库配置模块

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// 数据库类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub enum DatabaseType {
    /// SQLite数据库
    #[default]
    Sqlite,
    /// 内存数据库（用于测试）
    Memory,
}


/// 数据库配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    /// 数据库类型
    pub db_type: DatabaseType,
    /// 数据库文件路径（SQLite）
    pub db_path: PathBuf,
    /// 连接池大小
    pub pool_size: u32,
    /// 连接超时时间（秒）
    pub timeout: u64,
    /// 是否自动创建表
    pub auto_create_tables: bool,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            db_type: DatabaseType::Sqlite,
            db_path: PathBuf::from("data/agent_kit.db"),
            pool_size: 10,
            timeout: 30,
            auto_create_tables: true,
        }
    }
}

impl DatabaseConfig {
    /// 创建SQLite配置
    pub fn sqlite<P: Into<PathBuf>>(path: P) -> Self {
        Self {
            db_type: DatabaseType::Sqlite,
            db_path: path.into(),
            ..Default::default()
        }
    }

    /// 创建内存数据库配置（用于测试）
    pub fn memory() -> Self {
        Self {
            db_type: DatabaseType::Memory,
            db_path: PathBuf::from(":memory:"),
            ..Default::default()
        }
    }
}

//! 核心配置模块

use crate::agent::AgentConfig;
use crate::db::DatabaseConfig;
use crate::rwkv::ModelConfig;
use serde::{Deserialize, Serialize};

/// 主配置结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KitConfig {
    pub model: ModelConfig,
    pub agents: Vec<AgentConfig>,
    pub database: DatabaseConfig,
}

impl Default for KitConfig {
    fn default() -> Self {
        Self {
            model: ModelConfig::default(),
            agents: vec![AgentConfig::default()],
            database: DatabaseConfig::default(),
        }
    }
}

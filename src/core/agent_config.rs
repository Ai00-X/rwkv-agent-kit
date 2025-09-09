//! 智能体配置模块

use serde::{Deserialize, Serialize};

/// 智能体配置
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct AgentConfig {
    /// 是否启用记忆功能
    pub enable_memory: bool,
    /// 智能体描述（可选）
    pub description: Option<String>,
    /// 智能体版本（可选）
    pub version: Option<String>,
}


impl AgentConfig {
    /// 创建一个启用记忆的智能体配置
    pub fn with_memory() -> Self {
        Self {
            enable_memory: true,
            description: None,
            version: None,
        }
    }

    /// 创建一个不启用记忆的智能体配置
    pub fn without_memory() -> Self {
        Self {
            enable_memory: false,
            description: None,
            version: None,
        }
    }

    /// 设置智能体描述
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// 设置智能体版本
    pub fn with_version(mut self, version: String) -> Self {
        self.version = Some(version);
        self
    }

    /// 启用记忆功能
    pub fn enable_memory(mut self) -> Self {
        self.enable_memory = true;
        self
    }

    /// 禁用记忆功能
    pub fn disable_memory(mut self) -> Self {
        self.enable_memory = false;
        self
    }
}

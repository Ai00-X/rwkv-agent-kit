//! 智能体实例模块

use ai00_core::{sampler::Sampler, InputState};
use anyhow::Result;
use std::sync::Arc;
use tokio::sync::RwLock;

use super::{AgentConfig, Memory};
use crate::rwkv::{ModelConfig, StateManager};

/// 智能体实例
#[derive(Clone)]
pub struct Agent {
    pub config: AgentConfig,
    pub sampler: Arc<RwLock<dyn Sampler + Send + Sync>>,
    pub state: Arc<InputState>,
    pub memory: Memory,
}

impl Agent {
    /// 创建新的智能体实例
    pub fn new(config: AgentConfig, model_config: &ModelConfig) -> Result<Self> {
        let state_manager = StateManager::new(model_config.states.clone());

        let state = if let Some(state_name) = &config.state {
            // 查找指定名称的state
            state_manager
                .get_state_by_name(state_name)
                .unwrap_or_else(|| Arc::new(InputState::default()))
        } else {
            // 使用默认state
            state_manager.get_default_state()
        };

        // 根据配置创建采样器
        let mut nucleus_sampler = ai00_core::sampler::nucleus::NucleusSampler::default();
        nucleus_sampler.params.temperature = config.inference_params.temperature;
        nucleus_sampler.params.top_p = config.inference_params.top_p;
        nucleus_sampler.params.presence_penalty = config.inference_params.presence_penalty;
        nucleus_sampler.params.frequency_penalty = config.inference_params.frequency_penalty;

        Ok(Self {
            config,
            sampler: Arc::new(RwLock::new(nucleus_sampler)),
            state,
            memory: Memory::new(),
        })
    }

    /// 获取智能体的记忆管理器
    pub fn memory(&self) -> &Memory {
        &self.memory
    }
}

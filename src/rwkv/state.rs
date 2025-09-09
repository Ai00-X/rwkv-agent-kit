//! RWKV 状态管理模块

use ai00_core::{InputState, StateFile};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 状态配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateConfig {
    pub path: String,
    pub name: Option<String>,
    pub default: bool,
}

/// 状态管理器
pub struct StateManager {
    pub states: Vec<StateConfig>,
}

impl StateManager {
    /// 创建新的状态管理器
    pub fn new(states: Vec<StateConfig>) -> Self {
        Self { states }
    }

    /// 根据名称获取状态
    pub fn get_state_by_name(&self, name: &str) -> Option<Arc<InputState>> {
        self.states
            .iter()
            .find(|s| s.name.as_ref() == Some(&name.to_string()))
            .map(|s| {
                let state_file = StateFile {
                    name: s.name.clone().unwrap_or_default(),
                    id: Default::default(),
                    path: s.path.clone().into(),
                };
                Arc::new(InputState::File(state_file))
            })
    }

    /// 获取默认状态
    pub fn get_default_state(&self) -> Arc<InputState> {
        self.states
            .iter()
            .find(|s| s.default)
            .map(|s| {
                let state_file = StateFile {
                    name: s.name.clone().unwrap_or_default(),
                    id: Default::default(),
                    path: s.path.clone().into(),
                };
                Arc::new(InputState::File(state_file))
            })
            .unwrap_or_else(|| Arc::new(InputState::default()))
    }
}

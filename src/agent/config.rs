//! 智能体配置模块

use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 记忆配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryConfig {
    /// 是否启用记忆检索
    pub enabled: bool,
    /// 检索TopK数量
    pub top_k: usize,
    /// 时间衰减系数 (τ in exp(-Δt/τ))
    pub time_decay_factor: f32,
    /// 重要性权重 (importance boost factor)
    pub importance_weight: f32,
    /// 最大上下文长度（字符数）
    pub max_context_length: usize,

    // === 语义聚合配置 ===
    /// 语义片段创建的重要性阈值 (1-10)
    pub semantic_chunk_threshold: i32,
    /// 是否启用知识图谱更新（情节/程序记忆）
    pub enable_graph_updates: bool,
    /// 共现边权重映射策略：将 importance_score 转换为权重的除数
    pub cooccur_weight_divisor: f32,
    /// 边权重的最小值（避免过小的权重）
    pub min_edge_weight: f32,
    /// 边权重的最大值（防止权重过大）
    pub max_edge_weight: f32,
    /// 是否启用权重累加（而非直接返回已存在边的ID）
    pub enable_weight_accumulation: bool,
}

impl Default for MemoryConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            top_k: 5,
            time_decay_factor: 24.0,  // 24小时衰减
            importance_weight: 1.5,   // 重要性提升倍数
            max_context_length: 2000, // 最大上下文2000字符

            // 语义聚合默认配置
            semantic_chunk_threshold: 5,      // 重要性>=5才创建语义片段
            enable_graph_updates: false,      // 默认不启用知识图谱更新
            cooccur_weight_divisor: 10.0,     // importance_score / 10.0
            min_edge_weight: 0.1,             // 最小权重0.1
            max_edge_weight: 2.0,             // 最大权重2.0
            enable_weight_accumulation: true, // 默认启用权重累加
        }
    }
}

impl MemoryConfig {
    /// 创建启用记忆的配置
    pub fn enabled() -> Self {
        Self {
            enabled: true,
            ..Default::default()
        }
    }

    /// 创建禁用记忆的配置
    pub fn disabled() -> Self {
        Self {
            enabled: false,
            ..Default::default()
        }
    }

    /// 设置TopK
    pub fn with_top_k(mut self, top_k: usize) -> Self {
        self.top_k = top_k;
        self
    }

    /// 设置时间衰减系数
    pub fn with_time_decay_factor(mut self, factor: f32) -> Self {
        self.time_decay_factor = factor;
        self
    }

    /// 设置重要性权重
    pub fn with_importance_weight(mut self, weight: f32) -> Self {
        self.importance_weight = weight;
        self
    }

    /// 设置最大上下文长度
    pub fn with_max_context_length(mut self, length: usize) -> Self {
        self.max_context_length = length;
        self
    }

    /// 设置语义片段阈值
    pub fn with_semantic_chunk_threshold(mut self, threshold: i32) -> Self {
        self.semantic_chunk_threshold = threshold;
        self
    }

    /// 设置是否启用知识图谱更新
    pub fn with_graph_updates(mut self, enable: bool) -> Self {
        self.enable_graph_updates = enable;
        self
    }

    /// 设置共现边权重的除数
    pub fn with_cooccur_weight_divisor(mut self, divisor: f32) -> Self {
        self.cooccur_weight_divisor = divisor;
        self
    }

    /// 设置边权重范围
    pub fn with_edge_weight_range(mut self, min: f32, max: f32) -> Self {
        self.min_edge_weight = min;
        self.max_edge_weight = max;
        self
    }

    /// 设置是否启用权重累加
    pub fn with_weight_accumulation(mut self, enable: bool) -> Self {
        self.enable_weight_accumulation = enable;
        self
    }
}

/// 智能体配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    pub name: String,
    pub prompt_template: String,
    pub inference_params: InferenceParams,
    pub tools: Vec<String>,
    pub state: Option<String>,
    #[serde(skip)]
    pub prompt_builder: Option<Arc<super::prompt::PromptBuilderInstance>>,
    /// 是否保存对话到数据库，默认为true
    #[serde(default = "default_save_conversations")]
    pub save_conversations: bool,
    /// 记忆检索配置
    #[serde(default)]
    pub memory: MemoryConfig,
}

impl AgentConfig {
    /// 获取提示词构建器，如果没有设置则使用默认构建器
    pub fn get_prompt_builder(&self) -> Arc<super::prompt::PromptBuilderInstance> {
        self.prompt_builder.clone().unwrap_or_else(|| {
            Arc::new(super::prompt::PromptBuilderInstance::Default(
                super::prompt::DefaultPromptBuilder,
            ))
        })
    }

    /// 设置自定义提示词构建器
    pub fn with_prompt_builder(
        mut self,
        builder: Arc<super::prompt::PromptBuilderInstance>,
    ) -> Self {
        self.prompt_builder = Some(builder);
        self
    }

    /// 设置智能体名称
    pub fn with_name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = name.into();
        self
    }

    /// 设置是否保存对话
    pub fn with_save_conversations(mut self, save: bool) -> Self {
        self.save_conversations = save;
        self
    }
}

/// 默认保存对话设置
fn default_save_conversations() -> bool {
    false
}

impl Default for AgentConfig {
    fn default() -> Self {
        Self {
            name: "chat".to_string(),
            prompt_template: "你是一个有用的AI助手。".to_string(),
            inference_params: InferenceParams::default(),
            tools: vec![],
            state: None,
            prompt_builder: None,
            save_conversations: false,
            memory: MemoryConfig::default(),
        }
    }
}

/// 推理参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferenceParams {
    pub max_tokens: usize,
    pub temperature: f32,
    pub top_p: f32,
    pub presence_penalty: f32,
    pub frequency_penalty: f32,
    pub stop_sequences: Vec<String>,
}

impl Default for InferenceParams {
    fn default() -> Self {
        Self {
            max_tokens: 1024,
            temperature: 0.7,
            top_p: 0.9,
            presence_penalty: 0.0,
            frequency_penalty: 0.0,
            stop_sequences: vec!["\n\n".to_string()],
        }
    }
}

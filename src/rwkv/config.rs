//! RWKV 模型配置模块

use super::StateConfig;
use ai00_core::reload::{AdapterOption, BnfOption, Lora};
use ai00_core::ReloadRequest;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use web_rwkv::runtime::model::Quant;

/// 模型配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    /// 模型文件路径
    pub model_path: String,
    /// 分词器文件路径
    pub tokenizer_path: String,
    /// 精度设置 ("fp16" 或 "fp32")
    pub precision: String,

    /// 状态配置列表
    pub states: Vec<StateConfig>,

    // === ReloadRequest 的额外参数 ===
    /// LoRA 适配器列表
    pub lora: Option<Vec<LoraConfig>>,
    /// 量化层数
    pub quant: Option<usize>,
    /// 量化类型 ("none", "int8", "nf4", "sf4")
    pub quant_type: Option<String>,
    /// 并行处理的最大token数
    pub token_chunk_size: Option<usize>,
    /// GPU缓存的最大批次数
    pub max_batch: Option<usize>,
    /// 嵌入张量设备 ("cpu" 或 "gpu")
    pub embed_device: Option<String>,
    /// BNF选项
    pub bnf: Option<BnfConfig>,
    /// 适配器选项 ("auto", "economical", 或数字)
    pub adapter: Option<String>,
}

/// LoRA配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoraConfig {
    /// LoRA文件路径
    pub path: String,
    /// 混合因子
    pub alpha: Option<f32>,
}

/// BNF配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BnfConfig {
    /// 启用字节缓存
    pub enable_bytes_cache: Option<bool>,
    /// 起始非终结符
    pub start_nonterminal: Option<String>,
}

impl Default for ModelConfig {
    fn default() -> Self {
        Self {
            model_path: "./model/model.st".to_string(),
            tokenizer_path: "./model/tokenizer.json".to_string(),
            precision: "fp16".to_string(),

            states: vec![
                StateConfig {
                    path: "./model/chat.state".to_string(),
                    name: Some("chat".to_string()),
                    default: false,
                },
                StateConfig {
                    path: "./model/tool-call.state".to_string(),
                    name: Some("tool-call".to_string()),
                    default: false,
                },
            ],
            // ReloadRequest 参数的默认值
            lora: None,
            quant: None,
            quant_type: None,
            token_chunk_size: None,
            max_batch: None,
            embed_device: None,
            bnf: None,
            adapter: None,
        }
    }
}

impl TryFrom<ModelConfig> for ReloadRequest {
    type Error = anyhow::Error;

    fn try_from(value: ModelConfig) -> Result<Self, Self::Error> {
        let mut request = Self {
            model_path: value.model_path.into(),
            tokenizer_path: value.tokenizer_path.into(),
            precision: match value.precision.as_str() {
                "fp16" => ai00_core::reload::Precision::Fp16,
                "fp32" => ai00_core::reload::Precision::Fp32,
                _ => ai00_core::reload::Precision::Fp16,
            },
            // 处理新增的可选参数
            quant: value.quant.unwrap_or(0),
            quant_type: match value.quant_type.as_deref() {
                Some("int8") => Quant::Int8,
                Some("nf4") => Quant::NF4,
                Some("sf4") => Quant::NF4,
                _ => Quant::None,
            },
            token_chunk_size: value.token_chunk_size.unwrap_or(128),
            max_batch: value.max_batch.unwrap_or(8),

            bnf: BnfOption {
                enable_bytes_cache: value
                    .bnf
                    .as_ref()
                    .and_then(|b| b.enable_bytes_cache)
                    .unwrap_or(true),
                start_nonterminal: value
                    .bnf
                    .as_ref()
                    .and_then(|b| b.start_nonterminal.clone())
                    .unwrap_or_else(|| "start".to_string()),
            },
            adapter: match value.adapter.as_deref() {
                Some("economical") => AdapterOption::Economical,
                Some(num_str) => {
                    if let Ok(num) = num_str.parse::<usize>() {
                        AdapterOption::Manual(num)
                    } else {
                        AdapterOption::Auto
                    }
                }
                _ => AdapterOption::Auto,
            },
            ..Default::default()
        };

        // 添加LoRA配置
        if let Some(lora_configs) = value.lora {
            request.lora = lora_configs
                .into_iter()
                .map(|lora| Lora {
                    path: lora.path.into(),
                    alpha: lora.alpha.unwrap_or(1.0),
                })
                .collect();
        }

        // 添加state配置
        request.state = value
            .states
            .into_iter()
            .map(|state| ai00_core::reload::State {
                path: state.path.into(),
                name: state.name,
                default: state.default,
                id: ai00_core::StateId::new(),
            })
            .collect();

        Ok(request)
    }
}

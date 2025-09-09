---
title: agents
createTime: 2025/09/08 13:42:20
permalink: /article/bk7km33j/
---
# 智能体系统 API

智能体系统提供了创建和管理AI智能体的核心功能，包括智能体配置、记忆管理和推理参数设置。

## 核心组件

### Agent

智能体实例，包含配置、采样器、状态和记忆。

```rust
#[derive(Clone)]
pub struct Agent {
    pub config: AgentConfig,
    pub sampler: Arc<RwLock<dyn Sampler + Send + Sync>>,
    pub state: Arc<InputState>,
    pub memory: Memory,
}
```

#### 构造函数

##### `Agent::new`

创建新的智能体实例。

**参数：**
- `config: AgentConfig` - 智能体配置
- `model_config: &ModelConfig` - 模型配置

**返回值：**
- `Result<Agent>` - 智能体实例或错误

**异步：** 否

**示例：**
```rust
let config = AgentConfig::default();
let model_config = ModelConfig::default();
let agent = Agent::new(config, &model_config)?;
```

#### 方法

##### `Agent::memory`

获取智能体的记忆管理器引用。

**参数：** 无

**返回值：**
- `&Memory` - 记忆管理器引用

**异步：** 否

**示例：**
```rust
let memory_ref = agent.memory();
```

### Memory

智能体记忆管理器，负责存储和检索对话历史。

```rust
#[derive(Debug, Clone)]
pub struct Memory {
    /// 最近的对话历史 (user, assistant) 对
    history: Arc<RwLock<VecDeque<(String, String)>>>,
}
```

#### 构造函数

##### `Memory::new`

创建新的记忆管理器实例。

**参数：** 无

**返回值：**
- `Memory` - 记忆管理器实例

**异步：** 否

**示例：**
```rust
let memory = Memory::new();
```

#### 方法

##### `Memory::add_conversation`

添加一轮对话到历史记录。

**参数：**
- `user_input: String` - 用户输入
- `assistant_response: String` - 助手回复

**返回值：**
- `()` - 无返回值

**异步：** 是

**示例：**
```rust
memory.add_conversation(
    "你好".to_string(),
    "你好！有什么可以帮助你的吗？".to_string()
).await;
```

##### `Memory::filter_thought_content`

过滤掉思考内容标签之前的内容。

**参数：**
- `response: &str` - 原始回复内容

**返回值：**
- `String` - 过滤后的内容

**异步：** 否

**示例：**
```rust
let filtered = memory.filter_thought_content("<think>思考内容</think>实际回复");
```

##### `Memory::get_history`

获取格式化的历史对话记录。

**参数：** 无

**返回值：**
- `String` - 格式化的历史记录

**异步：** 是

**示例：**
```rust
let history = memory.get_history().await;
```

##### `Memory::clear_history`

清空历史记录。

**参数：** 无

**返回值：**
- `()` - 无返回值

**异步：** 是

**示例：**
```rust
memory.clear_history().await;
```

##### `Memory::history_count`

获取历史记录数量。

**参数：** 无

**返回值：**
- `usize` - 历史记录数量

**异步：** 是

**示例：**
```rust
let count = memory.history_count().await;
```

#### `get_history(&self) -> impl Future<Output = String>`

获取格式化的历史对话记录。

**返回值：**
- `String`: 格式化的对话历史

**注意：** 这是一个异步方法

#### `clear_history(&self) -> impl Future<Output = ()>`

清空历史记录。

**注意：** 这是一个异步方法

#### `history_count(&self) -> impl Future<Output = usize>`

获取历史记录数量。

**返回值：**
- `usize`: 历史记录条数

**注意：** 这是一个异步方法

### AgentConfig 结构体

智能体配置结构体，定义智能体的行为和参数。

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    pub name: String,
    pub prompt_template: String,
    pub inference_params: InferenceParams,
    pub tools: Vec<String>,
    pub state: Option<String>,
    #[serde(skip)]
    pub prompt_builder: Option<Arc<super::prompt::PromptBuilderInstance>>,
    /// 是否保存对话到数据库，默认为false
    #[serde(default = "default_save_conversations")]
    pub save_conversations: bool,
    /// 记忆检索配置
    #[serde(default)]
    pub memory: MemoryConfig,
}
```

#### 字段说明

- `name`: 智能体名称
- `prompt_template`: 提示词模板
- `inference_params`: 推理参数
- `tools`: 工具列表
- `state`: 可选的状态名称
- `prompt_builder`: 自定义提示词构建器
- `save_conversations`: 是否保存对话记录
- `memory`: 记忆配置

#### 构建方法

##### `default() -> Self`

创建默认配置。

**示例：**
```rust
let config = AgentConfig::default();
```

##### `with_name<S: Into<String>>(name: S) -> Self`

设置智能体名称。

**示例：**
```rust
let config = AgentConfig::default().with_name("my_agent");
```

##### `with_save_conversations(save: bool) -> Self`

设置是否保存对话。

**示例：**
```rust
let config = AgentConfig::default().with_save_conversations(true);
```

##### `get_prompt_builder(&self) -> Arc<PromptBuilderInstance>`

获取提示词构建器，如果没有设置则使用默认构建器。

**示例：**
```rust
let builder = config.get_prompt_builder();
```

##### `with_prompt_builder(builder: Arc<PromptBuilderInstance>) -> Self`

设置自定义提示词构建器。

**示例：**
```rust
let config = AgentConfig::default().with_prompt_builder(custom_builder);
```
```

### InferenceParams 结构体

推理参数配置，控制模型生成行为。

```rust
pub struct InferenceParams {
    pub max_tokens: usize,
    pub temperature: f32,
    pub top_p: f32,
    pub presence_penalty: f32,
    pub frequency_penalty: f32,
    pub stop_sequences: Vec<String>,
}
```

#### 字段说明

- `max_tokens`: 最大生成token数量
- `temperature`: 温度参数，控制随机性
- `top_p`: Top-p采样参数
- `presence_penalty`: 存在惩罚
- `frequency_penalty`: 频率惩罚
- `stop_sequences`: 停止序列

**默认值：**
- `max_tokens`: 1024
- `temperature`: 0.7
- `top_p`: 0.9
- `presence_penalty`: 0.0
- `frequency_penalty`: 0.0
- `stop_sequences`: `["\n\n"]`

### MemoryConfig 结构体

记忆配置结构体，控制智能体的记忆行为。

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryConfig {
    /// 是否启用记忆功能
    pub enabled: bool,
    /// 检索时返回的记忆数量
    pub top_k: usize,
    /// 时间衰减因子（小时）
    pub time_decay_factor: f32,
    /// 重要性权重倍数
    pub importance_weight: f32,
    /// 最大上下文长度
    pub max_context_length: usize,
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
```

#### 构建方法

```rust
impl MemoryConfig {
    /// 创建启用记忆的配置
    pub fn enabled() -> Self
    
    /// 创建禁用记忆的配置
    pub fn disabled() -> Self
    
    /// 设置TopK
    pub fn with_top_k(mut self, top_k: usize) -> Self
    
    /// 设置时间衰减系数
    pub fn with_time_decay_factor(mut self, factor: f32) -> Self
    
    /// 设置重要性权重
    pub fn with_importance_weight(mut self, weight: f32) -> Self
    
    /// 设置最大上下文长度
    pub fn with_max_context_length(mut self, length: usize) -> Self
    
    /// 设置语义片段阈值
    pub fn with_semantic_chunk_threshold(mut self, threshold: i32) -> Self
    
    /// 设置是否启用知识图谱更新
    pub fn with_graph_updates(mut self, enable: bool) -> Self
    
    /// 设置共现边权重的除数
    pub fn with_cooccur_weight_divisor(mut self, divisor: f32) -> Self
    
    /// 设置边权重范围
    pub fn with_edge_weight_range(mut self, min: f32, max: f32) -> Self
    
    /// 设置是否启用权重累加
    pub fn with_weight_accumulation(mut self, enable: bool) -> Self
}
```

## 使用示例

### 基本智能体配置

```rust
use rwkv_agent_kit::agent::{AgentConfig, InferenceParams, MemoryConfig};

// 创建基本配置
let config = AgentConfig::default()
    .with_name("chat_assistant")
    .with_save_conversations(false);
```

### 带记忆的智能体配置

```rust
// 创建带记忆的配置
let memory_config = MemoryConfig::enabled()
    .with_top_k(10)
    .with_max_context_length(2048)
    .with_semantic_chunk_threshold(6);

let config = AgentConfig {
    name: "memory_assistant".to_string(),
    prompt_template: "你是一个有记忆的AI助手。".to_string(),
    inference_params: InferenceParams::default(),
    tools: vec![],
    state: None,
    prompt_builder: None,
    save_conversations: true,
    memory: memory_config,
};
```

### 创建智能体实例

```rust
use rwkv_agent_kit::agent::{Agent, Memory};
use rwkv_agent_kit::rwkv::ModelConfig;

// 创建模型配置
let model_config = ModelConfig::default();

// 创建智能体
let agent = Agent::new(config, &model_config)?;

// 使用记忆功能
let memory = agent.memory();
memory.add_conversation(
    "你好".to_string(),
    "你好！有什么可以帮助你的吗？".to_string()
).await;

let history = memory.get_history().await;
println!("对话历史: {}", history);

// 清空历史记录
memory.clear_history().await;

// 获取历史记录数量
let count = memory.history_count().await;
println!("历史记录数量: {}", count);
```

## 注意事项

### 记忆配置

- 启用记忆功能会增加内存使用量和计算开销
- `top_k` 控制检索的相关记忆数量
- `time_decay_factor` 影响时间衰减的速度
- `semantic_chunk_threshold` 控制语义片段的创建阈值
- 知识图谱更新功能需要谨慎启用，会影响性能

### 推理参数

- `temperature` 控制生成的随机性，值越高越随机
- `top_p` 控制核采样的概率阈值
- `presence_penalty` 和 `frequency_penalty` 控制重复惩罚
- `max_tokens` 限制生成的最大令牌数
- `stop_sequences` 定义生成停止的标记

### 智能体实例

- 智能体创建需要有效的模型配置
- 记忆管理器自动过滤思考内容标签
- 对话历史最多保存5轮，超出会自动删除最旧的记录
- 所有记忆操作都是异步的，需要使用 `.await`
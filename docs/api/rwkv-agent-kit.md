---
title: rwkv-agent-kit
createTime: 2025/09/08 13:56:11
permalink: /article/7ai82xmd/
---
# RwkvAgentKit API

## 概述

`RwkvAgentKit` 是核心智能体工具包，提供了与 RWKV 模型交互的主要接口，支持多智能体管理、对话处理、记忆系统等功能。

## 结构体

### RwkvAgentKit

```rust
#[derive(Debug)]
pub struct RwkvAgentKit {
    /// 核心服务实例（全局单例）
    pub core_service: Arc<RwLock<CoreService>>,
    /// 数据库管理器
    pub database_manager: DatabaseManager,
    /// 工具注册表
    pub tools: SharedToolRegistry,
    /// 配置信息
    pub config: RwkvAgentKitConfig,
    /// 智能体配置映射 (智能体名称 -> 配置)
    pub agent_configs: HashMap<String, AgentConfig>,
    /// 错误处理器
    pub error_handler: Arc<ErrorHandler>,
}
```

核心智能体工具包，负责管理RWKV模型、数据库、智能体配置和对话处理。

### RwkvAgentKitConfig

```rust
#[derive(Debug, Clone, Default)]
pub struct RwkvAgentKitConfig {
    /// 模型配置
    pub model: ModelConfig,
    /// 数据库配置
    pub database: DatabaseConfig,
    /// 智能体配置列表
    pub agents: Vec<AgentConfig>,
}
```

RWKV Agent Kit的配置结构，包含模型、数据库和智能体的配置信息。

## 构造函数

### new (内部使用)

```rust
async fn new(config: RwkvAgentKitConfig) -> Result<Self>
```

创建新的RWKV Agent Kit实例。这是一个内部方法，建议使用`RwkvAgentKitBuilder`进行实例创建。

**参数：**
- `config`: RwkvAgentKitConfig - 工具包配置

**返回值：**
- `Result<Self>` - 创建的工具包实例或错误

**注意：** 此方法为内部实现，建议使用 `RwkvAgentKitBuilder` 进行实例创建。

## 主要方法

### register_agent

注册新的智能体到工具包中。

```rust
pub async fn register_agent(&mut self, agent_config: AgentConfig) -> Result<()>
```

**参数：**
- `agent_config`: AgentConfig - 智能体配置

**返回值：**
- `Result<()>` - 成功或错误

### chat

与指定智能体进行对话。

```rust
pub async fn chat(&mut self, agent_name: &str, user_input: &str) -> Result<String>
```

**参数：**
- `agent_name`: 智能体名称
- `user_input`: 用户输入

**返回值：**
- `Result<String>`: 智能体回复

### chat_with_nick

与chat智能体进行对话，并指定昵称。

```rust
pub async fn chat_with_nick(&mut self, user_input: &str, agent_nick: &str) -> Result<String>
```

**参数：**
- `user_input`: 用户输入
- `agent_nick`: 智能体昵称

**返回值：**
- `Result<String>`: 智能体回复

### chat_no_memory

与指定智能体进行对话（不存储记忆）。

```rust
pub async fn chat_no_memory(&mut self, agent_name: &str, user_input: &str) -> Result<String>
```

**参数：**
- `agent_name`: 智能体名称
- `user_input`: 用户输入

**返回值：**
- `Result<String>`: 智能体回复

### chat_no_memory_with_options

与指定智能体进行对话（不存储记忆，可选BNF模式和停止序列）。

```rust
pub async fn chat_no_memory_with_options(
    &mut self,
    agent_name: &str,
    user_input: &str,
    bnf_schema: Option<String>,
    stop_sequences: Option<Vec<String>>,
) -> Result<String>
```

**参数：**
- `agent_name`: 智能体名称
- `user_input`: 用户输入
- `bnf_schema`: BNF模式（可选）
- `stop_sequences`: 停止序列（可选）

**返回值：**
- `Result<String>`: 智能体回复



## 构建器模式

### RwkvAgentKitBuilder

```rust
pub struct RwkvAgentKitBuilder {
    config: RwkvAgentKitConfig,
}
```

便捷的构建器模式，用于创建RwkvAgentKit实例。

#### 构建器方法

##### `new() -> Self`

创建新的构建器实例。

##### `model_path<P: Into<String>>(mut self, path: P) -> Self`

设置模型路径。

##### `tokenizer_path<P: Into<String>>(mut self, path: P) -> Self`

设置分词器路径。

##### `precision<P: Into<String>>(mut self, precision: P) -> Self`

设置精度。

##### `quant(mut self, quant: usize) -> Self`

设置量化层数。

##### `quant_type<Q: Into<String>>(mut self, quant_type: Q) -> Self`

设置量化类型详细配置。

##### `token_chunk_size(mut self, size: usize) -> Self`

设置token块大小。

##### `max_batch(mut self, batch: usize) -> Self`

设置最大批次数。

##### `embed_device<D: Into<String>>(mut self, device: D) -> Self`

设置嵌入设备。

##### `bnf(mut self, bnf: BnfConfig) -> Self`

设置BNF配置。

##### `adapter<A: Into<String>>(mut self, adapter: A) -> Self`

设置适配器。

##### `database_config(mut self, database_config: DatabaseConfig) -> Self`

设置数据库配置。

##### `add_agent(mut self, agent_config: AgentConfig) -> Self`

添加智能体配置。

##### `with_default_agents(mut self) -> Self`

添加默认智能体（主要agent和对话总结智能体）。

##### `build(self) -> impl Future<Output = Result<RwkvAgentKit>>`

构建RWKV Agent Kit实例（异步方法）。

**返回值：**
- `Result<RwkvAgentKit>` - 创建的工具包实例或错误

## 使用示例

### 基本使用

```rust
use rwkv_agent_kit::{
    RwkvAgentKitBuilder,
    agent::AgentConfig,
    db::DatabaseConfig,
    rwkv::ModelConfig,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 使用构建器创建工具包
    let kit = RwkvAgentKitBuilder::new()
        .model_path("./model/model.st")
        .tokenizer_path("./model/tokenizer.json")
        .with_default_agents()
        .build()
        .await?;
    
    // 与智能体对话
    let response = kit.chat("chat", "你好，请介绍一下自己").await?;
    println!("AI回复: {}", response);
    
    Ok(())
}
```

### 自定义配置

```rust
use rwkv_agent_kit::{
    RwkvAgentKitBuilder,
    agent::{AgentConfig, InferenceParams, MemoryConfig},
    db::{DatabaseConfig, DatabaseType},
    rwkv::ModelConfig,
};
use std::path::PathBuf;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 创建自定义数据库配置
    let db_config = DatabaseConfig {
        db_type: DatabaseType::Sqlite,
        db_path: PathBuf::from("./data/agent.db"),
        pool_size: 10,
        timeout: 30,
        auto_create_tables: true,
    };
    
    // 创建自定义智能体配置
    let agent_config = AgentConfig::builder()
        .name("assistant")
        .prompt_template("你是一个有用的助手。用户: {user_input}\n助手:")
        .inference_params(InferenceParams {
            max_tokens: 512,
            temperature: 0.7,
            top_p: 0.9,
            presence_penalty: 0.0,
            frequency_penalty: 0.0,
        })
        .memory(MemoryConfig {
            enabled: true,
            top_k: 10,
            time_decay_factor: 0.95,
            importance_threshold: 0.1,
        })
        .save_conversations(true)
        .build();
    
    // 构建工具包
    let kit = RwkvAgentKitBuilder::new()
        .model_path("./model/model.st")
        .tokenizer_path("./model/tokenizer.json")
        .database_config(db_config)
        .add_agent(agent_config)
        .build()
        .await?;
    
    // 进行对话
    let response = kit.chat("assistant", "请解释什么是人工智能").await?;
    println!("回复: {}", response);
    
    Ok(())
}
```

### 带昵称的对话

```rust
use rwkv_agent_kit::RwkvAgentKitBuilder;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let kit = RwkvAgentKitBuilder::new()
        .model_path("./model/model.st")
        .tokenizer_path("./model/tokenizer.json")
        .with_default_agents()
        .build()
        .await?;
    
    // 使用自定义昵称进行对话
    let response = kit.chat_with_nick(
        "小智",
        "你好，小助手！"
    ).await?;
    println!("小智回复: {}", response);
    
    Ok(())
}
```

### 无记忆对话

```rust
use rwkv_agent_kit::RwkvAgentKitBuilder;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let kit = RwkvAgentKitBuilder::new()
        .model_path("./model/model.st")
        .tokenizer_path("./model/tokenizer.json")
        .with_default_agents()
        .build()
        .await?;
    
    // 进行不保存记忆的对话
    let response = kit.chat_no_memory(
        "chat", 
        "这是一个临时问题，不需要记住"
    ).await?;
    println!("临时回复: {}", response);
    
    // 带选项的无记忆对话
    let options = ChatOptions {
        bnf_schema: Some("{\"response\": string}".to_string()),
        stop_sequences: Some(vec!["}".to_string()]),
        ..Default::default()
    };
    let response_with_options = kit.chat_no_memory_with_options(
        "chat",
        "请用JSON格式回复",
        options
    ).await?;
    println!("结构化回复: {}", response_with_options);
    
    Ok(())
}
```
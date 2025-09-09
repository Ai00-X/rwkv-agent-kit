---
title: agent-kit
createTime: 2025/09/08 13:17:52
permalink: /article/m88cn4zo/
---
# RWKV Agent Kit 核心API

## 概述

RWKV Agent Kit 是一个基于RWKV的智能体开发框架，提供了完整的智能记忆系统和AI助手开发能力。

## 核心模块

### RwkvAgentKit

主要的智能体类，提供完整的AI助手功能。

```rust
use rwkv_agent_kit::prelude::*;

// 创建智能体配置
let config = RwkvAgentKitConfig {
    model_path: "path/to/model.pth".to_string(),
    database_url: "sqlite:memory.db".to_string(),
    max_memory_size: 1000,
    ..Default::default()
};

// 使用构建器模式创建智能体
let agent = RwkvAgentKitBuilder::new()
    .with_config(config)
    .with_memory_manager(memory_manager)
    .build()
    .await?;
```

#### 主要方法

##### `new(config: RwkvAgentKitConfig) -> Result<Self>`

创建新的智能体实例。

**参数:**
- `config`: 智能体配置

**返回值:**
- `Result<RwkvAgentKit>`: 成功时返回智能体实例

**示例:**
```rust
let config = RwkvAgentKitConfig::default();
let agent = RwkvAgentKit::new(config).await?;
```

##### `chat(&mut self, message: &str) -> Result<String>`

与智能体进行对话。

**参数:**
- `message`: 用户输入的消息

**返回值:**
- `Result<String>`: 智能体的回复

**示例:**
```rust
let response = agent.chat("你好，请介绍一下自己").await?;
println!("智能体回复: {}", response);
```

##### `add_memory(&mut self, content: &str, memory_type: MemoryType) -> Result<Uuid>`

添加新的记忆。

**参数:**
- `content`: 记忆内容
- `memory_type`: 记忆类型

**返回值:**
- `Result<Uuid>`: 记忆的唯一标识符

**示例:**
```rust
let memory_id = agent.add_memory(
    "用户喜欢喝咖啡", 
    MemoryType::UserPreference
).await?;
```

##### `search_memories(&self, query: &str) -> Result<Vec<Memory>>`

搜索相关记忆。

**参数:**
- `query`: 搜索查询

**返回值:**
- `Result<Vec<Memory>>`: 相关记忆列表

**示例:**
```rust
let memories = agent.search_memories("咖啡").await?;
for memory in memories {
    println!("记忆: {}", memory.content);
}
```

### RwkvAgentKitBuilder

智能体构建器，提供灵活的配置选项。

```rust
let agent = RwkvAgentKitBuilder::new()
    .with_model_path("path/to/model.pth")
    .with_database_url("sqlite:memory.db")
    .with_max_memory_size(2000)
    .with_learning_rate(0.01)
    .build()
    .await?;
```

#### 构建方法

##### `new() -> Self`

创建新的构建器实例。

##### `with_config(mut self, config: RwkvAgentKitConfig) -> Self`

设置完整配置。

##### `with_model_path(mut self, path: impl Into<String>) -> Self`

设置模型文件路径。

##### `with_database_url(mut self, url: impl Into<String>) -> Self`

设置数据库连接URL。

##### `with_max_memory_size(mut self, size: usize) -> Self`

设置最大记忆数量。

##### `with_learning_rate(mut self, rate: f32) -> Self`

设置学习率。

##### `build(self) -> Result<RwkvAgentKit>`

构建智能体实例。

### RwkvAgentKitConfig

智能体配置结构。

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RwkvAgentKitConfig {
    /// RWKV模型文件路径
    pub model_path: String,
    
    /// 数据库连接URL
    pub database_url: String,
    
    /// 最大记忆数量
    pub max_memory_size: usize,
    
    /// 学习率
    pub learning_rate: f32,
    
    /// 记忆衰减率
    pub memory_decay_rate: f32,
    
    /// 语义相似度阈值
    pub semantic_threshold: f32,
    
    /// 是否启用持续学习
    pub enable_continuous_learning: bool,
    
    /// 日志级别
    pub log_level: String,
}
```

#### 默认配置

```rust
impl Default for RwkvAgentKitConfig {
    fn default() -> Self {
        Self {
            model_path: "models/rwkv-4-world-3b-v1-20230803-ctx4096.pth".to_string(),
            database_url: "sqlite:memory.db".to_string(),
            max_memory_size: 1000,
            learning_rate: 0.01,
            memory_decay_rate: 0.001,
            semantic_threshold: 0.7,
            enable_continuous_learning: true,
            log_level: "info".to_string(),
        }
    }
}
```

## 错误处理

所有API方法都返回 `Result<T, MemoryError>` 类型，支持完整的错误处理。

```rust
use rwkv_agent_kit::error::MemoryError;

match agent.chat("hello").await {
    Ok(response) => println!("回复: {}", response),
    Err(MemoryError::DatabaseError(e)) => {
        eprintln!("数据库错误: {}", e);
    },
    Err(MemoryError::ModelError(e)) => {
        eprintln!("模型错误: {}", e);
    },
    Err(e) => {
        eprintln!("其他错误: {}", e);
    }
}
```

## 完整示例

```rust
use rwkv_agent_kit::prelude::*;
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    // 创建配置
    let config = RwkvAgentKitConfig {
        model_path: "models/rwkv-model.pth".to_string(),
        database_url: "sqlite:agent_memory.db".to_string(),
        max_memory_size: 2000,
        learning_rate: 0.01,
        ..Default::default()
    };
    
    // 创建智能体
    let mut agent = RwkvAgentKit::new(config).await?;
    
    // 添加一些初始记忆
    agent.add_memory(
        "用户是一名软件工程师，喜欢Rust编程", 
        MemoryType::UserProfile
    ).await?;
    
    agent.add_memory(
        "用户经常在晚上工作，喜欢喝咖啡", 
        MemoryType::UserPreference
    ).await?;
    
    // 开始对话
    let response = agent.chat("你好，我想了解一下Rust的异步编程").await?;
    println!("智能体: {}", response);
    
    // 搜索相关记忆
    let memories = agent.search_memories("编程").await?;
    println!("找到 {} 条相关记忆", memories.len());
    
    Ok(())
}
```

## 版本信息

```rust
use rwkv_agent_kit::{VERSION, NAME, DESCRIPTION, version_info};

println!("库名称: {}", NAME);
println!("版本: {}", VERSION);
println!("描述: {}", DESCRIPTION);
println!("完整信息: {}", version_info());
```

## 下一步

- [记忆管理API](./memory.md)
- [数据库API](./database.md)
- [工具系统API](./tools.md)
- [配置选项](../config/README.md)
- [使用示例](../examples/README.md)
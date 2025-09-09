# API 文档

欢迎查阅 RWKV Agent Kit 的完整 API 文档。本文档提供了所有公共接口的详细说明和使用示例。

## 📚 API 概览

RWKV Agent Kit 提供以下主要模块：

- **[RwkvAgentKit](./agent-kit.md)** - 核心智能代理类
- **[DatabaseManager](./database.md)** - 数据库管理器
- **[MemorySystem](./memory.md)** - 记忆系统
- **[Tools](./tools.md)** - 工具系统

## API 模块

### 核心模块
- [Agent Kit 核心](./agent-kit.md) - 主要的 Agent Kit 接口
- [记忆管理](./memory.md) - 记忆存储和检索系统
- [数据库操作](./database.md) - 向量图数据库接口
- [工具系统](./tools.md) - 工具注册和执行框架
- [核心类型定义](./types.md) - 系统中使用的核心数据类型和结构
- [配置管理](./config.md) - 配置系统和选项管理

## 🚀 快速导航

### 核心类

| 类名 | 描述 | 文档链接 |
|------|------|----------|
| `RwkvAgentKit` | 主要的智能代理类 | [详细文档](./agent-kit.md) |
| `DatabaseManager` | 数据库操作管理 | [详细文档](./database.md) |
| `MemorySystem` | 记忆存储和检索 | [详细文档](./memory.md) |
| `ToolRegistry` | 工具注册和管理 | [详细文档](./tools.md) |

### 主要特性

- **异步支持**: 所有 API 都支持异步操作
- **错误处理**: 完善的错误类型和处理机制
- **类型安全**: 充分利用 Rust 的类型系统
- **内存安全**: 零成本抽象和内存安全保证

## 🔧 基本使用模式

### 初始化

```rust
use rwkv_agent_kit::RwkvAgentKit;

// 从配置文件初始化
let agent = RwkvAgentKit::new("config.toml").await?;

// 或者使用构建器模式
let agent = RwkvAgentKit::builder()
    .model_path("model.pth")
    .database_url("sqlite://memory.db")
    .max_tokens(2048)
    .build()
    .await?;
```

### 错误处理

```rust
use rwkv_agent_kit::{RwkvAgentKit, AgentError};

match agent.chat("Hello").await {
    Ok(response) => println!("Response: {}", response),
    Err(AgentError::ModelError(e)) => eprintln!("Model error: {}", e),
    Err(AgentError::DatabaseError(e)) => eprintln!("Database error: {}", e),
    Err(e) => eprintln!("Other error: {}", e),
}
```

### 异步操作

```rust
use tokio::time::{timeout, Duration};

// 设置超时
let response = timeout(
    Duration::from_secs(30),
    agent.chat("复杂的问题")
).await??;

// 并发操作
let (response1, response2) = tokio::join!(
    agent.chat("问题1"),
    agent.chat("问题2")
);
```

## 📖 详细文档

### [RwkvAgentKit](./agent-kit.md)
核心智能代理类，提供对话、记忆管理、工具调用等功能。

### [DatabaseManager](./database.md)
数据库管理器，负责数据的持久化存储和检索。

### [MemorySystem](./memory.md)
记忆系统，实现长期记忆的存储、检索和管理。

### [Tools](./tools.md)
工具系统，支持自定义工具的注册和调用。

## 🎯 最佳实践

### 1. 资源管理

```rust
// 使用 Arc 在多个任务间共享代理
use std::sync::Arc;

let agent = Arc::new(RwkvAgentKit::new("config.toml").await?);
let agent_clone = agent.clone();

tokio::spawn(async move {
    let response = agent_clone.chat("Hello from task").await;
    // 处理响应
});
```

### 2. 错误处理

```rust
// 定义自定义错误类型
#[derive(Debug, thiserror::Error)]
enum MyAppError {
    #[error("Agent error: {0}")]
    Agent(#[from] rwkv_agent_kit::AgentError),
    #[error("Custom error: {0}")]
    Custom(String),
}
```

### 3. 配置管理

```rust
// 使用环境变量
use std::env;

let model_path = env::var("RWKV_MODEL_PATH")
    .unwrap_or_else(|_| "default_model.pth".to_string());

let agent = RwkvAgentKit::builder()
    .model_path(&model_path)
    .build()
    .await?;
```

## 🔍 类型定义

### 主要类型

```rust
// 配置类型
pub struct AgentConfig {
    pub model_path: String,
    pub database_url: String,
    pub max_tokens: usize,
    pub temperature: f32,
}

// 消息类型
pub struct Message {
    pub id: String,
    pub content: String,
    pub role: MessageRole,
    pub timestamp: DateTime<Utc>,
}

// 记忆类型
pub struct Memory {
    pub id: String,
    pub content: String,
    pub category: String,
    pub embedding: Vec<f32>,
    pub created_at: DateTime<Utc>,
}
```

## 📝 版本兼容性

| 版本 | Rust 版本要求 | 主要变更 |
|------|---------------|----------|
| 0.1.x | >= 1.70.0 | 初始版本 |
| 0.2.x | >= 1.72.0 | 添加工具系统 |
| 1.0.x | >= 1.75.0 | 稳定 API |

---

**需要帮助？** 查看 [示例代码](/examples/) 或访问 [GitHub Issues](https://github.com/Ai00-X/rwkv-agent-kit/issues)。
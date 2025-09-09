# 快速开始

欢迎使用 RWKV Agent Kit！本指南将帮助您快速上手这个强大的智能体开发框架。

## 📋 系统要求

- Rust 1.70.0 或更高版本
- 支持的操作系统：Linux、macOS、Windows
- 内存：建议 4GB 以上
- 存储：至少 1GB 可用空间

## 🚀 安装

### 方法一：使用 Cargo

```bash
cargo add rwkv-agent-kit
```

### 方法二：在 Cargo.toml 中添加依赖

```toml
[dependencies]
rwkv-agent-kit = "0.1.1"
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
```

## 🎯 第一个智能体

让我们创建一个简单的智能体来体验 RWKV Agent Kit 的基本功能：

```rust
use rwkv_agent_kit::{
    RwkvAgentKit, 
    config::AgentConfig,
    memory::MemoryConfig,
    tools::ToolRegistry
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. 创建配置
    let config = AgentConfig::builder()
        .model_path("path/to/your/rwkv/model.pth")
        .memory(MemoryConfig::default())
        .build()?;
    
    // 2. 初始化智能体
    let mut agent = RwkvAgentKit::new(config).await?;
    
    // 3. 开始对话
    let response = agent.chat("你好，请介绍一下你自己").await?;
    println!("AI: {}", response);
    
    // 4. 继续对话（智能体会记住之前的内容）
    let response = agent.chat("我刚才问了你什么问题？").await?;
    println!("AI: {}", response);
    
    Ok(())
}
```

## 🧠 记忆系统

RWKV Agent Kit 的核心特色是其强大的记忆系统。智能体可以：

- **短期记忆**：保持对话上下文
- **长期记忆**：将重要信息存储到向量数据库
- **语义检索**：根据相关性检索历史记忆

### 配置记忆系统

```rust
use rwkv_agent_kit::memory::{MemoryConfig, VectorStore};

let memory_config = MemoryConfig::builder()
    .vector_store(VectorStore::Sqlite {
        path: "./memory.db".to_string(),
    })
    .max_context_length(2048)
    .memory_threshold(0.7)
    .build();
```

## 🔧 工具系统

智能体可以使用各种工具来扩展其能力：

```rust
use rwkv_agent_kit::tools::{Tool, ToolRegistry};
use serde_json::Value;

// 定义一个简单的计算器工具
#[derive(Clone)]
struct Calculator;

#[async_trait::async_trait]
impl Tool for Calculator {
    fn name(&self) -> &str {
        "calculator"
    }
    
    fn description(&self) -> &str {
        "执行基本的数学计算"
    }
    
    async fn execute(&self, input: Value) -> Result<Value, Box<dyn std::error::Error>> {
        // 实现计算逻辑
        Ok(serde_json::json!({"result": 42}))
    }
}

// 注册工具
let mut registry = ToolRegistry::new();
registry.register(Box::new(Calculator));

let config = AgentConfig::builder()
    .model_path("path/to/model.pth")
    .tools(registry)
    .build()?;
```

## 📊 配置选项

### 基本配置

```rust
let config = AgentConfig::builder()
    .model_path("./models/rwkv-4-world-7b-v1-20230626-ctx4096.pth")
    .device("wgpu")  // 或 "cpu"
    .precision("fp16")  // 或 "fp32"
    .max_tokens(2048)
    .temperature(0.7)
    .top_p(0.9)
    .build()?;
```

### 高级配置

```rust
let config = AgentConfig::builder()
    .model_path("./models/rwkv-model.pth")
    .memory(MemoryConfig::builder()
        .vector_store(VectorStore::Sqlite {
            path: "./agent_memory.db".to_string(),
        })
        .embedding_model("sentence-transformers/all-MiniLM-L6-v2")
        .max_context_length(4096)
        .memory_threshold(0.8)
        .build())
    .logging(LoggingConfig::builder()
        .level("info")
        .file("./logs/agent.log")
        .build())
    .build()?;
```

## 🎨 实际应用示例

### 智能客服助手

```rust
use rwkv_agent_kit::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = AgentConfig::builder()
        .model_path("./models/customer-service-model.pth")
        .system_prompt("你是一个专业的客服助手，请友好、耐心地回答用户问题。")
        .memory(MemoryConfig::default())
        .build()?;
    
    let mut agent = RwkvAgentKit::new(config).await?;
    
    // 模拟客服对话
    let queries = vec![
        "我想了解你们的退货政策",
        "我的订单什么时候能到？",
        "刚才我问的退货问题，具体流程是什么？"
    ];
    
    for query in queries {
        println!("用户: {}", query);
        let response = agent.chat(query).await?;
        println!("客服: {}\n", response);
    }
    
    Ok(())
}
```

## 🔍 调试和监控

### 启用日志

```rust
use tracing_subscriber;

// 在 main 函数开始处添加
tracing_subscriber::fmt::init();
```

### 性能监控

```rust
// 获取智能体状态
let stats = agent.get_stats().await?;
println!("处理的消息数: {}", stats.messages_processed);
println!("平均响应时间: {}ms", stats.avg_response_time);
println!("内存使用量: {}MB", stats.memory_usage);
```

## 📚 下一步

- 查看 [API 文档](/api/) 了解详细的接口说明
- 浏览 [示例代码](/examples/) 学习更多用法
- 阅读 [配置指南](/config/) 优化性能
- 参与 [社区讨论](https://github.com/Ai00-X/rwkv-agent-kit/discussions)

## ❓ 常见问题

### Q: 如何选择合适的模型？
A: 建议根据您的硬件配置选择：
- 8GB+ 显存：7B 模型
- 4-8GB 显存：3B 模型  
- 4GB 以下：1.5B 模型

### Q: 智能体的记忆会占用多少存储空间？
A: 这取决于对话量和记忆配置。通常每1000条对话约占用10-50MB存储空间。

### Q: 如何提高响应速度？
A: 可以通过以下方式优化：
- 使用 GPU 加速
- 调整 `max_tokens` 参数
- 启用状态缓存
- 使用较小的模型

---

🎉 **恭喜！您已经掌握了 RWKV Agent Kit 的基本用法。开始构建您的智能体应用吧！**
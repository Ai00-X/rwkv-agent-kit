# RWKV Agent Kit: 基于RWKV的智能体开发工具包

中文版 | [English](README_EN.md)

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)
[![Repository](https://img.shields.io/badge/repository-GitHub-green.svg)](https://github.com/Ai00-X/rwkv-agent-kit)

一个高性能的智能体开发工具包，基于RWKV大语言模型，提供多智能体系统、记忆管理、向量检索和工具集成等功能。专为构建智能对话系统和AI应用而设计。

## 🌟 核心特性

### 🤖 RWKV LLM 集成
- **高性能推理**: 基于web-rwkv的RWKV架构推理引擎
- **全局单例管理**: 自动管理模型加载和资源优化
- **异步处理**: 支持高并发的异步推理请求
- **状态管理**: 智能的模型状态缓存和恢复机制
- **State Tuning**: 支持RWKV独特的状态调优实现低成本智能体定制

### 👥 多智能体系统
- **预定义智能体**: 开箱即用的对话、总结、路由等智能体
- **智能体工厂**: 统一的智能体创建和管理接口
- **自定义智能体**: 支持自定义提示词构建器和配置
- **工作流聚合**: 多智能体协作和结果聚合
- **请求驱动**: 通过不同类型的LLM请求实现多智能体功能

### 🧠 智能记忆系统
- **会话管理**: 多会话支持，自动会话状态管理
- **记忆事件**: 结构化存储用户对话和AI回复
- **语义片段**: 长期记忆的语义聚合和总结
- **图谱存储**: 实体关系图谱，支持复杂知识表示
- **个人画像**: 用户偏好和特征的持久化存储

### 🔍 向量检索能力
- **Model2Vec集成**: 高效的多语言向量嵌入
- **语义搜索**: 基于向量相似度的智能检索
- **混合检索**: 结合关键词和语义的多模式检索
- **性能优化**: 查询优化器和缓存机制

### 🛠️ 工具生态系统
- **工具注册表**: 动态工具注册和管理
- **共享工具**: 跨智能体的工具共享机制
- **扩展接口**: 易于集成外部工具和API
- **错误处理**: 完善的错误处理和恢复机制

### 🗄️ 数据存储
- **SQLite数据库**: 轻量级但功能强大的本地存储
- **统一接口**: 抽象的数据库操作接口
- **数据迁移**: 自动数据库结构迁移
- **性能监控**: 数据库性能分析和优化

## 🚀 快速开始

### 环境要求

- Rust 1.70+
- 支持的操作系统：Windows、Linux、macOS
- 内存：建议4GB以上
- 存储：模型文件需要额外空间

### 安装

#### 方式一：在现有 Rust 项目中引入库

1. **从 crates.io 引入（推荐）**

在你的 `Cargo.toml` 文件中添加依赖：

```toml
[dependencies]
rwkv-agent-kit = "0.1.1"
tokio = { version = "1.0", features = ["full"] }
anyhow = "1.0"
```

2. **从 Git 仓库引入（开发版本）**

```toml
[dependencies]
rwkv-agent-kit = { git = "https://github.com/Ai00-X/rwkv-agent-kit.git" }
tokio = { version = "1.0", features = ["full"] }
anyhow = "1.0"
```

3. **可选特性**

```toml
[dependencies]
rwkv-agent-kit = { version = "0.1.1", features = ["embedding", "tools"] }
```

#### 方式二：克隆项目进行开发

1. **克隆项目**
```bash
git clone https://github.com/Ai00-X/rwkv-agent-kit.git
cd rwkv-agent-kit
```

2. **准备模型文件**
将RWKV模型文件放置在 `model/` 目录下：
- `model.st` - RWKV模型文件
- `tokenizer.json` - 分词器文件
- `chat.state` - 对话状态文件（可选）
- `tool-call.state` - 工具调用状态文件（可选）

3. **构建项目**
```bash
cargo build --release
```

### 运行示例

#### 运行 chat_demo 示例

项目提供了一个完整的对话示例，展示多轮对话和记忆功能：

```bash
# 确保已准备好模型文件
cargo run --example chat_demo
```

**预期输出：**
```
[DEBUG] 已清理数据库中的历史记录

=== 第1轮对话 ===
[USER] 你好，我的名字叫小明，是一名程序员。
[ASSISTANT] 你好小明！很高兴认识你这位程序员...

=== 第2轮对话 ===
[USER] 我最喜欢的编程语言是Rust，你知道为什么吗？
[ASSISTANT] 根据我们之前的对话，我知道你是小明...
```

**可能遇到的问题：**

1. **模型文件缺失**
   ```
   Error: 无法加载模型文件
   ```
   解决方案：确保 `model/` 目录下有必要的模型文件

2. **内存不足**
   ```
   Error: 内存分配失败
   ```
   解决方案：确保系统有足够内存（建议4GB+）

3. **权限问题**
   ```
   Error: 无法创建数据库文件
   ```
   解决方案：确保对 `data/` 目录有写入权限

#### 其他示例

查看 `examples/` 目录了解更多使用示例：

```bash
# 查看所有可用示例
ls examples/

# 运行特定示例
cargo run --example <示例名称>
```

### 基本使用

#### 简单使用（推荐）

```rust
use rwkv_agent_kit::RwkvAgentKitBuilder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 使用默认配置快速启动
    let mut kit = RwkvAgentKitBuilder::new()
        .with_default_agents()
        .build()
        .await?;
    
    // 开始对话
    let response = kit.chat("chat", "你好，请介绍一下Rust编程语言").await?;
    println!("AI回复: {}", response);
    
    Ok(())
}
```

#### 自定义配置

```rust
use rwkv_agent_kit::{
    RwkvAgentKit, RwkvAgentKitConfig,
    agents::{AgentFactory, AgentType},
    rwkv::config::ModelConfig,
    db::DatabaseConfig,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. 创建配置
    let config = RwkvAgentKitConfig {
        model: ModelConfig {
            model_path: "./model/model.st".to_string(),
            tokenizer_path: "./model/tokenizer.json".to_string(),
            state_path: Some("./model/chat.state".to_string()),
            ..Default::default()
        },
        database: DatabaseConfig::default(),
        agents: vec![
            AgentFactory::create_config(AgentType::Chat),
            AgentFactory::create_config(AgentType::ConversationSummarizer),
        ],
    };
    
    // 2. 启动 RWKV Agent Kit
    let mut kit = RwkvAgentKit::from_config(config).await?;
    
    // 3. 与智能体对话
    let response = kit.chat("chat", "你好，请介绍一下Rust编程语言").await?;
    println!("AI回复: {}", response);
    
    // 4. 查看对话历史
    let session_id = kit.database_manager.get_active_session().await?.unwrap();
    let events = kit.database_manager.list_memory_events(session_id).await?;
    println!("对话历史共 {} 条记录", events.len());
    
    Ok(())
}
```

## 📖 技术架构

### 核心模块

```
RWKV Agent Kit
├── core/                    # 核心功能模块
│   ├── rwkv_singleton.rs   # RWKV模型单例管理
│   ├── service.rs          # 核心服务
│   ├── tools.rs            # 工具注册表
│   └── error_handler.rs    # 错误处理
├── agent/                   # 智能体框架
│   ├── agent.rs            # 智能体基础实现
│   ├── config.rs           # 智能体配置
│   ├── memory.rs           # 记忆配置
│   └── prompt.rs           # 提示词构建器
├── agents/                  # 预定义智能体
│   ├── chat.rs             # 对话智能体
│   ├── conversation_summarizer.rs  # 对话总结智能体
│   ├── router.rs           # 路由智能体
│   └── workflow_aggregator.rs      # 工作流聚合智能体
├── db/                      # 数据库模块
│   ├── manager.rs          # 数据库管理器
│   ├── sqlite.rs           # SQLite实现
│   ├── embedding.rs        # 嵌入服务
│   └── query_optimizer.rs  # 查询优化器
└── rwkv/                    # RWKV模块
    ├── config.rs           # RWKV配置
    └── state.rs            # 状态管理
```

### 数据流架构

```
用户输入 → 智能体路由 → RWKV推理 → 记忆存储 → 响应生成
    ↓                                      ↑
工具调用 → 工具执行 → 结果处理 → 记忆更新 → 上下文增强
```

## 🔧 配置说明

### 模型配置

```rust
use rwkv_agent_kit::rwkv::config::ModelConfig;

let model_config = ModelConfig {
    model_path: "./model/model.st".to_string(),
    tokenizer_path: "./model/tokenizer.json".to_string(),
    state_path: Some("./model/chat.state".to_string()),
    max_tokens: 2048,
    temperature: 0.7,
    top_p: 0.9,
    presence_penalty: 0.0,
    frequency_penalty: 0.0,
};
```

### 数据库配置

```rust
use rwkv_agent_kit::db::DatabaseConfig;

let db_config = DatabaseConfig {
    database_url: "./data/agent_kit.db".to_string(),
    max_connections: 10,
    enable_wal: true,
    cache_size: 1000,
};
```

### 智能体配置

```rust
use rwkv_agent_kit::agent::{AgentConfig, MemoryConfig};
use rwkv_agent_kit::agents::chat::ChatPromptBuilder;
use std::sync::Arc;

let agent_config = AgentConfig {
    name: "assistant".to_string(),
    description: "专业助手智能体".to_string(),
    prompt_builder: Some(Arc::new(ChatPromptBuilder::with_nick("小助手"))),
    memory: MemoryConfig {
        enabled: true,
        max_context_length: 4000,
        semantic_chunk_threshold: 7,
    },
    save_conversations: true,
    bnf_schema: None,
    stop_sequences: None,
};
```

## 🛠️ 高级功能

### 自定义工具

```rust
use rwkv_agent_kit::core::tools::{Tool, ToolRegistry};
use async_trait::async_trait;

struct WeatherTool;

#[async_trait]
impl Tool for WeatherTool {
    fn name(&self) -> &str { "get_weather" }
    
    fn description(&self) -> &str { "获取天气信息" }
    
    async fn execute(&self, params: serde_json::Value) -> anyhow::Result<String> {
        // 实现天气查询逻辑
        Ok("今天天气晴朗，温度25°C".to_string())
    }
}

// 注册工具
let tools = kit.tools.clone();
{
    let mut registry = tools.write().await;
    registry.register(Box::new(WeatherTool));
}
```

### 记忆检索

```rust
// 语义搜索
let embedding_service = rwkv_agent_kit::db::embedding::get_global_embedding_service()?;
let query_embedding = embedding_service.lock().await
    .encode_single("机器学习相关内容").await?;

// 检索相关记忆事件
let similar_events = kit.database_manager
    .search_similar_events(&query_embedding, 10).await?;

for event in similar_events {
    println!("相关记忆: {} (相似度: {:.3})", 
        event.text, event.similarity_score);
}
```

### 会话管理

```rust
// 创建新会话
let session_id = kit.database_manager.create_session("用户123").await?;

// 切换会话
kit.database_manager.set_active_session(session_id).await?;

// 获取会话历史
let events = kit.database_manager.list_memory_events(session_id).await?;
```

## 📚 API 文档

### RwkvAgentKit 主要方法

- `from_config(config: RwkvAgentKitConfig) -> Result<Self>` - 从配置创建实例
- `register_agent(config: AgentConfig) -> Result<()>` - 注册新智能体
- `chat(agent_name: &str, message: &str) -> Result<String>` - 与智能体对话
- `list_agents() -> Vec<String>` - 列出所有可用智能体

### 数据库管理器方法

- `create_session(user_id: &str) -> Result<Uuid>` - 创建新会话
- `list_memory_events(session_id: Uuid) -> Result<Vec<MemoryEvent>>` - 获取记忆事件
- `search_similar_events(embedding: &[f32], limit: usize) -> Result<Vec<MemoryEvent>>` - 语义搜索
- `create_semantic_chunk(content: &str, summary: &str) -> Result<Uuid>` - 创建语义片段

## 🔍 示例项目

查看 `examples/` 目录中的示例：

- `chat_demo.rs` - 基本对话示例
- `embedding_demo.rs` - 向量嵌入示例
- `model_download_test.rs` - 模型下载测试

运行示例：
```bash
cargo run --example chat_demo
```

## 🤝 开发指南

### 添加新智能体

1. 在 `src/agents/` 目录下创建新文件
2. 实现智能体逻辑和提示词构建器
3. 在 `AgentType` 枚举中添加新类型
4. 在 `AgentFactory` 中添加创建逻辑

### 扩展工具系统

1. 实现 `Tool` trait
2. 在工具注册表中注册
3. 在智能体中使用工具

### 自定义记忆类型

1. 扩展数据库模式
2. 实现新的存储和检索逻辑
3. 更新智能体配置

## 📄 许可证

本项目采用 MIT 或 Apache-2.0 双重许可证。详见 [LICENSE](LICENSE) 文件。

## 🙏 致谢

- [RWKV](https://github.com/BlinkDL/RWKV-LM) - 革命性的语言模型架构
- [web-rwkv](https://github.com/cryscan/web-rwkv) - 高性能RWKV推理引擎
- [Model2Vec](https://github.com/MinishLab/model2vec) - 高效的文本嵌入模型
- [SQLite](https://www.sqlite.org/) - 可靠的嵌入式数据库
- [Tokio](https://tokio.rs/) - 异步运行时框架

感谢所有为开源社区做出贡献的开发者们！

## 📞 联系我们

- GitHub Issues: [提交问题](https://github.com/Ai00-X/rwkv-agent-kit/issues)
- 讨论区: [GitHub Discussions](https://github.com/Ai00-X/rwkv-agent-kit/discussions)

---

**RWKV Agent Kit** - 让AI智能体拥有真正的记忆和思考能力 🚀
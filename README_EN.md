# RWKV Agent Kit: RWKV-based Agent Development Toolkit

[中文版](README.md) | English

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)
[![Repository](https://img.shields.io/badge/repository-GitHub-green.svg)](https://github.com/Ai00-X/rwkv-agent-kit)

A high-performance agent development toolkit based on RWKV large language model, providing multi-agent systems, memory management, vector retrieval, and tool integration capabilities. Designed specifically for building intelligent dialogue systems and AI applications.

## 🌟 Core Features

### 🤖 RWKV LLM Integration
- **High-Performance Inference**: RWKV architecture inference engine based on web-rwkv
- **Global Singleton Management**: Automatic model loading and resource optimization management
- **Asynchronous Processing**: Support for high-concurrency asynchronous inference requests
- **State Management**: Intelligent model state caching and recovery mechanisms
- **State Tuning**: Support for RWKV's unique state tuning for low-cost agent customization

### 👥 Multi-Agent System
- **Predefined Agents**: Out-of-the-box dialogue, summarization, routing, and other agents
- **Agent Factory**: Unified agent creation and management interface
- **Custom Agents**: Support for custom prompt builders and configurations
- **Workflow Aggregation**: Multi-agent collaboration and result aggregation
- **Request-Driven**: Multi-agent functionality through different types of LLM requests

### 🧠 Intelligent Memory System
- **Session Management**: Multi-session support with automatic session state management
- **Memory Events**: Structured storage of user conversations and AI responses
- **Semantic Chunks**: Semantic aggregation and summarization of long-term memory
- **Graph Storage**: Entity relationship graphs supporting complex knowledge representation
- **Personal Profiles**: Persistent storage of user preferences and characteristics

### 🔍 Vector Retrieval Capabilities
- **Model2Vec Integration**: Efficient multilingual vector embeddings
- **Semantic Search**: Intelligent retrieval based on vector similarity
- **Hybrid Retrieval**: Multi-modal retrieval combining keywords and semantics
- **Performance Optimization**: Query optimizer and caching mechanisms

### 🛠️ Tool Ecosystem
- **Tool Registry**: Dynamic tool registration and management
- **Shared Tools**: Cross-agent tool sharing mechanisms
- **Extension Interface**: Easy integration of external tools and APIs
- **Error Handling**: Comprehensive error handling and recovery mechanisms

### 🗄️ Data Storage
- **SQLite Database**: Lightweight yet powerful local storage
- **Unified Interface**: Abstract database operation interface
- **Data Migration**: Automatic database structure migration
- **Performance Monitoring**: Database performance analysis and optimization

## 🚀 Quick Start

### Requirements

- Rust 1.70+
- Supported Operating Systems: Windows, Linux, macOS
- Memory: 4GB+ recommended
- Storage: Additional space required for model files

### Installation

1. **Clone the Project**
```bash
git clone https://github.com/Ai00-X/rwkv-agent-kit.git
cd rwkv-agent-kit
```

2. **Prepare Model Files**
Place RWKV model files in the `model/` directory:
- `model.st` - RWKV model file
- `tokenizer.json` - Tokenizer file
- `chat.state` - Chat state file (optional)
- `tool-call.state` - Tool call state file (optional)

3. **Build the Project**
```bash
cargo build --release
```

## 📦 Using the Library in Rust Projects

### Method 1: Add as Dependency

1. **From crates.io (Recommended)**

```toml
[dependencies]
rwkv-agent-kit = "0.1.1"
tokio = { version = "1.0", features = ["full"] }
anyhow = "1.0"
```

2. **From Git Repository (Development Version)**

```toml
[dependencies]
rwkv-agent-kit = { git = "https://github.com/Ai00-X/rwkv-agent-kit.git" }
tokio = { version = "1.0", features = ["full"] }
anyhow = "1.0"
```

3. **Optional Features**

```toml
[dependencies]
rwkv-agent-kit = { version = "0.1.1", features = ["embedding", "tools"] }
```

### Method 2: Clone for Development

Follow the installation steps above for cloning and building the project.

## 🎯 Running Examples

### Running chat_demo Example

The project provides a complete conversation example demonstrating multi-turn dialogue and memory functionality:

```bash
# Ensure model files are prepared
cargo run --example chat_demo
```

**Expected Output:**
```
[DEBUG] Cleared historical records in database

=== Round 1 Conversation ===
[USER] Hello, my name is Xiao Ming, and I'm a programmer.
[ASSISTANT] Hello Xiao Ming! Nice to meet you, a programmer...

=== Round 2 Conversation ===
[USER] My favorite programming language is Rust, do you know why?
[ASSISTANT] Based on our previous conversation, I know you are Xiao Ming...
```

**Common Issues:**

1. **Missing Model Files**
   ```
   Error: Unable to load model file
   ```
   Solution: Ensure necessary model files are in the `model/` directory

2. **Insufficient Memory**
   ```
   Error: Memory allocation failed
   ```
   Solution: Ensure sufficient system memory (4GB+ recommended)

3. **Permission Issues**
   ```
   Error: Unable to create database file
   ```
   Solution: Ensure write permissions for the `data/` directory

### Other Examples

Check the `examples/` directory for more usage examples:

```bash
# View all available examples
ls examples/

# Run specific example
cargo run --example <example_name>
```

### Basic Usage

#### Simple Usage (Recommended)

```rust
use rwkv_agent_kit::RwkvAgentKitBuilder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Quick start with default configuration
    let mut kit = RwkvAgentKitBuilder::new()
        .with_default_agents()
        .build()
        .await?;
    
    // Start conversation
    let response = kit.chat("chat", "Hello, please introduce the Rust programming language").await?;
    println!("AI Response: {}", response);
    
    Ok(())
}
```

#### Custom Configuration

```rust
use rwkv_agent_kit::{
    RwkvAgentKit, RwkvAgentKitConfig,
    agents::{AgentFactory, AgentType},
    rwkv::config::ModelConfig,
    db::DatabaseConfig,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. Create configuration
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
    
    // 2. Start RWKV Agent Kit
    let mut kit = RwkvAgentKit::from_config(config).await?;
    
    // 3. Chat with agent
    let response = kit.chat("chat", "Hello, please introduce the Rust programming language").await?;
    println!("AI Response: {}", response);
    
    // 4. View conversation history
    let session_id = kit.database_manager.get_active_session().await?.unwrap();
    let events = kit.database_manager.list_memory_events(session_id).await?;
    println!("Conversation history has {} records", events.len());
    
    Ok(())
}
```

## 📖 Technical Architecture

### Core Modules

```
RWKV Agent Kit
├── core/                    # Core functionality modules
│   ├── rwkv_singleton.rs   # RWKV model singleton management
│   ├── service.rs          # Core services
│   ├── tools.rs            # Tool registry
│   └── error_handler.rs    # Error handling
├── agent/                   # Agent framework
│   ├── agent.rs            # Agent base implementation
│   ├── config.rs           # Agent configuration
│   ├── memory.rs           # Memory configuration
│   └── prompt.rs           # Prompt builder
├── agents/                  # Predefined agents
│   ├── chat.rs             # Chat agent
│   ├── conversation_summarizer.rs  # Conversation summarizer agent
│   ├── router.rs           # Router agent
│   └── workflow_aggregator.rs      # Workflow aggregator agent
├── db/                      # Database modules
│   ├── manager.rs          # Database manager
│   ├── sqlite.rs           # SQLite implementation
│   ├── embedding.rs        # Embedding service
│   └── query_optimizer.rs  # Query optimizer
└── rwkv/                    # RWKV modules
    ├── config.rs           # RWKV configuration
    └── state.rs            # State management
```

### Data Flow Architecture

```
User Input → Agent Routing → RWKV Inference → Memory Storage → Response Generation
    ↓                                      ↑
Tool Call → Tool Execution → Result Processing → Memory Update → Context Enhancement
```

## 🔧 Configuration

### Model Configuration

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

### Database Configuration

```rust
use rwkv_agent_kit::db::DatabaseConfig;

let db_config = DatabaseConfig {
    database_url: "./data/agent_kit.db".to_string(),
    max_connections: 10,
    enable_wal: true,
    cache_size: 1000,
};
```

### Agent Configuration

```rust
use rwkv_agent_kit::agent::{AgentConfig, MemoryConfig};
use rwkv_agent_kit::agents::chat::ChatPromptBuilder;
use std::sync::Arc;

let agent_config = AgentConfig {
    name: "assistant".to_string(),
    description: "Professional assistant agent".to_string(),
    prompt_builder: Some(Arc::new(ChatPromptBuilder::with_nick("Assistant"))),
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

## 🛠️ Advanced Features

### Custom Tools

```rust
use rwkv_agent_kit::core::tools::{Tool, ToolRegistry};
use async_trait::async_trait;

struct WeatherTool;

#[async_trait]
impl Tool for WeatherTool {
    fn name(&self) -> &str { "get_weather" }
    
    fn description(&self) -> &str { "Get weather information" }
    
    async fn execute(&self, params: serde_json::Value) -> anyhow::Result<String> {
        // Implement weather query logic
        Ok("Today is sunny, temperature 25°C".to_string())
    }
}

// Register tool
let tools = kit.tools.clone();
{
    let mut registry = tools.write().await;
    registry.register(Box::new(WeatherTool));
}
```

### Memory Retrieval

```rust
// Semantic search
let embedding_service = rwkv_agent_kit::db::embedding::get_global_embedding_service()?;
let query_embedding = embedding_service.lock().await
    .encode_single("machine learning related content").await?;

// Retrieve related memory events
let similar_events = kit.database_manager
    .search_similar_events(&query_embedding, 10).await?;

for event in similar_events {
    println!("Related memory: {} (similarity: {:.3})", 
        event.text, event.similarity_score);
}
```

### Session Management

```rust
// Create new session
let session_id = kit.database_manager.create_session("user123").await?;

// Switch session
kit.database_manager.set_active_session(session_id).await?;

// Get session history
let events = kit.database_manager.list_memory_events(session_id).await?;
```

## 📚 API Documentation

### RwkvAgentKit Main Methods

- `from_config(config: RwkvAgentKitConfig) -> Result<Self>` - Create instance from configuration
- `register_agent(config: AgentConfig) -> Result<()>` - Register new agent
- `chat(agent_name: &str, message: &str) -> Result<String>` - Chat with agent
- `list_agents() -> Vec<String>` - List all available agents

### Database Manager Methods

- `create_session(user_id: &str) -> Result<Uuid>` - Create new session
- `list_memory_events(session_id: Uuid) -> Result<Vec<MemoryEvent>>` - Get memory events
- `search_similar_events(embedding: &[f32], limit: usize) -> Result<Vec<MemoryEvent>>` - Semantic search
- `create_semantic_chunk(content: &str, summary: &str) -> Result<Uuid>` - Create semantic chunk

## 🔍 Example Projects

Check out examples in the `examples/` directory:

- `chat_demo.rs` - Basic chat example
- `embedding_demo.rs` - Vector embedding example
- `model_download_test.rs` - Model download test

Run examples:
```bash
cargo run --example chat_demo
```

## 🤝 Development Guide

### Adding New Agents

1. Create a new file in the `src/agents/` directory
2. Implement agent logic and prompt builder
3. Add new type to `AgentType` enum
4. Add creation logic to `AgentFactory`

### Extending Tool System

1. Implement the `Tool` trait
2. Register in the tool registry
3. Use tools in agents

### Custom Memory Types

1. Extend database schema
2. Implement new storage and retrieval logic
3. Update agent configuration

## 📄 License

This project is licensed under MIT OR Apache-2.0 dual license. See [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [RWKV](https://github.com/BlinkDL/RWKV-LM) - Revolutionary language model architecture
- [web-rwkv](https://github.com/cryscan/web-rwkv) - High-performance RWKV inference engine
- [Model2Vec](https://github.com/MinishLab/model2vec) - Efficient text embedding model
- [SQLite](https://www.sqlite.org/) - Reliable embedded database
- [Tokio](https://tokio.rs/) - Asynchronous runtime framework

Thanks to all developers who contribute to the open source community!

## 📞 Contact Us

- GitHub Issues: [Submit Issues](https://github.com/Ai00-X/rwkv-agent-kit/issues)
- Discussions: [GitHub Discussions](https://github.com/Ai00-X/rwkv-agent-kit/discussions)

---

**RWKV Agent Kit** - Empowering AI agents with true memory and thinking capabilities 🚀
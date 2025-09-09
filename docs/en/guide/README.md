# Getting Started

Welcome to RWKV Agent Kit! This guide will help you quickly get started with this powerful agent development framework.

## 📋 System Requirements

- Rust 1.70.0 or higher
- Supported OS: Linux, macOS, Windows
- Memory: 4GB+ recommended
- Storage: At least 1GB available space

## 🚀 Installation

### Method 1: Using Cargo

```bash
cargo add rwkv-agent-kit
```

### Method 2: Add to Cargo.toml

```toml
[dependencies]
rwkv-agent-kit = "0.1.1"
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
```

## 🎯 Your First Agent

Let's create a simple agent to experience the basic functionality of RWKV Agent Kit:

```rust
use rwkv_agent_kit::{
    RwkvAgentKit, 
    config::AgentConfig,
    memory::MemoryConfig,
    tools::ToolRegistry
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Create configuration
    let config = AgentConfig::builder()
        .model_path("path/to/your/rwkv/model.pth")
        .memory(MemoryConfig::default())
        .build()?;
    
    // 2. Initialize agent
    let mut agent = RwkvAgentKit::new(config).await?;
    
    // 3. Start conversation
    let response = agent.chat("Hello, please introduce yourself").await?;
    println!("AI: {}", response);
    
    // 4. Continue conversation (agent remembers previous content)
    let response = agent.chat("What did I just ask you?").await?;
    println!("AI: {}", response);
    
    Ok(())
}
```

## 🧠 Memory System

The core feature of RWKV Agent Kit is its powerful memory system. The agent can:

- **Short-term Memory**: Maintain conversation context
- **Long-term Memory**: Store important information to vector database
- **Semantic Retrieval**: Retrieve historical memories based on relevance

### Configuring Memory System

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

## 🔧 Tool System

Agents can use various tools to extend their capabilities:

```rust
use rwkv_agent_kit::tools::{Tool, ToolRegistry};
use serde_json::Value;

// Define a simple calculator tool
#[derive(Clone)]
struct Calculator;

#[async_trait::async_trait]
impl Tool for Calculator {
    fn name(&self) -> &str {
        "calculator"
    }
    
    fn description(&self) -> &str {
        "Perform basic mathematical calculations"
    }
    
    async fn execute(&self, input: Value) -> Result<Value, Box<dyn std::error::Error>> {
        // Implement calculation logic
        Ok(serde_json::json!({"result": 42}))
    }
}

// Register tool
let mut registry = ToolRegistry::new();
registry.register(Box::new(Calculator));

let config = AgentConfig::builder()
    .model_path("path/to/model.pth")
    .tools(registry)
    .build()?;
```

## 📊 Configuration Options

### Basic Configuration

```rust
let config = AgentConfig::builder()
    .model_path("./models/rwkv-4-world-7b-v1-20230626-ctx4096.pth")
    .device("wgpu")  // or "cpu"
    .precision("fp16")  // or "fp32"
    .max_tokens(2048)
    .temperature(0.7)
    .top_p(0.9)
    .build()?;
```

### Advanced Configuration

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

## 🎨 Practical Examples

### Intelligent Customer Service Assistant

```rust
use rwkv_agent_kit::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = AgentConfig::builder()
        .model_path("./models/customer-service-model.pth")
        .system_prompt("You are a professional customer service assistant. Please answer user questions in a friendly and patient manner.")
        .memory(MemoryConfig::default())
        .build()?;
    
    let mut agent = RwkvAgentKit::new(config).await?;
    
    // Simulate customer service conversation
    let queries = vec![
        "I want to know about your return policy",
        "When will my order arrive?",
        "About the return question I just asked, what's the specific process?"
    ];
    
    for query in queries {
        println!("Customer: {}", query);
        let response = agent.chat(query).await?;
        println!("Support: {}\n", response);
    }
    
    Ok(())
}
```

## 🔍 Debugging and Monitoring

### Enable Logging

```rust
use tracing_subscriber;

// Add at the beginning of main function
tracing_subscriber::fmt::init();
```

### Performance Monitoring

```rust
// Get agent statistics
let stats = agent.get_stats().await?;
println!("Messages processed: {}", stats.messages_processed);
println!("Average response time: {}ms", stats.avg_response_time);
println!("Memory usage: {}MB", stats.memory_usage);
```

## 📚 Next Steps

- Check [API Reference](/en/api/) for detailed interface documentation
- Browse [Examples](/en/examples/) to learn more usage patterns
- Read [Configuration Guide](/en/config/) to optimize performance
- Join [Community Discussions](https://github.com/Ai00-X/rwkv-agent-kit/discussions)

## ❓ FAQ

### Q: How to choose the right model?
A: We recommend choosing based on your hardware configuration:
- 8GB+ VRAM: 7B model
- 4-8GB VRAM: 3B model  
- Below 4GB: 1.5B model

### Q: How much storage will agent memory use?
A: This depends on conversation volume and memory configuration. Typically, every 1000 conversations use about 10-50MB of storage.

### Q: How to improve response speed?
A: You can optimize through:
- Use GPU acceleration
- Adjust `max_tokens` parameter
- Enable state caching
- Use smaller models

---

🎉 **Congratulations! You've mastered the basics of RWKV Agent Kit. Start building your agent applications!**
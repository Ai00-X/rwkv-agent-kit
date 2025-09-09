# API Documentation

Welcome to the complete API documentation for RWKV Agent Kit. This documentation provides detailed descriptions and usage examples for all public interfaces.

## 📚 API Overview

RWKV Agent Kit provides the following main modules:

- **[RwkvAgentKit](./agent-kit.md)** - Core intelligent agent class
- **[DatabaseManager](./database.md)** - Database manager
- **[MemorySystem](./memory.md)** - Memory system
- **[Tools](./tools.md)** - Tool system

## 🚀 Quick Navigation

### Core Classes

| Class | Description | Documentation |
|-------|-------------|---------------|
| `RwkvAgentKit` | Main intelligent agent class | [Detailed docs](./agent-kit.md) |
| `DatabaseManager` | Database operation management | [Detailed docs](./database.md) |
| `MemorySystem` | Memory storage and retrieval | [Detailed docs](./memory.md) |
| `ToolRegistry` | Tool registration and management | [Detailed docs](./tools.md) |

### Key Features

- **Async Support**: All APIs support asynchronous operations
- **Error Handling**: Comprehensive error types and handling mechanisms
- **Type Safety**: Full utilization of Rust's type system
- **Memory Safety**: Zero-cost abstractions and memory safety guarantees

## 🔧 Basic Usage Patterns

### Initialization

```rust
use rwkv_agent_kit::RwkvAgentKit;

// Initialize from config file
let agent = RwkvAgentKit::new("config.toml").await?;

// Or use builder pattern
let agent = RwkvAgentKit::builder()
    .model_path("model.pth")
    .database_url("sqlite://memory.db")
    .max_tokens(2048)
    .build()
    .await?;
```

### Error Handling

```rust
use rwkv_agent_kit::{RwkvAgentKit, AgentError};

match agent.chat("Hello").await {
    Ok(response) => println!("Response: {}", response),
    Err(AgentError::ModelError(e)) => eprintln!("Model error: {}", e),
    Err(AgentError::DatabaseError(e)) => eprintln!("Database error: {}", e),
    Err(e) => eprintln!("Other error: {}", e),
}
```

### Async Operations

```rust
use tokio::time::{timeout, Duration};

// Set timeout
let response = timeout(
    Duration::from_secs(30),
    agent.chat("Complex question")
).await??;

// Concurrent operations
let (response1, response2) = tokio::join!(
    agent.chat("Question 1"),
    agent.chat("Question 2")
);
```

## 📖 Detailed Documentation

### [RwkvAgentKit](./agent-kit.md)
Core intelligent agent class providing conversation, memory management, tool calling, and other functionalities.

### [DatabaseManager](./database.md)
Database manager responsible for persistent storage and retrieval of data.

### [MemorySystem](./memory.md)
Memory system implementing storage, retrieval, and management of long-term memory.

### [Tools](./tools.md)
Tool system supporting registration and invocation of custom tools.

## 🎯 Best Practices

### 1. Resource Management

```rust
// Use Arc to share agent across multiple tasks
use std::sync::Arc;

let agent = Arc::new(RwkvAgentKit::new("config.toml").await?);
let agent_clone = agent.clone();

tokio::spawn(async move {
    let response = agent_clone.chat("Hello from task").await;
    // Handle response
});
```

### 2. Error Handling

```rust
// Define custom error types
#[derive(Debug, thiserror::Error)]
enum MyAppError {
    #[error("Agent error: {0}")]
    Agent(#[from] rwkv_agent_kit::AgentError),
    #[error("Custom error: {0}")]
    Custom(String),
}
```

### 3. Configuration Management

```rust
// Use environment variables
use std::env;

let model_path = env::var("RWKV_MODEL_PATH")
    .unwrap_or_else(|_| "default_model.pth".to_string());

let agent = RwkvAgentKit::builder()
    .model_path(&model_path)
    .build()
    .await?;
```

## 🔍 Type Definitions

### Main Types

```rust
// Configuration types
pub struct AgentConfig {
    pub model_path: String,
    pub database_url: String,
    pub max_tokens: usize,
    pub temperature: f32,
}

// Message types
pub struct Message {
    pub id: String,
    pub content: String,
    pub role: MessageRole,
    pub timestamp: DateTime<Utc>,
}

// Memory types
pub struct Memory {
    pub id: String,
    pub content: String,
    pub category: String,
    pub embedding: Vec<f32>,
    pub created_at: DateTime<Utc>,
}
```

## 📝 Version Compatibility

| Version | Rust Version Required | Major Changes |
|---------|----------------------|---------------|
| 0.1.x | >= 1.70.0 | Initial release |
| 0.2.x | >= 1.72.0 | Added tool system |
| 1.0.x | >= 1.75.0 | Stable API |

---

**Need help?** Check out [Examples](/en/examples/) or visit [GitHub Issues](https://github.com/Ai00-X/rwkv-agent-kit/issues).
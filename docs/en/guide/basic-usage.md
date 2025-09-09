---
title: basic-usage
createTime: 2025/09/08 22:26:57
permalink: /en/article/2wrknwur/
---
# Basic Usage

This guide covers the fundamental usage patterns of RWKV Agent Kit.

## Quick Start

### Creating Your First Agent

```rust
use rwkv_agent_kit::{RwkvAgentKit, Agent, AgentConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the kit
    let kit = RwkvAgentKit::new("config.toml").await?;
    
    // Create an agent configuration
    let config = AgentConfig::new()
        .with_name("assistant")
        .with_system_prompt("You are a helpful assistant.")
        .with_max_tokens(1024);
    
    // Create the agent
    let agent = kit.create_agent(config).await?;
    
    // Send a message
    let response = agent.chat("Hello, how are you?").await?;
    println!("Agent: {}", response);
    
    Ok(())
}
```

### Basic Chat Loop

```rust
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let kit = RwkvAgentKit::new("config.toml").await?;
    let agent = kit.create_agent(AgentConfig::default()).await?;
    
    loop {
        print!("You: ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        if input.trim() == "quit" {
            break;
        }
        
        let response = agent.chat(&input).await?;
        println!("Agent: {}", response);
    }
    
    Ok(())
}
```

## Core Concepts

### Agents

Agents are the primary interface for interacting with RWKV models. Each agent maintains its own conversation context and memory.

```rust
// Create multiple agents with different personalities
let assistant = kit.create_agent(
    AgentConfig::new()
        .with_name("assistant")
        .with_system_prompt("You are a helpful assistant.")
).await?;

let creative_writer = kit.create_agent(
    AgentConfig::new()
        .with_name("writer")
        .with_system_prompt("You are a creative writer.")
).await?;
```

### Memory System

The memory system automatically stores and retrieves relevant conversation history:

```rust
// Memory is automatically managed
let response1 = agent.chat("My name is Alice").await?;
let response2 = agent.chat("What's my name?").await?; // Will remember "Alice"

// Access memory directly
let memories = agent.get_memories("name").await?;
for memory in memories {
    println!("Remembered: {}", memory.content);
}
```

### Tools

Agents can use tools to extend their capabilities:

```rust
use rwkv_agent_kit::tools::{Calculator, WebSearch};

// Add tools to an agent
let agent = kit.create_agent(
    AgentConfig::new()
        .with_tool(Calculator::new())
        .with_tool(WebSearch::new("your-api-key"))
).await?;

// The agent can now perform calculations and web searches
let response = agent.chat("What's 15 * 23 + 45?").await?;
```

## Configuration Options

### Agent Configuration

```rust
let config = AgentConfig::new()
    .with_name("my_agent")
    .with_system_prompt("Custom system prompt")
    .with_max_tokens(2048)
    .with_temperature(0.8)
    .with_top_p(0.9)
    .with_memory_limit(100);
```

### Runtime Configuration

```rust
// Adjust parameters at runtime
agent.set_temperature(0.5).await?;
agent.set_max_tokens(1024).await?;

// Enable/disable memory
agent.set_memory_enabled(false).await?;
```

## Error Handling

```rust
use rwkv_agent_kit::error::AgentError;

match agent.chat("Hello").await {
    Ok(response) => println!("Response: {}", response),
    Err(AgentError::ModelError(e)) => eprintln!("Model error: {}", e),
    Err(AgentError::MemoryError(e)) => eprintln!("Memory error: {}", e),
    Err(e) => eprintln!("Other error: {}", e),
}
```

## Best Practices

1. **Resource Management**: Always properly dispose of agents when done
2. **Error Handling**: Handle errors gracefully in production code
3. **Memory Limits**: Set appropriate memory limits to prevent excessive resource usage
4. **System Prompts**: Use clear and specific system prompts for better results
5. **Temperature Settings**: Lower temperatures for factual tasks, higher for creative tasks

## Next Steps

- Learn about [Advanced Features](./advanced-features.md)
- Explore [API Reference](../api/)
- Check out [Examples](../examples/)
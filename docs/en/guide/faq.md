---
title: faq
createTime: 2025/09/08 22:28:04
permalink: /en/article/va8lb8ny/
---
# Frequently Asked Questions

## General Questions

### What is RWKV Agent Kit?

RWKV Agent Kit is a comprehensive framework for building intelligent agents powered by RWKV (Receptance Weighted Key Value) language models. It provides tools for creating conversational AI, multi-agent systems, and intelligent automation workflows.

### What makes RWKV different from other language models?

RWKV combines the benefits of Transformers and RNNs:
- **Linear scaling**: O(n) complexity instead of O(n²)
- **Infinite context length**: No fixed context window limitations
- **Efficient inference**: Lower memory usage and faster generation
- **Parallelizable training**: Can be trained efficiently on modern hardware

### What programming languages are supported?

Currently, RWKV Agent Kit primarily supports:
- **Rust** (primary implementation)
- **Python** (bindings available)
- **JavaScript/TypeScript** (planned)

## Installation and Setup

### What are the system requirements?

**Minimum requirements:**
- 8GB RAM
- 4GB available disk space
- Rust 1.70+ or Python 3.8+

**Recommended:**
- 16GB+ RAM
- GPU with 8GB+ VRAM (for larger models)
- SSD storage

### How do I install RWKV models?

You can download RWKV models from:
1. [Hugging Face Model Hub](https://huggingface.co/models?search=rwkv)
2. [Official RWKV releases](https://github.com/BlinkDL/RWKV-LM)

```bash
# Example: Download a model
wget https://huggingface.co/BlinkDL/rwkv-4-pile-430m/resolve/main/RWKV-4-Pile-430M-20220808-8066.pth
```

### Why am I getting "model not found" errors?

Check the following:
1. Verify the model path in your configuration
2. Ensure the model file exists and is readable
3. Check file permissions
4. Verify the model format is compatible

```toml
# config.toml
[rwkv]
model_path = "/absolute/path/to/your/model.pth"  # Use absolute paths
```

## Usage and Development

### How do I create my first agent?

```rust
use rwkv_agent_kit::{RwkvAgentKit, AgentConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let kit = RwkvAgentKit::new("config.toml").await?;
    let agent = kit.create_agent(AgentConfig::default()).await?;
    
    let response = agent.chat("Hello!").await?;
    println!("Agent: {}", response);
    
    Ok(())
}
```

### How do I improve response quality?

1. **Use better system prompts:**
```rust
let config = AgentConfig::new()
    .with_system_prompt("You are a helpful, accurate, and concise assistant.");
```

2. **Adjust temperature:**
```rust
let config = AgentConfig::new()
    .with_temperature(0.7);  // Lower for more focused responses
```

3. **Provide context:**
```rust
let response = agent.chat("Given that we're discussing programming, what is Rust?").await?;
```

### How do I handle long conversations?

The memory system automatically manages conversation history:

```rust
// Memory is handled automatically
let agent = kit.create_agent(
    AgentConfig::new()
        .with_memory_limit(100)  // Keep last 100 exchanges
).await?;

// For very long conversations, consider summarization
let summary = agent.summarize_conversation().await?;
```

### Can I use multiple models simultaneously?

Yes! You can create agents with different models:

```rust
let small_agent = kit.create_agent(
    AgentConfig::new()
        .with_model_path("small-model.pth")
).await?;

let large_agent = kit.create_agent(
    AgentConfig::new()
        .with_model_path("large-model.pth")
).await?;
```

## Performance and Optimization

### Why is my agent slow?

Common causes and solutions:

1. **Large model on CPU:**
   - Use a smaller model
   - Enable GPU acceleration
   - Use quantization

2. **Memory issues:**
   - Reduce `max_tokens`
   - Clear conversation history periodically
   - Use memory limits

3. **Inefficient prompts:**
   - Keep prompts concise
   - Avoid repetitive context

### How do I enable GPU acceleration?

```toml
# config.toml
[rwkv]
device = "cuda"  # or "mps" for Apple Silicon
model_path = "model.pth"
```

### What's the difference between model sizes?

| Model Size | Parameters | RAM Usage | Use Case |
|------------|------------|-----------|----------|
| 430M | 430 million | ~2GB | Testing, simple tasks |
| 1.5B | 1.5 billion | ~6GB | General purpose |
| 3B | 3 billion | ~12GB | Complex reasoning |
| 7B | 7 billion | ~28GB | Professional use |
| 14B | 14 billion | ~56GB | Research, specialized tasks |

## Troubleshooting

### Common Error Messages

**"Failed to load model"**
- Check model path and file permissions
- Verify model format compatibility
- Ensure sufficient RAM

**"Out of memory"**
- Use a smaller model
- Reduce batch size or max tokens
- Enable model quantization

**"Agent not responding"**
- Check if the model is still loading
- Verify network connectivity (for remote models)
- Look for deadlocks in multi-agent setups

### How do I enable debug logging?

```rust
use tracing_subscriber;

// Enable debug logging
tracing_subscriber::fmt()
    .with_max_level(tracing::Level::DEBUG)
    .init();

let kit = RwkvAgentKit::new("config.toml").await?;
```

### Memory usage keeps growing

This usually indicates a memory leak. Try:

1. **Set memory limits:**
```rust
let config = AgentConfig::new()
    .with_memory_limit(50);  // Limit conversation history
```

2. **Periodic cleanup:**
```rust
// Clear old memories periodically
agent.clear_old_memories(Duration::from_hours(24)).await?;
```

3. **Monitor resource usage:**
```rust
let stats = agent.get_memory_stats().await?;
println!("Memory usage: {} MB", stats.memory_mb);
```

## Advanced Topics

### How do I create custom tools?

See the [Advanced Features](./advanced-features.md#custom-tools-development) guide for detailed examples.

### Can I fine-tune RWKV models?

Yes, but it requires the RWKV training framework. The Agent Kit focuses on inference and agent orchestration.

### How do I deploy agents in production?

Consider:
1. **Containerization** with Docker
2. **Load balancing** for multiple agents
3. **Monitoring** and logging
4. **Rate limiting** and security
5. **Model caching** and optimization

See our [deployment guide](./deployment.md) for details.

### Is there a REST API?

You can create one using web frameworks:

```rust
// Example with Axum
use axum::{routing::post, Router};

let app = Router::new()
    .route("/chat", post(chat_handler))
    .with_state(kit);
```

## Community and Support

### Where can I get help?

- **GitHub Issues**: [Report bugs and request features](https://github.com/your-org/rwkv-agent-kit/issues)
- **Discussions**: [Community discussions](https://github.com/your-org/rwkv-agent-kit/discussions)
- **Discord**: [Join our community](https://discord.gg/rwkv)
- **Documentation**: [Read the full docs](../)

### How can I contribute?

We welcome contributions! See our [Contributing Guide](https://github.com/your-org/rwkv-agent-kit/blob/main/CONTRIBUTING.md) for details.

### Is there a roadmap?

Yes! Check our [project roadmap](https://github.com/your-org/rwkv-agent-kit/projects) for upcoming features and improvements.

## License and Legal

### What license is RWKV Agent Kit under?

RWKV Agent Kit is released under the MIT License. See the [LICENSE](https://github.com/your-org/rwkv-agent-kit/blob/main/LICENSE) file for details.

### Can I use this commercially?

Yes, the MIT license allows commercial use. However, check the licenses of any RWKV models you use, as they may have different terms.

### Are there any usage restrictions?

The framework itself has no restrictions beyond the MIT license. Model-specific restrictions may apply depending on the RWKV model you choose.
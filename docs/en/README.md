---
home: true
config:
  - type: hero
    full: true
    background: tint-plate
    tintPlate: 210
    hero:
      name: RWKV Agent Kit
      tagline: <span lang="en">RWKV-based<br>Agent Development Framework</span>
      text: <span lang="en">Providing AI agents with true memory and thinking capabilities</span>
      actions:
        - theme: brand
          text: Get Started →
          link: /en/guide/
        - theme: alt
          text: GitHub
          link: https://github.com/Ai00-X/rwkv-agent-kit
  - type: features
    features:
      - title: 💡 Easy to Use
        details: Simple API design, create intelligent agents with just a few lines of code, supporting rapid prototyping and production deployment
      - title: 🧠 True Memory
        details: Long-term memory system based on vector databases, enabling AI agents to remember and learn from historical conversations
      - title: ⚡ High Performance
        details: Efficient inference engine based on RWKV models, supporting CPU inference with millisecond-level response speed
      - title: 🔧 Extensible
        details: Flexible tool system and plugin architecture, easily integrate various external services and functional modules
  - type: features
    title: 🌟 Core Features
    features:
      - title: 🤖 RWKV LLM Integration
        details: Native RWKV model support, intelligent state management, multimodal input, streaming output, optimized inference performance
      - title: 👥 Multi-Agent System
        details: Agent orchestration and collaboration, role customization, communication mechanisms, load balancing, intelligent task allocation
      - title: 🧠 Intelligent Memory System
        details: Dynamic short-term memory management, persistent long-term memory storage, semantic fragment aggregation, graph storage, personal profiling
      - title: 🔍 Vector Retrieval Capabilities
        details: Model2Vec integration, semantic search, hybrid retrieval, performance optimization, query optimizer and caching mechanisms
      - title: 🛠️ Tool Ecosystem
        details: Tool registry, shared tools, extension interfaces, error handling, cross-agent tool sharing mechanisms
      - title: 🗄️ Data Storage
        details: SQLite database, unified interface, data migration, performance monitoring, lightweight yet powerful local storage
---

## Getting Started

RWKV Agent Kit is an intelligent agent toolkit based on the RWKV model, focusing on providing AI agents with true memory and thinking capabilities.

### Installation

```bash
cargo add rwkv-agent-kit
```

### Quick Start

```rust
use rwkv_agent_kit::RwkvAgentKit;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let agent = RwkvAgentKit::new("config.toml").await?;
    
    let response = agent.chat("Hello, please introduce yourself").await?;
    println!("AI: {}", response);
    
    Ok(())
}
```

## Why Choose RWKV Agent Kit?

- **Easy to Use** - Simple API design, create intelligent agents with just a few lines of code
- **True Memory** - Long-term memory system based on vector databases
- **High Performance** - Based on RWKV models, supporting CPU inference with millisecond-level response
- **Extensible** - Flexible tool system and plugin architecture

## Learn More

- [Get Started](/en/guide/) - Learn how to install and use
- [API Reference](/en/api/) - Detailed API documentation
- [Configuration](/en/config/) - Configuration options and best practices
- [GitHub](https://github.com/Ai00-X/rwkv-agent-kit) - View source code and contribute
---
title: installation
createTime: 2025/09/08 22:26:44
permalink: /en/article/xbwvi9qb/
---
# Installation Guide

## Prerequisites

Before installing RWKV Agent Kit, ensure you have the following prerequisites:

- Rust 1.70 or later
- Python 3.8 or later (for Python bindings)
- Git

## Installation Methods

### Method 1: Install from Crates.io

```bash
cargo add rwkv-agent-kit
```

### Method 2: Install from Source

1. Clone the repository:
```bash
git clone https://github.com/your-org/rwkv-agent-kit.git
cd rwkv-agent-kit
```

2. Build the project:
```bash
cargo build --release
```

3. Run tests to verify installation:
```bash
cargo test
```

## Configuration

After installation, you need to configure the RWKV model path and other settings:

1. Create a configuration file:
```toml
[rwkv]
model_path = "path/to/your/rwkv/model.pth"
max_tokens = 2048
temperature = 0.7

[database]
url = "sqlite://memory.db"

[memory]
max_entries = 1000
embedding_dim = 768
```

2. Set environment variables (optional):
```bash
export RWKV_MODEL_PATH="/path/to/model.pth"
export RWKV_CONFIG_PATH="/path/to/config.toml"
```

## Verification

To verify your installation is working correctly:

```rust
use rwkv_agent_kit::RwkvAgentKit;

fn main() {
    let kit = RwkvAgentKit::new("config.toml").unwrap();
    println!("RWKV Agent Kit initialized successfully!");
}
```

## Troubleshooting

### Common Issues

1. **Model loading errors**: Ensure the model path is correct and the model file is accessible.
2. **Memory issues**: Reduce `max_tokens` or `max_entries` in configuration.
3. **Database connection errors**: Check database URL and permissions.

### Getting Help

If you encounter issues:
- Check the [FAQ](./faq.md)
- Visit our [GitHub Issues](https://github.com/your-org/rwkv-agent-kit/issues)
- Join our community discussions
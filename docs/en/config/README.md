# Configuration Guide

RWKV Agent Kit provides a flexible configuration system that supports multiple configuration methods and rich configuration options. This guide will detail how to configure and optimize your intelligent agents.

## 📋 Configuration File Formats

RWKV Agent Kit supports TOML, JSON, and YAML format configuration files:

### TOML Format (Recommended)

```toml
# config.toml
[model]
path = "./models/rwkv-4-world-7b-v1-20230626-ctx4096.pth"
device = "wgpu"  # or "cpu"
precision = "fp16"  # or "fp32"
max_tokens = 2048
temperature = 0.7
top_p = 0.9
top_k = 40

[memory]
type = "sqlite"
path = "./data/agent_memory.db"
max_context_length = 4096
memory_threshold = 0.8
embedding_model = "sentence-transformers/all-MiniLM-L6-v2"

[database]
url = "sqlite://./data/agent.db"
max_connections = 10
connection_timeout = 30

[logging]
level = "info"
file = "./logs/agent.log"
max_file_size = "10MB"
max_files = 5

[tools]
enable_builtin = true
custom_tools_dir = "./tools"

[performance]
thread_pool_size = 4
batch_size = 8
cache_size = 1000
```

### JSON Format

```json
{
  "model": {
    "path": "./models/rwkv-model.pth",
    "device": "wgpu",
    "precision": "fp16",
    "max_tokens": 2048,
    "temperature": 0.7,
    "top_p": 0.9,
    "top_k": 40
  },
  "memory": {
    "type": "sqlite",
    "path": "./data/memory.db",
    "max_context_length": 4096,
    "memory_threshold": 0.8
  },
  "database": {
    "url": "sqlite://./data/agent.db"
  },
  "logging": {
    "level": "info",
    "file": "./logs/agent.log"
  }
}
```

## 🎛️ Configuration Options Details

### Model Configuration (model)

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `path` | String | Required | RWKV model file path |
| `device` | String | "cpu" | Runtime device: "cpu", "wgpu", "mps" |
| `precision` | String | "fp32" | Precision: "fp16", "fp32" |
| `max_tokens` | u32 | 2048 | Maximum generation tokens |
| `temperature` | f32 | 0.7 | Temperature parameter (0.0-2.0) |
| `top_p` | f32 | 0.9 | Top-p sampling parameter |
| `top_k` | u32 | 40 | Top-k sampling parameter |
| `repetition_penalty` | f32 | 1.1 | Repetition penalty |

### Memory Configuration (memory)

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `type` | String | "sqlite" | Storage type: "sqlite", "postgresql" |
| `path` | String | "./memory.db" | SQLite database path |
| `url` | String | - | PostgreSQL connection URL |
| `max_context_length` | u32 | 2048 | Maximum context length |
| `memory_threshold` | f32 | 0.7 | Memory storage threshold |
| `embedding_model` | String | "all-MiniLM-L6-v2" | Embedding model name |
| `max_memories` | u32 | 10000 | Maximum number of memories |

### Database Configuration (database)

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `url` | String | Required | Database connection URL |
| `max_connections` | u32 | 10 | Maximum connections |
| `connection_timeout` | u64 | 30 | Connection timeout (seconds) |
| `idle_timeout` | u64 | 600 | Idle timeout (seconds) |
| `enable_logging` | bool | false | Enable SQL logging |

### Logging Configuration (logging)

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `level` | String | "info" | Log level: "trace", "debug", "info", "warn", "error" |
| `file` | String | - | Log file path |
| `max_file_size` | String | "10MB" | Maximum file size |
| `max_files` | u32 | 5 | Maximum number of files |
| `format` | String | "json" | Log format: "json", "text" |

## 🚀 Performance Optimization Configuration

### GPU Acceleration Configuration

```toml
[model]
device = "wgpu"
precision = "fp16"  # Use half precision for speed
batch_size = 8      # Batch size

[performance]
gpu_memory_fraction = 0.8  # GPU memory usage ratio
enable_tensor_cores = true # Enable Tensor Cores
```

### Memory Optimization Configuration

```toml
[memory]
max_context_length = 1024  # Reduce context length
memory_threshold = 0.9     # Increase memory threshold
max_memories = 5000        # Limit number of memories

[performance]
cache_size = 500           # Reduce cache size
thread_pool_size = 2       # Reduce thread count
```

### High Concurrency Configuration

```toml
[database]
max_connections = 50
connection_timeout = 10

[performance]
thread_pool_size = 16
batch_size = 16
max_concurrent_requests = 100
```

## 🔧 Environment Variable Configuration

You can use environment variables to override settings in configuration files:

```bash
# Model configuration
export RWKV_MODEL_PATH="/path/to/model.pth"
export RWKV_DEVICE="wgpu"
export RWKV_PRECISION="fp16"

# Database configuration
export RWKV_DATABASE_URL="sqlite://./data/agent.db"

# Logging configuration
export RWKV_LOG_LEVEL="debug"
export RWKV_LOG_FILE="./logs/debug.log"
```

Usage in code:

```rust
use rwkv_agent_kit::config::AgentConfig;

// Automatically load configuration from environment variables
let config = AgentConfig::from_env()?;

// Or merge configuration file and environment variables
let config = AgentConfig::from_file("config.toml")?
    .merge_env()?;
```

## 📊 Configuration Validation

### Validate Configuration File

```rust
use rwkv_agent_kit::config::{AgentConfig, ConfigError};

match AgentConfig::from_file("config.toml") {
    Ok(config) => {
        // Validate configuration
        if let Err(e) = config.validate() {
            eprintln!("Configuration validation failed: {}", e);
            return;
        }
        println!("Configuration validation successful");
    }
    Err(ConfigError::FileNotFound) => {
        eprintln!("Configuration file not found");
    }
    Err(ConfigError::ParseError(e)) => {
        eprintln!("Configuration file parse error: {}", e);
    }
    Err(e) => {
        eprintln!("Other configuration error: {}", e);
    }
}
```

### Configuration Check Tool

```bash
# Use command line tool to check configuration
rwkv-agent-kit check-config config.toml

# Example output
✓ Model file exists
✓ Database connection normal
✓ Memory configuration reasonable
⚠ GPU unavailable, will use CPU
✓ Configuration validation passed
```

## 🎯 Use Case Configurations

### Development Environment

```toml
[model]
device = "cpu"
max_tokens = 512

[logging]
level = "debug"
file = "./logs/dev.log"

[memory]
max_memories = 1000
```

### Production Environment

```toml
[model]
device = "cuda"
precision = "fp16"
max_tokens = 2048

[logging]
level = "warn"
file = "/var/log/rwkv-agent/agent.log"
max_file_size = "100MB"

[database]
url = "postgresql://user:pass@localhost/rwkv_agent"
max_connections = 50

[performance]
thread_pool_size = 16
cache_size = 5000
```

### Edge Devices

```toml
[model]
device = "cpu"
precision = "fp32"
max_tokens = 256

[memory]
type = "sqlite"
max_context_length = 512
max_memories = 500

[performance]
thread_pool_size = 2
batch_size = 1
cache_size = 100
```

## 🔍 Troubleshooting

### Common Configuration Issues

1. **Model Loading Failed**
   ```
   Error: Unable to load model file
   Solution: Check if model path is correct and file exists
   ```

2. **GPU Memory Insufficient**
   ```
   Error: CUDA out of memory
   Solution: Reduce batch_size or use fp16 precision
   ```

3. **Database Connection Failed**
   ```
   Error: Unable to connect to database
   Solution: Check database URL and permission settings
   ```

### Performance Tuning Recommendations

1. **CPU Optimization**
   - Set appropriate `thread_pool_size`
   - Use `fp32` precision
   - Reduce `batch_size`

2. **GPU Optimization**
   - Use `fp16` precision
   - Increase `batch_size`
   - Enable `tensor_cores`

3. **Memory Optimization**
   - Reduce `max_context_length`
   - Limit `max_memories`
   - Adjust `cache_size`

---

**Need more help?** Check [API Documentation](/en/api/) or visit [GitHub Issues](https://github.com/Ai00-X/rwkv-agent-kit/issues).
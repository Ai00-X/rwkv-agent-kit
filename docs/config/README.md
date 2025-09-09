# 配置指南

RWKV Agent Kit 提供了灵活的配置系统，支持多种配置方式和丰富的配置选项。本指南将详细介绍如何配置和优化您的智能体。

## 📋 配置文件格式

RWKV Agent Kit 支持 TOML、JSON 和 YAML 格式的配置文件：

### TOML 格式 (推荐)

```toml
# config.toml
[database]
database_type = "SQLite"
url = "sqlite:./data/agent_kit.db"
max_connections = 10
min_connections = 1
connect_timeout = 30
idle_timeout = 600
max_lifetime = 3600
enable_ssl = false
database_name = "agent_kit"
table_prefix = ""
auto_migrate = true

[cache]
enabled = true
cache_type = "Memory"
max_size_mb = 256
ttl_seconds = 3600
lru_capacity = 1000

[vector]
dimension = 768
similarity_threshold = 0.7
index_type = "Flat"
distance_metric = "Cosine"

[logging]
level = "Info"
format = "Json"
targets = ["Console", "File"]
max_file_size_mb = 10
max_files = 5
structured = true

[performance]
worker_threads = 4
batch_size = 8
query_timeout_ms = 5000
creation_timeout_ms = 10000
concurrency_limit = 100
memory_limit_mb = 1024
enable_metrics = true
metrics_interval_seconds = 60
```

### JSON 格式

```json
{
  "database": {
    "database_type": "SQLite",
    "url": "sqlite:./data/agent_kit.db",
    "max_connections": 10,
    "min_connections": 1,
    "connect_timeout": 30,
    "auto_migrate": true,
    "database_name": "agent_kit",
    "table_prefix": ""
  },
  "cache": {
    "enabled": true,
    "cache_type": "Memory",
    "max_size_mb": 256,
    "ttl_seconds": 3600,
    "lru_capacity": 1000
  },
  "performance": {
    "worker_threads": 4,
    "batch_size": 8,
    "query_timeout_ms": 5000,
    "creation_timeout_ms": 10000,
    "concurrency_limit": 100,
    "memory_limit_mb": 1024,
    "enable_metrics": true,
    "metrics_interval_seconds": 60
  },
  "logging": {
    "level": "Info",
    "format": "Json",
    "targets": ["Console", "File"],
    "max_file_size_mb": 10,
    "max_files": 5,
    "structured": true
  }
}
```

## 🎛️ 配置选项详解

### 数据库配置 (database)

| 选项 | 类型 | 默认值 | 描述 |
|------|------|--------|------|
| `database_type` | String | "SQLite" | 数据库类型："SQLite" |
| `url` | String | 必需 | 数据库连接URL |
| `max_connections` | u32 | 10 | 最大连接数 |
| `min_connections` | u32 | 1 | 最小连接数 |
| `connect_timeout` | u64 | 30 | 连接超时时间（秒） |
| `idle_timeout` | u64 | 600 | 空闲超时时间（秒） |
| `max_lifetime` | u64 | 3600 | 最大生命周期（秒） |
| `auto_migrate` | bool | true | 是否自动迁移 |
| `database_name` | String | "agent_kit" | 数据库名称 |
| `table_prefix` | String | "" | 表前缀 |

### 缓存配置 (cache)

| 选项 | 类型 | 默认值 | 描述 |
|------|------|--------|------|
| `enabled` | bool | true | 是否启用缓存 |
| `cache_type` | String | "Memory" | 缓存类型："Memory", "Redis", "Hybrid" |
| `max_size_mb` | u64 | 256 | 最大缓存大小（MB） |
| `ttl_seconds` | u64 | 3600 | 缓存TTL（秒） |
| `lru_capacity` | usize | 1000 | LRU缓存容量 |

### 性能配置 (performance)

| 选项 | 类型 | 默认值 | 描述 |
|------|------|--------|------|
| `worker_threads` | usize | 4 | 工作线程数 |
| `batch_size` | usize | 8 | 批处理大小 |
| `query_timeout_ms` | u64 | 5000 | 查询超时（毫秒） |
| `creation_timeout_ms` | u64 | 10000 | 创建超时（毫秒） |
| `concurrency_limit` | usize | 100 | 并发限制 |
| `memory_limit_mb` | u64 | 1024 | 内存限制（MB） |
| `enable_metrics` | bool | true | 是否启用性能监控 |
| `metrics_interval_seconds` | u64 | 60 | 指标收集间隔（秒） |

### 日志配置 (logging)

| 选项 | 类型 | 默认值 | 描述 |
|------|------|--------|------|
| `level` | String | "info" | 日志级别："trace", "debug", "info", "warn", "error" |
| `file` | String | - | 日志文件路径 |
| `max_file_size` | String | "10MB" | 最大文件大小 |
| `max_files` | u32 | 5 | 最大文件数量 |
| `format` | String | "json" | 日志格式："json", "text" |

## 🚀 性能优化配置

### 高性能配置

```toml
[performance]
worker_threads = 8          # 增加工作线程数
batch_size = 16             # 增大批处理大小
concurrency_limit = 200     # 提高并发限制
memory_limit_mb = 2048      # 增加内存限制
enable_metrics = true       # 启用性能监控
metrics_interval_seconds = 30  # 更频繁的指标收集

[cache]
enabled = true
cache_type = "Memory"
max_size_mb = 512           # 增大缓存大小
ttl_seconds = 7200          # 延长缓存时间
lru_capacity = 2000         # 增大LRU容量
```

### 内存优化配置

```toml
[cache]
enabled = true
cache_type = "Memory"
max_size_mb = 256                   # 减少缓存大小
ttl_seconds = 3600
lru_capacity = 1000

[performance]
worker_threads = 2                  # 减少线程数
batch_size = 4
memory_limit_mb = 512               # 限制内存使用
concurrency_limit = 50
enable_metrics = false              # 禁用性能监控以节省资源
```

### 高并发配置

```toml
[database]
database_type = "SQLite"
url = "sqlite:./data/agent_kit.db"
max_connections = 50
connect_timeout = 10
auto_migrate = true

[performance]
worker_threads = 16
batch_size = 16
concurrency_limit = 100
memory_limit_mb = 2048
enable_metrics = true
```



1. **CPU 优化**
   - 设置合适的 `thread_pool_size`
   - 使用 `fp32` 精度
   - 减少 `batch_size`

2. **GPU 优化**
   - 使用 `fp16` 精度
   - 增加 `batch_size`
   - 启用 `tensor_cores`

3. **内存优化**
   - 减少 `max_context_length`
   - 限制 `max_memories`
   - 调整 `cache_size`

---

**需要更多帮助？** 查看 [API 文档](/api/) 或访问 [GitHub Issues](https://github.com/Ai00-X/rwkv-agent-kit/issues)。
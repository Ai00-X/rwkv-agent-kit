---
title: config
createTime: 2025/09/08 15:27:08
permalink: /article/zm61tx8x/
---
# 配置管理API

## 概述

配置管理模块提供了灵活的配置系统，支持多种配置源、环境变量、配置验证和热重载等功能。本文档详细描述了配置管理的所有API接口。

## 核心结构

### RwkvAgentKitConfig

主配置结构，包含所有子系统的配置选项。

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RwkvAgentKitConfig {
    /// 数据库配置
    pub database: DatabaseConfig,
    /// 记忆管理配置
    pub memory: MemoryConfig,
    /// 嵌入模型配置
    pub embedding: EmbeddingConfig,
    /// 工具系统配置
    pub tools: ToolConfig,
    /// 日志配置
    pub logging: LoggingConfig,
    /// 性能配置
    pub performance: PerformanceConfig,
    /// 安全配置
    pub security: SecurityConfig,
    /// 网络配置
    pub network: NetworkConfig,
}

impl RwkvAgentKitConfig {
    /// 从文件加载配置
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self>;
    
    /// 从环境变量加载配置
    pub fn from_env() -> Result<Self>;
    
    /// 从多个源合并配置
    pub fn from_sources(sources: Vec<ConfigSource>) -> Result<Self>;
    
    /// 验证配置有效性
    pub fn validate(&self) -> Result<()>;
    
    /// 保存配置到文件
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()>;
    
    /// 获取默认配置
    pub fn default() -> Self;
    
    /// 合并其他配置
    pub fn merge(&mut self, other: Self) -> Result<()>;
    
    /// 应用环境变量覆盖
    pub fn apply_env_overrides(&mut self) -> Result<()>;
}

impl Default for RwkvAgentKitConfig {
    fn default() -> Self {
        Self {
            database: DatabaseConfig::default(),
            memory: MemoryConfig::default(),
            embedding: EmbeddingConfig::default(),
            tools: ToolConfig::default(),
            logging: LoggingConfig::default(),
            performance: PerformanceConfig::default(),
            security: SecurityConfig::default(),
            network: NetworkConfig::default(),
        }
    }
}
```

## 配置源

### ConfigSource

配置源枚举，支持多种配置来源。

```rust
#[derive(Debug, Clone)]
pub enum ConfigSource {
    /// 文件配置源
    File {
        path: PathBuf,
        format: ConfigFormat,
        required: bool,
    },
    /// 环境变量配置源
    Environment {
        prefix: Option<String>,
    },
    /// 内存配置源
    Memory {
        config: RwkvAgentKitConfig,
    },
    /// 远程配置源
    Remote {
        url: String,
        headers: HashMap<String, String>,
        timeout: Duration,
    },
    /// 命令行参数配置源
    CommandLine {
        args: Vec<String>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConfigFormat {
    Json,
    Yaml,
    Toml,
    Ron,
}
```

### ConfigBuilder

配置构建器，用于灵活构建配置。

```rust
pub struct ConfigBuilder {
    sources: Vec<ConfigSource>,
    env_prefix: Option<String>,
    validation_enabled: bool,
    merge_strategy: MergeStrategy,
}

impl ConfigBuilder {
    /// 创建新的配置构建器
    pub fn new() -> Self;
    
    /// 添加文件配置源
    pub fn add_file<P: AsRef<Path>>(mut self, path: P, format: ConfigFormat) -> Self;
    
    /// 添加可选文件配置源
    pub fn add_optional_file<P: AsRef<Path>>(mut self, path: P, format: ConfigFormat) -> Self;
    
    /// 添加环境变量配置源
    pub fn add_env(mut self) -> Self;
    
    /// 设置环境变量前缀
    pub fn env_prefix<S: Into<String>>(mut self, prefix: S) -> Self;
    
    /// 添加远程配置源
    pub fn add_remote<S: Into<String>>(mut self, url: S) -> Self;
    
    /// 启用/禁用配置验证
    pub fn validation(mut self, enabled: bool) -> Self;
    
    /// 设置合并策略
    pub fn merge_strategy(mut self, strategy: MergeStrategy) -> Self;
    
    /// 构建配置
    pub fn build(self) -> Result<RwkvAgentKitConfig>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MergeStrategy {
    /// 覆盖策略（后面的配置覆盖前面的）
    Override,
    /// 合并策略（深度合并）
    Merge,
    /// 仅填充空值
    FillEmpty,
}
```

## 数据库配置

### DatabaseConfig

数据库相关配置。

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    /// 数据库连接URL
    pub url: String,
    /// 最大连接数
    pub max_connections: u32,
    /// 最小连接数
    pub min_connections: u32,
    /// 连接超时时间（秒）
    pub connection_timeout: u64,
    /// 空闲超时时间（秒）
    pub idle_timeout: u64,
    /// 最大生命周期（秒）
    pub max_lifetime: u64,
    /// 是否启用WAL模式
    pub enable_wal: bool,
    /// 是否启用外键约束
    pub enable_foreign_keys: bool,
    /// 缓存大小（MB）
    pub cache_size_mb: u32,
    /// 页面大小（字节）
    pub page_size: u32,
    /// 是否启用同步模式
    pub synchronous: SynchronousMode,
    /// 日志模式
    pub journal_mode: JournalMode,
    /// 临时存储模式
    pub temp_store: TempStore,
    /// 锁定模式
    pub locking_mode: LockingMode,
    /// 是否启用查询计划缓存
    pub enable_query_cache: bool,
    /// 查询缓存大小
    pub query_cache_size: u32,
    /// 备份配置
    pub backup: Option<BackupConfig>,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            url: "sqlite:rwkv_agent.db".to_string(),
            max_connections: 10,
            min_connections: 1,
            connection_timeout: 30,
            idle_timeout: 600,
            max_lifetime: 3600,
            enable_wal: true,
            enable_foreign_keys: true,
            cache_size_mb: 64,
            page_size: 4096,
            synchronous: SynchronousMode::Normal,
            journal_mode: JournalMode::Wal,
            temp_store: TempStore::Memory,
            locking_mode: LockingMode::Normal,
            enable_query_cache: true,
            query_cache_size: 100,
            backup: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SynchronousMode {
    Off,
    Normal,
    Full,
    Extra,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum JournalMode {
    Delete,
    Truncate,
    Persist,
    Memory,
    Wal,
    Off,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TempStore {
    Default,
    File,
    Memory,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LockingMode {
    Normal,
    Exclusive,
}
```

### BackupConfig

数据库备份配置。

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupConfig {
    /// 是否启用自动备份
    pub enabled: bool,
    /// 备份间隔（小时）
    pub interval_hours: u64,
    /// 备份保留天数
    pub retention_days: u64,
    /// 备份目录
    pub backup_dir: PathBuf,
    /// 备份文件名模式
    pub filename_pattern: String,
    /// 是否压缩备份
    pub compress: bool,
    /// 压缩级别（1-9）
    pub compression_level: u8,
    /// 是否验证备份完整性
    pub verify_integrity: bool,
}

impl Default for BackupConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            interval_hours: 24,
            retention_days: 30,
            backup_dir: PathBuf::from("./backups"),
            filename_pattern: "rwkv_agent_%Y%m%d_%H%M%S.db".to_string(),
            compress: true,
            compression_level: 6,
            verify_integrity: true,
        }
    }
}
```

## 记忆配置

### MemoryConfig

记忆管理相关配置。

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryConfig {
    /// 最大记忆数量
    pub max_memories: Option<usize>,
    /// 默认重要性阈值
    pub importance_threshold: f32,
    /// 记忆衰减配置
    pub decay: DecayConfig,
    /// 自动清理配置
    pub cleanup: CleanupConfig,
    /// 记忆演化配置
    pub evolution: EvolutionConfig,
    /// 连接管理配置
    pub connections: ConnectionConfig,
    /// 缓存配置
    pub cache: CacheConfig,
    /// 索引配置
    pub indexing: IndexingConfig,
}

impl Default for MemoryConfig {
    fn default() -> Self {
        Self {
            max_memories: Some(100000),
            importance_threshold: 0.1,
            decay: DecayConfig::default(),
            cleanup: CleanupConfig::default(),
            evolution: EvolutionConfig::default(),
            connections: ConnectionConfig::default(),
            cache: CacheConfig::default(),
            indexing: IndexingConfig::default(),
        }
    }
}
```

### DecayConfig

记忆衰减配置。

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecayConfig {
    /// 是否启用衰减
    pub enabled: bool,
    /// 衰减率
    pub decay_rate: f32,
    /// 衰减函数类型
    pub decay_function: DecayFunction,
    /// 衰减间隔（小时）
    pub decay_interval_hours: u64,
    /// 最小重要性（低于此值的记忆将被删除）
    pub min_importance: f32,
    /// 保护期（天数，新记忆在此期间不会衰减）
    pub protection_period_days: u64,
}

impl Default for DecayConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            decay_rate: 0.01,
            decay_function: DecayFunction::Exponential,
            decay_interval_hours: 24,
            min_importance: 0.01,
            protection_period_days: 7,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DecayFunction {
    /// 线性衰减
    Linear,
    /// 指数衰减
    Exponential,
    /// 对数衰减
    Logarithmic,
    /// 幂函数衰减
    Power { exponent: f32 },
    /// 自定义衰减函数
    Custom { formula: String },
}
```

### CleanupConfig

自动清理配置。

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CleanupConfig {
    /// 是否启用自动清理
    pub enabled: bool,
    /// 清理间隔（小时）
    pub interval_hours: u64,
    /// 清理策略
    pub strategies: Vec<CleanupStrategy>,
    /// 是否在启动时执行清理
    pub cleanup_on_startup: bool,
    /// 清理时的批处理大小
    pub batch_size: usize,
    /// 清理操作超时（秒）
    pub timeout_seconds: u64,
}

impl Default for CleanupConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            interval_hours: 24,
            strategies: vec![
                CleanupStrategy::LowImportance { threshold: 0.1 },
                CleanupStrategy::Expired,
                CleanupStrategy::Duplicate { similarity_threshold: 0.95 },
            ],
            cleanup_on_startup: false,
            batch_size: 1000,
            timeout_seconds: 300,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CleanupStrategy {
    /// 清理低重要性记忆
    LowImportance { threshold: f32 },
    /// 清理过期记忆
    Expired,
    /// 清理重复记忆
    Duplicate { similarity_threshold: f32 },
    /// 清理旧记忆（按年龄）
    OldAge { max_age_days: u64 },
    /// 清理低频访问记忆
    LowFrequency { min_access_count: u32 },
    /// 自定义清理策略
    Custom { name: String, config: Value },
}
```

## 嵌入配置

### EmbeddingConfig

嵌入模型相关配置。

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingConfig {
    /// 嵌入提供者
    pub provider: EmbeddingProvider,
    /// 模型名称
    pub model_name: String,
    /// 嵌入维度
    pub dimensions: usize,
    /// 批处理大小
    pub batch_size: usize,
    /// 请求超时（秒）
    pub timeout_seconds: u64,
    /// 重试配置
    pub retry: RetryConfig,
    /// 缓存配置
    pub cache: EmbeddingCacheConfig,
    /// 预处理配置
    pub preprocessing: PreprocessingConfig,
}

impl Default for EmbeddingConfig {
    fn default() -> Self {
        Self {
            provider: EmbeddingProvider::Local,
            model_name: "all-MiniLM-L6-v2".to_string(),
            dimensions: 384,
            batch_size: 32,
            timeout_seconds: 30,
            retry: RetryConfig::default(),
            cache: EmbeddingCacheConfig::default(),
            preprocessing: PreprocessingConfig::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EmbeddingProvider {
    /// 本地模型
    Local,
    /// OpenAI
    OpenAI {
        api_key: String,
        base_url: Option<String>,
    },
    /// Hugging Face
    HuggingFace {
        api_key: Option<String>,
        model_id: String,
    },
    /// Cohere
    Cohere {
        api_key: String,
    },
    /// 自定义提供者
    Custom {
        name: String,
        endpoint: String,
        headers: HashMap<String, String>,
    },
}
```

### RetryConfig

重试配置。

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryConfig {
    /// 最大重试次数
    pub max_attempts: u32,
    /// 初始延迟（毫秒）
    pub initial_delay_ms: u64,
    /// 最大延迟（毫秒）
    pub max_delay_ms: u64,
    /// 退避策略
    pub backoff_strategy: BackoffStrategy,
    /// 可重试的错误类型
    pub retryable_errors: Vec<String>,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            initial_delay_ms: 1000,
            max_delay_ms: 30000,
            backoff_strategy: BackoffStrategy::Exponential { multiplier: 2.0 },
            retryable_errors: vec![
                "timeout".to_string(),
                "rate_limit".to_string(),
                "server_error".to_string(),
            ],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BackoffStrategy {
    /// 固定延迟
    Fixed,
    /// 线性退避
    Linear { increment: u64 },
    /// 指数退避
    Exponential { multiplier: f64 },
    /// 随机抖动
    Jitter { max_jitter_ms: u64 },
}
```

## 工具配置

### ToolConfig

工具系统配置。

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolConfig {
    /// 工具目录
    pub tool_directories: Vec<PathBuf>,
    /// 是否启用内置工具
    pub enable_builtin_tools: bool,
    /// 工具执行超时（秒）
    pub execution_timeout_seconds: u64,
    /// 最大并发工具数
    pub max_concurrent_tools: usize,
    /// 工具安全配置
    pub security: ToolSecurityConfig,
    /// 工具缓存配置
    pub cache: ToolCacheConfig,
    /// 工具监控配置
    pub monitoring: ToolMonitoringConfig,
}

impl Default for ToolConfig {
    fn default() -> Self {
        Self {
            tool_directories: vec![PathBuf::from("./tools")],
            enable_builtin_tools: true,
            execution_timeout_seconds: 300,
            max_concurrent_tools: 10,
            security: ToolSecurityConfig::default(),
            cache: ToolCacheConfig::default(),
            monitoring: ToolMonitoringConfig::default(),
        }
    }
}
```

### ToolSecurityConfig

工具安全配置。

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolSecurityConfig {
    /// 是否启用沙箱
    pub enable_sandbox: bool,
    /// 允许的工具列表
    pub allowed_tools: Option<Vec<String>>,
    /// 禁止的工具列表
    pub blocked_tools: Vec<String>,
    /// 资源限制
    pub resource_limits: ResourceLimits,
    /// 网络访问控制
    pub network_access: NetworkAccessConfig,
    /// 文件系统访问控制
    pub filesystem_access: FilesystemAccessConfig,
}

impl Default for ToolSecurityConfig {
    fn default() -> Self {
        Self {
            enable_sandbox: true,
            allowed_tools: None,
            blocked_tools: vec![],
            resource_limits: ResourceLimits::default(),
            network_access: NetworkAccessConfig::default(),
            filesystem_access: FilesystemAccessConfig::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLimits {
    /// 最大内存使用（MB）
    pub max_memory_mb: Option<u64>,
    /// 最大CPU使用率（百分比）
    pub max_cpu_percent: Option<f32>,
    /// 最大执行时间（秒）
    pub max_execution_time_seconds: Option<u64>,
    /// 最大文件大小（MB）
    pub max_file_size_mb: Option<u64>,
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            max_memory_mb: Some(512),
            max_cpu_percent: Some(80.0),
            max_execution_time_seconds: Some(300),
            max_file_size_mb: Some(100),
        }
    }
}
```

## 性能配置

### PerformanceConfig

性能相关配置。

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    /// 工作线程数
    pub worker_threads: Option<usize>,
    /// 是否启用并行处理
    pub enable_parallel_processing: bool,
    /// 批处理大小
    pub batch_size: usize,
    /// 缓存配置
    pub cache: GlobalCacheConfig,
    /// 内存池配置
    pub memory_pool: MemoryPoolConfig,
    /// 预取配置
    pub prefetch: PrefetchConfig,
    /// 压缩配置
    pub compression: CompressionConfig,
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            worker_threads: None, // 使用系统默认
            enable_parallel_processing: true,
            batch_size: 100,
            cache: GlobalCacheConfig::default(),
            memory_pool: MemoryPoolConfig::default(),
            prefetch: PrefetchConfig::default(),
            compression: CompressionConfig::default(),
        }
    }
}
```

## 配置验证

### ConfigValidator

配置验证器。

```rust
pub struct ConfigValidator {
    rules: Vec<ValidationRule>,
    strict_mode: bool,
}

impl ConfigValidator {
    /// 创建新的验证器
    pub fn new() -> Self;
    
    /// 添加验证规则
    pub fn add_rule(mut self, rule: ValidationRule) -> Self;
    
    /// 启用严格模式
    pub fn strict_mode(mut self, enabled: bool) -> Self;
    
    /// 验证配置
    pub fn validate(&self, config: &RwkvAgentKitConfig) -> Result<ValidationReport>;
}

#[derive(Debug, Clone)]
pub enum ValidationRule {
    /// 范围检查
    Range {
        field: String,
        min: f64,
        max: f64,
    },
    /// 必填字段检查
    Required {
        field: String,
    },
    /// 格式检查
    Format {
        field: String,
        pattern: String,
    },
    /// 依赖检查
    Dependency {
        field: String,
        depends_on: String,
    },
    /// 自定义验证
    Custom {
        name: String,
        validator: Box<dyn Fn(&RwkvAgentKitConfig) -> Result<()>>,
    },
}

#[derive(Debug, Clone)]
pub struct ValidationReport {
    pub is_valid: bool,
    pub errors: Vec<ValidationError>,
    pub warnings: Vec<ValidationWarning>,
}

#[derive(Debug, Clone)]
pub struct ValidationError {
    pub field: String,
    pub message: String,
    pub severity: ErrorSeverity,
}

#[derive(Debug, Clone)]
pub struct ValidationWarning {
    pub field: String,
    pub message: String,
    pub suggestion: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorSeverity {
    Low,
    Medium,
    High,
    Critical,
}
```

## 配置热重载

### ConfigWatcher

配置文件监控器，支持热重载。

```rust
pub struct ConfigWatcher {
    config: Arc<RwLock<RwkvAgentKitConfig>>,
    watchers: Vec<FileWatcher>,
    callbacks: Vec<ConfigChangeCallback>,
}

impl ConfigWatcher {
    /// 创建新的配置监控器
    pub fn new(config: RwkvAgentKitConfig) -> Self;
    
    /// 监控配置文件
    pub fn watch_file<P: AsRef<Path>>(&mut self, path: P) -> Result<()>;
    
    /// 添加配置变更回调
    pub fn on_change<F>(&mut self, callback: F) -> Result<()>
    where
        F: Fn(&RwkvAgentKitConfig, &RwkvAgentKitConfig) + Send + Sync + 'static;
    
    /// 启动监控
    pub fn start(&mut self) -> Result<()>;
    
    /// 停止监控
    pub fn stop(&mut self) -> Result<()>;
    
    /// 获取当前配置
    pub fn get_config(&self) -> RwkvAgentKitConfig;
    
    /// 手动重载配置
    pub fn reload(&mut self) -> Result<()>;
}

type ConfigChangeCallback = Box<dyn Fn(&RwkvAgentKitConfig, &RwkvAgentKitConfig) + Send + Sync>;
```

## 环境变量映射

### 环境变量命名规则

配置支持通过环境变量进行覆盖，命名规则如下：

```bash
# 基本格式：RWKV_AGENT_<SECTION>_<FIELD>

# 数据库配置
RWKV_AGENT_DATABASE_URL="sqlite:custom.db"
RWKV_AGENT_DATABASE_MAX_CONNECTIONS=20
RWKV_AGENT_DATABASE_ENABLE_WAL=true

# 记忆配置
RWKV_AGENT_MEMORY_MAX_MEMORIES=50000
RWKV_AGENT_MEMORY_IMPORTANCE_THRESHOLD=0.2
RWKV_AGENT_MEMORY_DECAY_ENABLED=true

# 嵌入配置
RWKV_AGENT_EMBEDDING_PROVIDER="openai"
RWKV_AGENT_EMBEDDING_MODEL_NAME="text-embedding-ada-002"
RWKV_AGENT_EMBEDDING_DIMENSIONS=1536

# 工具配置
RWKV_AGENT_TOOLS_EXECUTION_TIMEOUT_SECONDS=600
RWKV_AGENT_TOOLS_MAX_CONCURRENT_TOOLS=5

# 性能配置
RWKV_AGENT_PERFORMANCE_WORKER_THREADS=8
RWKV_AGENT_PERFORMANCE_ENABLE_PARALLEL_PROCESSING=true

# 日志配置
RWKV_AGENT_LOGGING_LEVEL="info"
RWKV_AGENT_LOGGING_FORMAT="json"
```

## 完整示例

### 基本配置使用

```rust
use rwkv_agent_kit::config::*;
use std::path::PathBuf;

fn main() -> Result<()> {
    // 方法1：使用默认配置
    let config = RwkvAgentKitConfig::default();
    
    // 方法2：从文件加载
    let config = RwkvAgentKitConfig::from_file("config.toml")?;
    
    // 方法3：从环境变量加载
    let config = RwkvAgentKitConfig::from_env()?;
    
    // 方法4：使用配置构建器
    let config = ConfigBuilder::new()
        .add_file("config.toml", ConfigFormat::Toml)
        .add_optional_file("config.local.toml", ConfigFormat::Toml)
        .add_env()
        .env_prefix("RWKV_AGENT")
        .validation(true)
        .build()?;
    
    // 验证配置
    config.validate()?;
    
    // 保存配置
    config.save_to_file("output_config.toml")?;
    
    println!("数据库URL: {}", config.database.url);
    println!("最大记忆数: {:?}", config.memory.max_memories);
    println!("嵌入模型: {}", config.embedding.model_name);
    
    Ok(())
}
```

### 配置热重载示例

```rust
use rwkv_agent_kit::config::*;
use std::sync::{Arc, RwLock};
use std::time::Duration;
use std::thread;

fn main() -> Result<()> {
    // 加载初始配置
    let config = RwkvAgentKitConfig::from_file("config.toml")?;
    
    // 创建配置监控器
    let mut watcher = ConfigWatcher::new(config);
    
    // 监控配置文件
    watcher.watch_file("config.toml")?;
    
    // 添加配置变更回调
    watcher.on_change(|old_config, new_config| {
        println!("配置已更新！");
        
        if old_config.database.url != new_config.database.url {
            println!("数据库URL已更改: {} -> {}", 
                old_config.database.url, new_config.database.url);
        }
        
        if old_config.memory.max_memories != new_config.memory.max_memories {
            println!("最大记忆数已更改: {:?} -> {:?}", 
                old_config.memory.max_memories, new_config.memory.max_memories);
        }
    })?;
    
    // 启动监控
    watcher.start()?;
    
    // 模拟应用运行
    loop {
        let current_config = watcher.get_config();
        
        // 使用当前配置进行操作
        println!("当前数据库URL: {}", current_config.database.url);
        
        thread::sleep(Duration::from_secs(5));
    }
}
```

### 自定义配置验证

```rust
use rwkv_agent_kit::config::*;

fn main() -> Result<()> {
    let config = RwkvAgentKitConfig::from_file("config.toml")?;
    
    // 创建配置验证器
    let validator = ConfigValidator::new()
        .add_rule(ValidationRule::Range {
            field: "database.max_connections".to_string(),
            min: 1.0,
            max: 100.0,
        })
        .add_rule(ValidationRule::Range {
            field: "memory.importance_threshold".to_string(),
            min: 0.0,
            max: 1.0,
        })
        .add_rule(ValidationRule::Required {
            field: "database.url".to_string(),
        })
        .add_rule(ValidationRule::Custom {
            name: "embedding_dimensions_check".to_string(),
            validator: Box::new(|config| {
                if config.embedding.dimensions == 0 {
                    return Err("嵌入维度不能为0".into());
                }
                if config.embedding.dimensions > 4096 {
                    return Err("嵌入维度过大".into());
                }
                Ok(())
            }),
        })
        .strict_mode(true);
    
    // 执行验证
    let report = validator.validate(&config)?;
    
    if !report.is_valid {
        println!("配置验证失败：");
        for error in &report.errors {
            println!("  错误: {} - {}", error.field, error.message);
        }
    }
    
    if !report.warnings.is_empty() {
        println!("配置警告：");
        for warning in &report.warnings {
            println!("  警告: {} - {}", warning.field, warning.message);
            if let Some(suggestion) = &warning.suggestion {
                println!("    建议: {}", suggestion);
            }
        }
    }
    
    Ok(())
}
```

### 配置文件示例

#### config.toml

```toml
[database]
url = "sqlite:rwkv_agent.db"
max_connections = 10
min_connections = 1
connection_timeout = 30
enable_wal = true
enable_foreign_keys = true
cache_size_mb = 64

[database.backup]
enabled = true
interval_hours = 24
retention_days = 30
backup_dir = "./backups"
compress = true

[memory]
max_memories = 100000
importance_threshold = 0.1

[memory.decay]
enabled = true
decay_rate = 0.01
decay_function = "Exponential"
decay_interval_hours = 24
min_importance = 0.01
protection_period_days = 7

[memory.cleanup]
enabled = true
interval_hours = 24
cleanup_on_startup = false
batch_size = 1000

[[memory.cleanup.strategies]]
type = "LowImportance"
threshold = 0.1

[[memory.cleanup.strategies]]
type = "Expired"

[[memory.cleanup.strategies]]
type = "Duplicate"
similarity_threshold = 0.95

[embedding]
provider = "Local"
model_name = "all-MiniLM-L6-v2"
dimensions = 384
batch_size = 32
timeout_seconds = 30

[embedding.retry]
max_attempts = 3
initial_delay_ms = 1000
max_delay_ms = 30000

[embedding.retry.backoff_strategy]
type = "Exponential"
multiplier = 2.0

[tools]
tool_directories = ["./tools"]
enable_builtin_tools = true
execution_timeout_seconds = 300
max_concurrent_tools = 10

[tools.security]
enable_sandbox = true
blocked_tools = []

[tools.security.resource_limits]
max_memory_mb = 512
max_cpu_percent = 80.0
max_execution_time_seconds = 300
max_file_size_mb = 100

[performance]
worker_threads = 4
enable_parallel_processing = true
batch_size = 100

[logging]
level = "info"
format = "json"
output = "stdout"

[security]
enable_encryption = false
enable_authentication = false

[network]
timeout_seconds = 30
max_retries = 3
user_agent = "RWKV-Agent-Kit/1.0"
```

## 下一步

- [核心API](./agent-kit.md)
- [记忆管理API](./memory.md)
- [工具系统API](./tools.md)
- [使用指南](../guide/README.md)
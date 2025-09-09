//! 配置管理模块
//!
//! 本模块负责管理RWKV-Agent-Kit记忆系统的配置选项，包括数据库连接、缓存设置、
//! 性能参数等。支持从环境变量、配置文件等多种方式加载配置。

use crate::error::{MemoryError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Duration;

/// 主配置结构
///
/// 包含系统运行所需的所有配置选项。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Config {
    /// 数据库配置
    pub database: DatabaseConfig,
    /// 缓存配置
    pub cache: CacheConfig,
    /// 向量配置
    pub vector: VectorConfig,
    /// 图数据库配置
    pub graph: GraphConfig,
    /// 性能配置
    pub performance: PerformanceConfig,
    /// 学习配置
    pub learning: LearningConfig,
    /// 检索配置
    pub retrieval: RetrievalConfig,
    /// 安全配置
    pub security: SecurityConfig,
    /// 日志配置
    pub logging: LoggingConfig,
    /// 特性开关
    pub features: FeatureFlags,
    /// 自定义配置
    pub custom: HashMap<String, serde_json::Value>,
}

/// 数据库配置
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DatabaseConfig {
    /// 数据库类型
    pub database_type: DatabaseType,
    /// 连接URL
    pub url: String,
    /// 最大连接数
    pub max_connections: u32,
    /// 最小连接数
    pub min_connections: u32,
    /// 连接超时时间（秒）
    pub connect_timeout: u64,
    /// 空闲超时时间（秒）
    pub idle_timeout: u64,
    /// 最大生命周期（秒）
    pub max_lifetime: u64,
    /// 是否启用SSL
    pub enable_ssl: bool,
    /// SSL证书路径
    pub ssl_cert_path: Option<PathBuf>,
    /// 数据库名称
    pub database_name: String,
    /// 表前缀
    pub table_prefix: String,
    /// 是否自动迁移
    pub auto_migrate: bool,
    /// 备份配置
    pub backup: BackupConfig,
}

/// 数据库类型枚举
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DatabaseType {
    SQLite,
}

/// 备份配置
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BackupConfig {
    /// 是否启用自动备份
    pub enabled: bool,
    /// 备份间隔（小时）
    pub interval_hours: u64,
    /// 备份保留天数
    pub retention_days: u64,
    /// 备份存储路径
    pub backup_path: PathBuf,
    /// 压缩备份
    pub compress: bool,
}

/// 缓存配置
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CacheConfig {
    /// 是否启用缓存
    pub enabled: bool,
    /// 缓存类型
    pub cache_type: CacheType,
    /// 最大缓存大小（MB）
    pub max_size_mb: u64,
    /// 缓存TTL（秒）
    pub ttl_seconds: u64,
    /// LRU缓存大小
    pub lru_capacity: usize,
    /// Redis配置（如果使用Redis缓存）
    pub redis: Option<RedisConfig>,
    /// 预热配置
    pub warmup: WarmupConfig,
}

/// 缓存类型枚举
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CacheType {
    Memory,
    Redis,
    Hybrid,
}

/// Redis配置
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RedisConfig {
    /// Redis URL
    pub url: String,
    /// 连接池大小
    pub pool_size: u32,
    /// 连接超时（毫秒）
    pub connect_timeout_ms: u64,
    /// 命令超时（毫秒）
    pub command_timeout_ms: u64,
    /// 密码
    pub password: Option<String>,
    /// 数据库编号
    pub database: u8,
}

/// 预热配置
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WarmupConfig {
    /// 是否启用预热
    pub enabled: bool,
    /// 预热记忆数量
    pub memory_count: usize,
    /// 预热策略
    pub strategy: WarmupStrategy,
}

/// 预热策略枚举
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum WarmupStrategy {
    /// 最近访问
    RecentlyAccessed,
    /// 高重要性
    HighImportance,
    /// 高频访问
    HighFrequency,
    /// 混合策略
    Mixed,
}

/// 向量配置
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VectorConfig {
    /// 向量维度
    pub dimension: usize,
    /// 相似度阈值
    pub similarity_threshold: f32,
    /// 索引类型
    pub index_type: VectorIndexType,
    /// 距离度量
    pub distance_metric: DistanceMetric,
    /// 索引参数
    pub index_params: IndexParams,
    /// 搜索参数
    pub search_params: SearchParams,
}

/// 向量索引类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum VectorIndexType {
    /// 平坦索引
    Flat,
    /// IVF索引
    IVF,
    /// HNSW索引
    HNSW,
    /// LSH索引
    LSH,
}

/// 距离度量
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DistanceMetric {
    /// 余弦相似度
    Cosine,
    /// 欧几里得距离
    Euclidean,
    /// 曼哈顿距离
    Manhattan,
    /// 点积
    DotProduct,
}

/// 索引参数
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IndexParams {
    /// 聚类数量（IVF）
    pub nlist: Option<usize>,
    /// 连接数（HNSW）
    pub m: Option<usize>,
    /// 构建时的候选数（HNSW）
    pub ef_construction: Option<usize>,
    /// 哈希表数量（LSH）
    pub num_tables: Option<usize>,
    /// 哈希函数数量（LSH）
    pub num_hash_funcs: Option<usize>,
}

/// 搜索参数
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SearchParams {
    /// 搜索的聚类数量（IVF）
    pub nprobe: Option<usize>,
    /// 搜索时的候选数（HNSW）
    pub ef_search: Option<usize>,
    /// 最大搜索结果数
    pub max_results: usize,
    /// 搜索超时（毫秒）
    pub timeout_ms: u64,
}

/// 图配置
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GraphConfig {
    /// 最大连接数
    pub max_connections_per_node: usize,
    /// 连接强度阈值
    pub connection_threshold: f32,
    /// PageRank参数
    pub pagerank: PageRankConfig,
    /// 图遍历配置
    pub traversal: TraversalConfig,
    /// 图压缩配置
    pub compression: CompressionConfig,
}

/// PageRank配置
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PageRankConfig {
    /// 阻尼因子
    pub damping_factor: f32,
    /// 最大迭代次数
    pub max_iterations: usize,
    /// 收敛阈值
    pub convergence_threshold: f32,
    /// 个性化向量权重
    pub personalization_weight: f32,
}

/// 图遍历配置
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TraversalConfig {
    /// 最大遍历深度
    pub max_depth: usize,
    /// 最大访问节点数
    pub max_visited_nodes: usize,
    /// 遍历超时（毫秒）
    pub timeout_ms: u64,
    /// 是否启用循环检测
    pub cycle_detection: bool,
}

/// 图压缩配置
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CompressionConfig {
    /// 是否启用压缩
    pub enabled: bool,
    /// 弱连接清理阈值
    pub weak_connection_threshold: f32,
    /// 压缩间隔（小时）
    pub compression_interval_hours: u64,
    /// 保留最小连接数
    pub min_connections_to_keep: usize,
}

/// 性能配置
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PerformanceConfig {
    /// 工作线程数
    pub worker_threads: usize,
    /// 批处理大小
    pub batch_size: usize,
    /// 查询超时（毫秒）
    pub query_timeout_ms: u64,
    /// 创建超时（毫秒）
    pub creation_timeout_ms: u64,
    /// 并发限制
    pub concurrency_limit: usize,
    /// 内存限制（MB）
    pub memory_limit_mb: u64,
    /// 是否启用性能监控
    pub enable_metrics: bool,
    /// 指标收集间隔（秒）
    pub metrics_interval_seconds: u64,
}

/// 检索配置
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RetrievalConfig {
    /// 融合权重配置
    pub fusion_weights: FusionWeights,
    /// 最大检索结果数
    pub max_results: usize,
    /// 检索超时（毫秒）
    pub timeout_ms: u64,
    /// 是否启用缓存
    pub enable_cache: bool,
    /// 缓存TTL（秒）
    pub cache_ttl_seconds: u64,
}

/// 融合权重配置
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FusionWeights {
    /// 语义权重
    pub semantic_weight: f32,
    /// 时间权重
    pub temporal_weight: f32,
    /// 结构权重
    pub structural_weight: f32,
    /// 重要性权重
    pub importance_weight: f32,
    /// 个性化权重
    pub personalization_weight: f32,
}

/// 学习配置
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LearningConfig {
    /// 是否启用学习
    pub enabled: bool,
    /// 学习率
    pub learning_rate: f32,
    /// 重要性衰减因子
    pub importance_decay_factor: f32,
    /// 连接强度衰减因子
    pub connection_decay_factor: f32,
    /// 学习间隔（小时）
    pub learning_interval_hours: u64,
    /// 最小学习样本数
    pub min_learning_samples: usize,
    /// 最大交互历史记录数
    pub max_interaction_history: usize,
    /// 个性化学习配置
    pub personalization: PersonalizationConfig,
}

/// 个性化学习配置
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PersonalizationConfig {
    /// 是否启用个性化
    pub enabled: bool,
    /// 用户行为权重
    pub user_behavior_weight: f32,
    /// 时间衰减权重
    pub temporal_decay_weight: f32,
    /// 反馈权重
    pub feedback_weight: f32,
    /// 个性化更新间隔（小时）
    pub update_interval_hours: u64,
}

/// 安全配置
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SecurityConfig {
    /// 是否启用认证
    pub enable_auth: bool,
    /// JWT密钥
    pub jwt_secret: Option<String>,
    /// Token过期时间（小时）
    pub token_expiry_hours: u64,
    /// 是否启用加密
    pub enable_encryption: bool,
    /// 加密密钥
    pub encryption_key: Option<String>,
    /// 访问控制配置
    pub access_control: AccessControlConfig,
    /// 审计配置
    pub audit: AuditConfig,
}

/// 访问控制配置
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AccessControlConfig {
    /// 是否启用访问控制
    pub enabled: bool,
    /// 默认权限
    pub default_permissions: Vec<String>,
    /// 管理员用户列表
    pub admin_users: Vec<String>,
    /// 只读用户列表
    pub readonly_users: Vec<String>,
}

/// 审计配置
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AuditConfig {
    /// 是否启用审计
    pub enabled: bool,
    /// 审计日志路径
    pub log_path: PathBuf,
    /// 审计事件类型
    pub event_types: Vec<String>,
    /// 日志保留天数
    pub retention_days: u64,
}

/// 日志配置
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LoggingConfig {
    /// 日志级别
    pub level: LogLevel,
    /// 日志格式
    pub format: LogFormat,
    /// 日志输出目标
    pub targets: Vec<LogTarget>,
    /// 日志文件路径
    pub file_path: Option<PathBuf>,
    /// 日志文件最大大小（MB）
    pub max_file_size_mb: u64,
    /// 日志文件保留数量
    pub max_files: u32,
    /// 是否启用结构化日志
    pub structured: bool,
}

/// 日志级别
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

/// 日志格式
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum LogFormat {
    Plain,
    Json,
    Compact,
}

/// 日志输出目标
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum LogTarget {
    Console,
    File,
    Syslog,
}

/// 特性开关
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct FeatureFlags {
    /// 是否启用实验性特性
    pub experimental_features: bool,
    /// 是否启用调试模式
    pub debug_mode: bool,
    /// 是否启用性能分析
    pub profiling: bool,
    /// 是否启用A/B测试
    pub ab_testing: bool,
    /// 自定义特性开关
    pub custom_flags: HashMap<String, bool>,
}

/// 配置构建器
#[derive(Debug, Default)]
pub struct ConfigBuilder {
    config: Config,
}

impl ConfigBuilder {
    /// 创建新的配置构建器
    pub fn new() -> Self {
        Self {
            config: Config::default(),
        }
    }

    /// 设置数据库配置
    pub fn database(mut self, database: DatabaseConfig) -> Self {
        self.config.database = database;
        self
    }

    /// 设置缓存配置
    pub fn cache(mut self, cache: CacheConfig) -> Self {
        self.config.cache = cache;
        self
    }

    /// 设置向量配置
    pub fn vector(mut self, vector: VectorConfig) -> Self {
        self.config.vector = vector;
        self
    }

    /// 设置图配置
    pub fn graph(mut self, graph: GraphConfig) -> Self {
        self.config.graph = graph;
        self
    }

    /// 设置性能配置
    pub fn performance(mut self, performance: PerformanceConfig) -> Self {
        self.config.performance = performance;
        self
    }

    /// 设置学习配置
    pub fn learning(mut self, learning: LearningConfig) -> Self {
        self.config.learning = learning;
        self
    }

    /// 设置安全配置
    pub fn security(mut self, security: SecurityConfig) -> Self {
        self.config.security = security;
        self
    }

    /// 设置日志配置
    pub fn logging(mut self, logging: LoggingConfig) -> Self {
        self.config.logging = logging;
        self
    }

    /// 设置特性开关
    pub fn features(mut self, features: FeatureFlags) -> Self {
        self.config.features = features;
        self
    }

    /// 构建配置
    pub fn build(self) -> Config {
        self.config
    }
}

impl Config {
    /// 从环境变量加载配置
    pub fn from_env() -> Result<Self> {
        let mut config = Self::default();

        // 数据库URL
        if let Ok(url) = std::env::var("AI00_MEM_DATABASE_URL") {
            config.database.url = url;
        }

        // 数据库类型
        if let Ok(db_type) = std::env::var("AI00_MEM_DATABASE_TYPE") {
            config.database.database_type = match db_type.to_lowercase().as_str() {
                "sqlite" => DatabaseType::SQLite,
                _ => {
                    return Err(MemoryError::validation_error(format!(
                        "Unsupported database type: {}",
                        db_type
                    )))
                }
            };
        }

        // 缓存配置
        if let Ok(cache_enabled) = std::env::var("AI00_MEM_CACHE_ENABLED") {
            config.cache.enabled = cache_enabled.parse().unwrap_or(true);
        }

        if let Ok(cache_size) = std::env::var("AI00_MEM_CACHE_SIZE_MB") {
            config.cache.max_size_mb = cache_size.parse().unwrap_or(256);
        }

        // 向量配置
        if let Ok(dimension) = std::env::var("AI00_MEM_VECTOR_DIMENSION") {
            config.vector.dimension = dimension.parse().unwrap_or(256);
        }

        // 性能配置
        if let Ok(workers) = std::env::var("AI00_MEM_WORKER_THREADS") {
            config.performance.worker_threads = workers.parse().unwrap_or(4);
        }

        // 日志级别
        if let Ok(log_level) = std::env::var("AI00_MEM_LOG_LEVEL") {
            config.logging.level = match log_level.to_lowercase().as_str() {
                "error" => LogLevel::Error,
                "warn" => LogLevel::Warn,
                "info" => LogLevel::Info,
                "debug" => LogLevel::Debug,
                "trace" => LogLevel::Trace,
                _ => LogLevel::Info,
            };
        }

        Ok(config)
    }

    /// 从配置文件加载配置
    pub fn from_file<P: AsRef<std::path::Path>>(path: P) -> Result<Self> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| MemoryError::validation_error(e.to_string()))?;

        let config: Config = toml::from_str(&content)
            .or_else(|_| serde_json::from_str(&content))
            .or_else(|_| serde_yaml::from_str(&content))
            .map_err(|e| {
                MemoryError::validation_error(format!("Failed to parse configuration file: {}", e))
            })?;

        Ok(config)
    }

    /// 验证配置
    pub fn validate(&self) -> Result<()> {
        // 验证数据库配置
        if self.database.url.is_empty() {
            return Err(MemoryError::validation_error(
                "Database URL cannot be empty".to_string(),
            ));
        }

        if self.database.max_connections == 0 {
            return Err(MemoryError::validation_error(
                "Max connections must be greater than 0".to_string(),
            ));
        }

        if self.database.min_connections > self.database.max_connections {
            return Err(MemoryError::validation_error(
                "Min connections cannot be greater than max connections".to_string(),
            ));
        }

        // 验证向量配置
        if self.vector.dimension == 0 {
            return Err(MemoryError::validation_error(
                "Vector dimension must be greater than 0".to_string(),
            ));
        }

        if !(0.0..=1.0).contains(&self.vector.similarity_threshold) {
            return Err(MemoryError::validation_error(
                "Similarity threshold must be between 0.0 and 1.0".to_string(),
            ));
        }

        // 验证图配置
        if self.graph.max_connections_per_node == 0 {
            return Err(MemoryError::validation_error(
                "Max connections per node must be greater than 0".to_string(),
            ));
        }

        if !(0.0..=1.0).contains(&self.graph.connection_threshold) {
            return Err(MemoryError::validation_error(
                "Connection threshold must be between 0.0 and 1.0".to_string(),
            ));
        }

        // 验证PageRank配置
        if !(0.0..=1.0).contains(&self.graph.pagerank.damping_factor) {
            return Err(MemoryError::validation_error(
                "PageRank damping factor must be between 0.0 and 1.0".to_string(),
            ));
        }

        // 验证性能配置
        if self.performance.worker_threads == 0 {
            return Err(MemoryError::validation_error(
                "Worker threads must be greater than 0".to_string(),
            ));
        }

        if self.performance.batch_size == 0 {
            return Err(MemoryError::validation_error(
                "Batch size must be greater than 0".to_string(),
            ));
        }

        Ok(())
    }

    /// 获取数据库连接超时时间
    pub fn database_connect_timeout(&self) -> Duration {
        Duration::from_secs(self.database.connect_timeout)
    }

    /// 获取查询超时时间
    pub fn query_timeout(&self) -> Duration {
        Duration::from_millis(self.performance.query_timeout_ms)
    }

    /// 获取缓存TTL
    pub fn cache_ttl(&self) -> Duration {
        Duration::from_secs(self.cache.ttl_seconds)
    }
}

// 默认实现

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            database_type: DatabaseType::SQLite,
            url: "sqlite://ai00_mem.db".to_string(),
            max_connections: 10,
            min_connections: 1,
            connect_timeout: 30,
            idle_timeout: 600,
            max_lifetime: 3600,
            enable_ssl: false,
            ssl_cert_path: None,
            database_name: "ai00_mem".to_string(),
            table_prefix: "ai00_".to_string(),
            auto_migrate: true,
            backup: BackupConfig::default(),
        }
    }
}

impl Default for BackupConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            interval_hours: 24,
            retention_days: 30,
            backup_path: PathBuf::from("./backups"),
            compress: true,
        }
    }
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            cache_type: CacheType::Memory,
            max_size_mb: 256,
            ttl_seconds: 3600,
            lru_capacity: 10000,
            redis: None,
            warmup: WarmupConfig::default(),
        }
    }
}

impl Default for WarmupConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            memory_count: 1000,
            strategy: WarmupStrategy::Mixed,
        }
    }
}

impl Default for VectorConfig {
    fn default() -> Self {
        Self {
            dimension: 256, // 标准BERT模型输出维度
            similarity_threshold: 0.7,
            index_type: VectorIndexType::Flat,
            distance_metric: DistanceMetric::Cosine,
            index_params: IndexParams::default(),
            search_params: SearchParams::default(),
        }
    }
}

impl Default for IndexParams {
    fn default() -> Self {
        Self {
            nlist: Some(100),
            m: Some(16),
            ef_construction: Some(200),
            num_tables: Some(10),
            num_hash_funcs: Some(4),
        }
    }
}

impl Default for SearchParams {
    fn default() -> Self {
        Self {
            nprobe: Some(10),
            ef_search: Some(50),
            max_results: 100,
            timeout_ms: 5000,
        }
    }
}

impl Default for GraphConfig {
    fn default() -> Self {
        Self {
            max_connections_per_node: 50,
            connection_threshold: 0.5,
            pagerank: PageRankConfig::default(),
            traversal: TraversalConfig::default(),
            compression: CompressionConfig::default(),
        }
    }
}

impl Default for PageRankConfig {
    fn default() -> Self {
        Self {
            damping_factor: 0.85,
            max_iterations: 100,
            convergence_threshold: 1e-6,
            personalization_weight: 0.15,
        }
    }
}

impl Default for TraversalConfig {
    fn default() -> Self {
        Self {
            max_depth: 5,
            max_visited_nodes: 1000,
            timeout_ms: 10000,
            cycle_detection: true,
        }
    }
}

impl Default for CompressionConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            weak_connection_threshold: 0.1,
            compression_interval_hours: 24,
            min_connections_to_keep: 5,
        }
    }
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            worker_threads: num_cpus::get(),
            batch_size: 100,
            query_timeout_ms: 5000,
            creation_timeout_ms: 10000,
            concurrency_limit: 100,
            memory_limit_mb: 1024,
            enable_metrics: true,
            metrics_interval_seconds: 60,
        }
    }
}

impl Default for LearningConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            learning_rate: 0.01,
            importance_decay_factor: 0.95,
            connection_decay_factor: 0.98,
            learning_interval_hours: 1,
            min_learning_samples: 10,
            max_interaction_history: 1000,
            personalization: PersonalizationConfig::default(),
        }
    }
}

impl Default for RetrievalConfig {
    fn default() -> Self {
        Self {
            fusion_weights: FusionWeights::default(),
            max_results: 100,
            timeout_ms: 5000,
            enable_cache: true,
            cache_ttl_seconds: 3600,
        }
    }
}

impl Default for FusionWeights {
    fn default() -> Self {
        Self {
            semantic_weight: 0.4,
            temporal_weight: 0.2,
            structural_weight: 0.2,
            importance_weight: 0.1,
            personalization_weight: 0.1,
        }
    }
}

impl Default for PersonalizationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            user_behavior_weight: 0.4,
            temporal_decay_weight: 0.3,
            feedback_weight: 0.3,
            update_interval_hours: 6,
        }
    }
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            enable_auth: false,
            jwt_secret: None,
            token_expiry_hours: 24,
            enable_encryption: false,
            encryption_key: None,
            access_control: AccessControlConfig::default(),
            audit: AuditConfig::default(),
        }
    }
}

impl Default for AccessControlConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            default_permissions: vec!["read".to_string(), "write".to_string()],
            admin_users: Vec::new(),
            readonly_users: Vec::new(),
        }
    }
}

impl Default for AuditConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            log_path: PathBuf::from("./logs/audit.log"),
            event_types: vec![
                "create".to_string(),
                "update".to_string(),
                "delete".to_string(),
                "query".to_string(),
            ],
            retention_days: 90,
        }
    }
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: LogLevel::Info,
            format: LogFormat::Plain,
            targets: vec![LogTarget::Console],
            file_path: None,
            max_file_size_mb: 100,
            max_files: 10,
            structured: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert!(config.validate().is_ok());
        assert_eq!(config.database.database_type, DatabaseType::SQLite);
        assert_eq!(config.vector.dimension, 256);
        assert!(config.cache.enabled);
    }

    #[test]
    fn test_config_builder() {
        let config = ConfigBuilder::new()
            .database(DatabaseConfig {
                database_type: DatabaseType::SQLite,
                url: "sqlite://test.db".to_string(),
                ..Default::default()
            })
            .vector(VectorConfig {
                dimension: 1024,
                ..Default::default()
            })
            .build();

        assert_eq!(config.database.database_type, DatabaseType::SQLite);
        assert_eq!(config.vector.dimension, 1024);
    }

    #[test]
    fn test_config_validation() {
        let mut config = Config::default();

        // 测试无效的数据库URL
        config.database.url = String::new();
        assert!(config.validate().is_err());

        // 测试无效的向量维度
        config.database.url = "sqlite://test.db".to_string();
        config.vector.dimension = 0;
        assert!(config.validate().is_err());

        // 测试有效配置
        config.vector.dimension = 256;
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_config_from_env() {
        std::env::set_var("AI00_MEM_DATABASE_URL", "postgresql://localhost/test");
        std::env::set_var("AI00_MEM_DATABASE_TYPE", "sqlite");
        std::env::set_var("AI00_MEM_VECTOR_DIMENSION", "1024");

        let config = Config::from_env().unwrap();
        assert_eq!(config.database.url, "postgresql://localhost/test");
        assert_eq!(config.database.database_type, DatabaseType::SQLite);
        assert_eq!(config.vector.dimension, 1024);

        // 清理环境变量
        std::env::remove_var("AI00_MEM_DATABASE_URL");
        std::env::remove_var("AI00_MEM_DATABASE_TYPE");
        std::env::remove_var("AI00_MEM_VECTOR_DIMENSION");
    }

    #[test]
    fn test_config_from_file() {
        let config_content = r#"
[database]
database_type = "SQLite"
url = "sqlite://test.db"
max_connections = 20
min_connections = 1
connect_timeout = 30
idle_timeout = 600
max_lifetime = 3600
enable_ssl = false
database_name = "test_db"
table_prefix = "test_"
auto_migrate = true

[database.backup]
enabled = false
interval_hours = 24
retention_days = 30
backup_path = "./backups"
compress = true

[vector]
dimension = 1024
similarity_threshold = 0.8
index_type = "Flat"
distance_metric = "Cosine"

[vector.index_params]

[vector.search_params]
ef_search = 50
max_results = 100
timeout_ms = 5000

[cache]
enabled = true
cache_type = "Memory"
max_size_mb = 512
ttl_seconds = 3600
lru_capacity = 10000

[cache.warmup]
enabled = true
memory_count = 1000
strategy = "Mixed"

[graph]
max_connections_per_node = 50
connection_threshold = 0.5

[graph.pagerank]
damping_factor = 0.85
max_iterations = 100
convergence_threshold = 0.0001
personalization_weight = 0.1

[graph.traversal]
max_depth = 5
max_visited_nodes = 1000
timeout_ms = 5000
cycle_detection = true

[graph.compression]
enabled = true
weak_connection_threshold = 0.1
compression_interval_hours = 24
min_connections_to_keep = 5

[performance]
worker_threads = 4
batch_size = 100
query_timeout_ms = 5000
creation_timeout_ms = 10000
concurrency_limit = 100
memory_limit_mb = 1024
enable_metrics = true
metrics_interval_seconds = 60

[learning]
enabled = true
learning_rate = 0.01
importance_decay_factor = 0.95
connection_decay_factor = 0.9
learning_interval_hours = 6
min_learning_samples = 10
max_interaction_history = 1000

[learning.personalization]
enabled = true
user_behavior_weight = 0.4
temporal_decay_weight = 0.3
feedback_weight = 0.3
update_interval_hours = 6

[retrieval]
max_results = 100
timeout_ms = 5000
enable_cache = true
cache_ttl_seconds = 3600

[retrieval.fusion_weights]
semantic_weight = 0.4
temporal_weight = 0.2
structural_weight = 0.2
importance_weight = 0.1
personalization_weight = 0.1

[security]
enable_auth = false
token_expiry_hours = 24
enable_encryption = false

[security.access_control]
enabled = false
default_permissions = ["read", "write"]
admin_users = []
readonly_users = []

[security.audit]
enabled = false
log_path = "./logs/audit.log"
event_types = ["create", "update", "delete", "query"]
retention_days = 90

[logging]
level = "Info"
format = "Plain"
targets = ["Console"]
max_file_size_mb = 100
max_files = 10
structured = false

[features]
experimental_features = false
debug_mode = false
profiling = false
ab_testing = false

[features.custom_flags]

[custom]
"#;

        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(config_content.as_bytes()).unwrap();

        let config = Config::from_file(temp_file.path()).unwrap();
        assert_eq!(config.database.database_type, DatabaseType::SQLite);
        assert_eq!(config.database.max_connections, 20);
        assert_eq!(config.vector.dimension, 1024);
        assert_eq!(config.cache.max_size_mb, 512);
    }

    #[test]
    fn test_timeout_conversions() {
        let config = Config::default();

        let db_timeout = config.database_connect_timeout();
        assert_eq!(db_timeout, Duration::from_secs(30));

        let query_timeout = config.query_timeout();
        assert_eq!(query_timeout, Duration::from_millis(5000));

        let cache_ttl = config.cache_ttl();
        assert_eq!(cache_ttl, Duration::from_secs(3600));
    }
}

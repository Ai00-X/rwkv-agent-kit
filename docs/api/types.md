---
title: types
createTime: 2025/09/08 15:25:42
permalink: /article/jjp7bjqs/
---
# 核心类型定义

## 概述

本文档详细描述了RWKV Agent Kit中使用的核心数据类型和结构。这些类型构成了系统的基础，用于记忆管理、工具系统、配置管理等各个模块。

## 基础类型

### Result&lt;T&gt;

系统中所有可能失败的操作都返回`Result<T>`类型。

```rust
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
```

### Uuid

使用UUID作为唯一标识符。

```rust
use uuid::Uuid;

// 生成新的UUID
let id = Uuid::new_v4();
```

### DateTime

时间戳使用UTC时间。

```rust
use chrono::{DateTime, Utc};

let now = Utc::now();
```

## 记忆相关类型

### Memory

记忆对象是系统的核心数据结构。

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Memory {
    /// 唯一标识符
    pub id: Uuid,
    /// 记忆内容
    pub content: String,
    /// 记忆类型
    pub memory_type: MemoryType,
    /// 嵌入向量
    pub embedding: Vec<f32>,
    /// 记忆属性
    pub attributes: MemoryAttributes,
    /// 元数据
    pub metadata: MemoryMetadata,
    /// 连接信息
    pub connections: MemoryConnections,
}

impl Memory {
    /// 创建新记忆
    pub fn new(
        content: String,
        memory_type: MemoryType,
        embedding: Vec<f32>,
        attributes: MemoryAttributes,
    ) -> Self;
    
    /// 更新重要性
    pub fn update_importance(&mut self, importance: f32);
    
    /// 添加关键词
    pub fn add_keyword(&mut self, keyword: String);
    
    /// 添加标签
    pub fn add_tag(&mut self, tag: String);
    
    /// 检查相似度
    pub fn similarity_to(&self, other: &Memory) -> f32;
    
    /// 计算年龄（天数）
    pub fn age_in_days(&self) -> i64;
    
    /// 是否过期
    pub fn is_expired(&self, ttl_days: i64) -> bool;
}
```

### MemoryType

记忆类型枚举定义了不同种类的记忆。

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MemoryType {
    /// 用户个人信息
    UserProfile,
    /// 用户偏好设置
    UserPreference,
    /// 对话历史记录
    Conversation,
    /// 知识信息
    Knowledge,
    /// 任务相关信息
    Task,
    /// 情感记忆
    Emotional,
    /// 程序性记忆（技能、方法）
    Procedural,
    /// 情景记忆（事件、经历）
    Episodic,
    /// 语义记忆（概念、事实）
    Semantic,
    /// 工作记忆（临时信息）
    Working,
    /// 自定义类型
    Custom(String),
}

impl MemoryType {
    /// 获取默认重要性
    pub fn default_importance(&self) -> f32 {
        match self {
            MemoryType::UserProfile => 0.9,
            MemoryType::UserPreference => 0.8,
            MemoryType::Knowledge => 0.7,
            MemoryType::Conversation => 0.5,
            MemoryType::Task => 0.6,
            MemoryType::Emotional => 0.7,
            MemoryType::Procedural => 0.8,
            MemoryType::Episodic => 0.6,
            MemoryType::Semantic => 0.7,
            MemoryType::Working => 0.3,
            MemoryType::Custom(_) => 0.5,
        }
    }
    
    /// 获取默认TTL（天数）
    pub fn default_ttl_days(&self) -> Option<i64> {
        match self {
            MemoryType::Working => Some(1),
            MemoryType::Conversation => Some(30),
            MemoryType::Task => Some(7),
            _ => None, // 永久保存
        }
    }
}
```

### MemoryAttributes

记忆属性包含了记忆的各种特征和元信息。

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryAttributes {
    /// 关键词列表
    pub keywords: Vec<String>,
    /// 标签列表
    pub tags: Vec<String>,
    /// 重要性分数 (0.0-1.0)
    pub importance: f32,
    /// 情感倾向 (-1.0到1.0，负数表示负面，正数表示正面)
    pub emotional_valence: f32,
    /// 情感强度 (0.0-1.0)
    pub emotional_intensity: f32,
    /// 置信度 (0.0-1.0)
    pub confidence: f32,
    /// 访问频率
    pub access_frequency: u32,
    /// 最后访问时间
    pub last_accessed: DateTime<Utc>,
    /// 创建时间
    pub created_at: DateTime<Utc>,
    /// 更新时间
    pub updated_at: DateTime<Utc>,
    /// 过期时间（可选）
    pub expires_at: Option<DateTime<Utc>>,
    /// 来源信息
    pub source: Option<String>,
    /// 语言代码
    pub language: Option<String>,
    /// 自定义属性
    pub custom_attributes: HashMap<String, Value>,
}

impl Default for MemoryAttributes {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            keywords: Vec::new(),
            tags: Vec::new(),
            importance: 0.5,
            emotional_valence: 0.0,
            emotional_intensity: 0.0,
            confidence: 1.0,
            access_frequency: 0,
            last_accessed: now,
            created_at: now,
            updated_at: now,
            expires_at: None,
            source: None,
            language: None,
            custom_attributes: HashMap::new(),
        }
    }
}
```

### MemoryMetadata

记忆元数据包含系统级信息。

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryMetadata {
    /// 版本号
    pub version: u32,
    /// 校验和
    pub checksum: String,
    /// 压缩信息
    pub compression: Option<CompressionInfo>,
    /// 加密信息
    pub encryption: Option<EncryptionInfo>,
    /// 存储位置
    pub storage_location: Option<String>,
    /// 备份信息
    pub backup_info: Option<BackupInfo>,
}

impl Default for MemoryMetadata {
    fn default() -> Self {
        Self {
            version: 1,
            checksum: String::new(),
            compression: None,
            encryption: None,
            storage_location: None,
            backup_info: None,
        }
    }
}
```

### MemoryConnection

记忆连接表示记忆之间的关系。

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryConnection {
    /// 源记忆ID
    pub from_memory_id: Uuid,
    /// 目标记忆ID
    pub to_memory_id: Uuid,
    /// 连接类型
    pub connection_type: ConnectionType,
    /// 连接强度 (0.0-1.0)
    pub strength: f32,
    /// 连接方向
    pub direction: ConnectionDirection,
    /// 连接元数据
    pub metadata: ConnectionMetadata,
}

impl MemoryConnection {
    /// 创建新连接
    pub fn new(
        from_id: Uuid,
        to_id: Uuid,
        connection_type: ConnectionType,
        strength: f32,
    ) -> Self;
    
    /// 更新强度
    pub fn update_strength(&mut self, new_strength: f32);
    
    /// 是否为强连接
    pub fn is_strong(&self, threshold: f32) -> bool;
}
```

### ConnectionType

连接类型定义了记忆之间关系的性质。

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConnectionType {
    /// 语义相似
    Semantic,
    /// 时间相关
    Temporal,
    /// 因果关系
    Causal,
    /// 层次关系
    Hierarchical,
    /// 关联关系
    Associative,
    /// 对比关系
    Contrastive,
    /// 补充关系
    Complementary,
    /// 依赖关系
    Dependency,
    /// 引用关系
    Reference,
    /// 自定义关系
    Custom(String),
}

impl ConnectionType {
    /// 获取默认强度
    pub fn default_strength(&self) -> f32 {
        match self {
            ConnectionType::Semantic => 0.7,
            ConnectionType::Temporal => 0.5,
            ConnectionType::Causal => 0.8,
            ConnectionType::Hierarchical => 0.9,
            ConnectionType::Associative => 0.6,
            ConnectionType::Contrastive => 0.5,
            ConnectionType::Complementary => 0.7,
            ConnectionType::Dependency => 0.8,
            ConnectionType::Reference => 0.6,
            ConnectionType::Custom(_) => 0.5,
        }
    }
}
```

### ConnectionDirection

连接方向枚举。

```rust
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConnectionDirection {
    /// 单向连接（从源到目标）
    Unidirectional,
    /// 双向连接
    Bidirectional,
}
```

## 查询相关类型

### Query

查询对象用于记忆检索。

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Query {
    /// 查询文本
    pub text: String,
    /// 查询类型
    pub query_type: QueryType,
    /// 过滤条件
    pub filters: QueryFilters,
    /// 结果限制
    pub limit: Option<usize>,
    /// 偏移量
    pub offset: Option<usize>,
    /// 排序方式
    pub sort_by: Option<SortBy>,
    /// 权重配置
    pub weights: QueryWeights,
    /// 上下文信息
    pub context: Option<Context>,
}

impl Query {
    /// 创建简单查询
    pub fn simple(text: &str) -> Self;
    
    /// 创建语义查询
    pub fn semantic(text: &str) -> Self;
    
    /// 创建关键词查询
    pub fn keyword(keywords: Vec<String>) -> Self;
    
    /// 添加过滤器
    pub fn with_filter(mut self, filter: QueryFilter) -> Self;
    
    /// 设置限制
    pub fn with_limit(mut self, limit: usize) -> Self;
}
```

### QueryType

查询类型枚举。

```rust
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum QueryType {
    /// 语义搜索
    Semantic,
    /// 关键词搜索
    Keyword,
    /// 混合搜索
    Hybrid,
    /// 图遍历搜索
    Graph,
    /// 模糊搜索
    Fuzzy,
    /// 正则表达式搜索
    Regex,
}
```

### QueryFilters

查询过滤器。

```rust
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QueryFilters {
    /// 记忆类型过滤
    pub memory_types: Option<Vec<MemoryType>>,
    /// 重要性范围
    pub importance_range: Option<(f32, f32)>,
    /// 时间范围
    pub date_range: Option<(DateTime<Utc>, DateTime<Utc>)>,
    /// 标签过滤
    pub tags: Option<Vec<String>>,
    /// 关键词过滤
    pub keywords: Option<Vec<String>>,
    /// 情感倾向范围
    pub emotional_valence_range: Option<(f32, f32)>,
    /// 置信度范围
    pub confidence_range: Option<(f32, f32)>,
    /// 语言过滤
    pub languages: Option<Vec<String>>,
    /// 来源过滤
    pub sources: Option<Vec<String>>,
    /// 自定义过滤器
    pub custom_filters: HashMap<String, Value>,
}
```

### QueryWeights

查询权重配置。

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryWeights {
    /// 语义相似度权重
    pub semantic_weight: f32,
    /// 关键词匹配权重
    pub keyword_weight: f32,
    /// 时间新近度权重
    pub recency_weight: f32,
    /// 重要性权重
    pub importance_weight: f32,
    /// 访问频率权重
    pub frequency_weight: f32,
    /// 情感相关性权重
    pub emotional_weight: f32,
}

impl Default for QueryWeights {
    fn default() -> Self {
        Self {
            semantic_weight: 0.4,
            keyword_weight: 0.3,
            recency_weight: 0.1,
            importance_weight: 0.1,
            frequency_weight: 0.05,
            emotional_weight: 0.05,
        }
    }
}
```

### SortBy

排序方式枚举。

```rust
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SortBy {
    /// 按相关性排序
    Relevance,
    /// 按重要性排序
    Importance,
    /// 按时间排序
    CreatedAt,
    /// 按更新时间排序
    UpdatedAt,
    /// 按访问时间排序
    LastAccessed,
    /// 按访问频率排序
    AccessFrequency,
    /// 自定义排序
    Custom(String),
}
```

## 上下文类型

### Context

上下文信息用于提供查询和操作的背景。

```rust
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Context {
    /// 用户ID
    pub user_id: Option<String>,
    /// 会话ID
    pub session_id: Option<String>,
    /// 当前任务
    pub current_task: Option<String>,
    /// 用户意图
    pub user_intent: Option<UserIntent>,
    /// 情感状态
    pub emotional_state: Option<EmotionalState>,
    /// 环境信息
    pub environment: EnvironmentInfo,
    /// 时间上下文
    pub temporal_context: TemporalContext,
    /// 自定义上下文
    pub custom_context: HashMap<String, Value>,
}
```

### UserIntent

用户意图枚举。

```rust
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UserIntent {
    /// 信息查询
    InformationSeeking,
    /// 任务执行
    TaskExecution,
    /// 学习探索
    Learning,
    /// 创作生成
    Creation,
    /// 问题解决
    ProblemSolving,
    /// 娱乐互动
    Entertainment,
    /// 自定义意图
    Custom(String),
}
```

### EmotionalState

情感状态。

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionalState {
    /// 主要情感
    pub primary_emotion: Emotion,
    /// 情感强度 (0.0-1.0)
    pub intensity: f32,
    /// 情感倾向 (-1.0到1.0)
    pub valence: f32,
    /// 唤醒度 (0.0-1.0)
    pub arousal: f32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Emotion {
    Joy,
    Sadness,
    Anger,
    Fear,
    Surprise,
    Disgust,
    Neutral,
    Custom(String),
}
```

## 配置类型

### Config

系统配置结构。

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// 数据库配置
    pub database: DatabaseConfig,
    /// 记忆配置
    pub memory: MemoryConfig,
    /// 嵌入模型配置
    pub embedding: EmbeddingConfig,
    /// 工具配置
    pub tools: ToolConfig,
    /// 日志配置
    pub logging: LoggingConfig,
    /// 性能配置
    pub performance: PerformanceConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            database: DatabaseConfig::default(),
            memory: MemoryConfig::default(),
            embedding: EmbeddingConfig::default(),
            tools: ToolConfig::default(),
            logging: LoggingConfig::default(),
            performance: PerformanceConfig::default(),
        }
    }
}
```

### DatabaseConfig

数据库配置。

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    /// 数据库URL
    pub url: String,
    /// 最大连接数
    pub max_connections: u32,
    /// 连接超时（秒）
    pub connection_timeout: Duration,
    /// 是否启用WAL模式
    pub enable_wal: bool,
    /// 是否启用外键约束
    pub enable_foreign_keys: bool,
    /// 缓存大小（MB）
    pub cache_size_mb: u32,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            url: "sqlite:rwkv_agent.db".to_string(),
            max_connections: 10,
            connection_timeout: Duration::from_secs(30),
            enable_wal: true,
            enable_foreign_keys: true,
            cache_size_mb: 64,
        }
    }
}
```

### MemoryConfig

记忆配置。

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryConfig {
    /// 最大记忆数量
    pub max_memories: Option<usize>,
    /// 默认重要性阈值
    pub importance_threshold: f32,
    /// 记忆衰减率
    pub decay_rate: f32,
    /// 自动清理间隔（小时）
    pub cleanup_interval_hours: u64,
    /// 是否启用记忆演化
    pub enable_evolution: bool,
    /// 连接强度阈值
    pub connection_threshold: f32,
}

impl Default for MemoryConfig {
    fn default() -> Self {
        Self {
            max_memories: Some(100000),
            importance_threshold: 0.1,
            decay_rate: 0.01,
            cleanup_interval_hours: 24,
            enable_evolution: true,
            connection_threshold: 0.3,
        }
    }
}
```

## 错误类型

### AgentKitError

系统错误枚举。

```rust
#[derive(Debug, thiserror::Error)]
pub enum AgentKitError {
    #[error("数据库错误: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("序列化错误: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("IO错误: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("配置错误: {message}")]
    Config { message: String },
    
    #[error("记忆错误: {message}")]
    Memory { message: String },
    
    #[error("工具错误: {message}")]
    Tool { message: String },
    
    #[error("嵌入错误: {message}")]
    Embedding { message: String },
    
    #[error("验证错误: {message}")]
    Validation { message: String },
    
    #[error("网络错误: {message}")]
    Network { message: String },
    
    #[error("未知错误: {message}")]
    Unknown { message: String },
}
```

## 实用工具类型

### Pagination

分页参数。

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pagination {
    /// 偏移量
    pub offset: usize,
    /// 每页大小
    pub limit: usize,
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            offset: 0,
            limit: 20,
        }
    }
}
```

### SearchResult&lt;T&gt;

搜索结果泛型。

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult<T> {
    /// 结果项
    pub items: Vec<T>,
    /// 总数量
    pub total_count: usize,
    /// 分页信息
    pub pagination: Pagination,
    /// 搜索耗时（毫秒）
    pub search_time_ms: u64,
}
```

### Statistics

统计信息。

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Statistics {
    /// 计数
    pub count: u64,
    /// 平均值
    pub mean: f64,
    /// 中位数
    pub median: f64,
    /// 标准差
    pub std_dev: f64,
    /// 最小值
    pub min: f64,
    /// 最大值
    pub max: f64,
}
```

## 完整示例

```rust
use rwkv_agent_kit::prelude::*;
use chrono::Utc;
use uuid::Uuid;

fn main() -> Result<()> {
    // 创建记忆属性
    let mut attributes = MemoryAttributes::default();
    attributes.keywords = vec!["Rust".to_string(), "编程".to_string()];
    attributes.tags = vec!["技术".to_string()];
    attributes.importance = 0.8;
    attributes.emotional_valence = 0.2;
    attributes.confidence = 0.9;
    
    // 创建记忆
    let memory = Memory::new(
        "学习Rust编程语言的基础概念".to_string(),
        MemoryType::Knowledge,
        vec![0.1, 0.2, 0.3, 0.4], // 示例嵌入向量
        attributes,
    );
    
    println!("创建记忆: {}", memory.id);
    println!("内容: {}", memory.content);
    println!("类型: {:?}", memory.memory_type);
    println!("重要性: {}", memory.attributes.importance);
    
    // 创建查询
    let query = Query {
        text: "Rust编程".to_string(),
        query_type: QueryType::Semantic,
        filters: QueryFilters {
            memory_types: Some(vec![MemoryType::Knowledge]),
            importance_range: Some((0.5, 1.0)),
            ..Default::default()
        },
        limit: Some(10),
        weights: QueryWeights::default(),
        ..Default::default()
    };
    
    println!("查询文本: {}", query.text);
    println!("查询类型: {:?}", query.query_type);
    
    // 创建连接
    let connection = MemoryConnection::new(
        memory.id,
        Uuid::new_v4(),
        ConnectionType::Semantic,
        0.7,
    );
    
    println!("连接类型: {:?}", connection.connection_type);
    println!("连接强度: {}", connection.strength);
    
    // 创建上下文
    let context = Context {
        user_id: Some("user123".to_string()),
        session_id: Some("session456".to_string()),
        current_task: Some("学习编程".to_string()),
        user_intent: Some(UserIntent::Learning),
        emotional_state: Some(EmotionalState {
            primary_emotion: Emotion::Joy,
            intensity: 0.6,
            valence: 0.8,
            arousal: 0.4,
        }),
        ..Default::default()
    };
    
    println!("用户意图: {:?}", context.user_intent);
    
    Ok(())
}
```

## 下一步

- [记忆管理API](./memory.md)
- [数据库API](./database.md)
- [工具系统API](./tools.md)
- [配置选项](../config/README.md)
//! 核心数据结构和类型定义
//!
//! 本模块包含RWKV-Agent-Kit记忆系统的所有核心数据结构，包括记忆、连接、查询等类型。

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use uuid::Uuid;

// 注意：避免循环依赖，不在此处导入memory模块的类型

/// 记忆ID类型
pub type MemoryId = String;

/// 连接ID类型
pub type ConnectionId = String;

/// 会话ID类型
pub type SessionId = String;

/// 记忆类型枚举
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum MemoryType {
    Knowledge,    // 知识型记忆：事实、概念、技能
    Event,        // 事件型记忆：具体发生的事情
    Task,         // 任务型记忆：待办事项、计划
    Conversation, // 对话型记忆：聊天记录、交互
    Reflection,   // 反思型记忆：总结、思考
    Goal,         // 目标型记忆：长期目标、愿望
    Habit,        // 习惯型记忆：行为模式、偏好
    Emotion,      // 情感型记忆：情绪状态、感受
}

/// 用户状态枚举
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum UserState {
    /// 活跃状态
    Active,
    /// 学习状态
    Learning,
    /// 空闲状态
    Idle,
    /// 忙碌状态
    Busy,
}

/// 情感状态枚举
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EmotionalState {
    /// 中性
    Neutral,
    /// 好奇
    Curious,
    /// 兴奋
    Excited,
    /// 感兴趣
    Interested,
    /// 困惑
    Confused,
    /// 满意
    Satisfied,
}

/// 注意力水平枚举
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AttentionLevel {
    /// 低
    Low,
    /// 中等
    Medium,
    /// 高
    High,
}

/// 记忆结构定义
///
/// 表示系统中的一个记忆单元，包含内容、嵌入向量、属性、连接和元数据。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Memory {
    /// 唯一标识符
    pub id: MemoryId,
    /// 记忆内容
    pub content: String,
    /// 记忆类型
    pub memory_type: MemoryType,
    /// 语义嵌入向量
    pub embedding: Vec<f32>,
    /// 记忆属性
    pub attributes: MemoryAttributes,
    /// 记忆连接
    pub connections: MemoryConnections,
    /// 元数据
    pub metadata: MemoryMetadata,
}

/// 记忆属性
///
/// 包含记忆的各种属性信息，如关键词、标签、重要性等。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MemoryAttributes {
    /// 关键词列表
    pub keywords: Vec<String>,
    /// 标签列表
    pub tags: Vec<String>,
    /// 上下文描述
    pub context: String,
    /// 重要性评分 (0.0 - 1.0)
    pub importance: f32,
    /// 情感标记
    pub emotion: Option<String>,
    /// 来源信息
    pub source: Option<String>,
    /// 置信度 (0.0 - 1.0)
    pub confidence: f32,
    /// 语言标识
    pub language: Option<String>,
    /// 自定义属性
    pub custom_attributes: HashMap<String, Value>,
}

/// 记忆连接
///
/// 定义记忆之间的各种连接关系。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct MemoryConnections {
    /// 语义相关的记忆ID列表
    pub semantic_links: Vec<MemoryId>,
    /// 时间相关的记忆ID列表
    pub temporal_links: Vec<MemoryId>,
    /// 因果相关的记忆ID列表
    pub causal_links: Vec<MemoryId>,
    /// 主题相关的记忆ID列表
    pub thematic_links: Vec<MemoryId>,
    /// 自定义连接类型
    pub custom_links: HashMap<String, Vec<MemoryId>>,
}

/// 记忆元数据
///
/// 包含记忆的时间戳、访问统计等元信息。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MemoryMetadata {
    /// 创建时间
    pub created_at: DateTime<Utc>,
    /// 最后更新时间
    pub updated_at: DateTime<Utc>,
    /// 访问次数
    pub access_count: u64,
    /// 最后访问时间
    pub last_accessed: DateTime<Utc>,
    /// 版本号
    pub version: u32,
    /// 是否已删除
    pub is_deleted: bool,
    /// 删除时间
    pub deleted_at: Option<DateTime<Utc>>,
    /// 自定义元数据
    pub custom_metadata: HashMap<String, Value>,
}

/// 上下文结构
///
/// 表示当前的对话或操作上下文。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Context {
    /// 会话ID
    pub session_id: Option<SessionId>,
    /// 当前主题
    pub current_topic: Option<String>,
    /// 最近的记忆ID列表
    pub recent_memories: Vec<MemoryId>,
    /// 用户偏好设置
    pub user_preferences: HashMap<String, Value>,
    /// 环境信息
    pub environment: HashMap<String, Value>,
    /// 时间窗口
    pub time_window: Option<(DateTime<Utc>, DateTime<Utc>)>,
    /// 优先级
    pub priority: Priority,
}

/// 优先级枚举
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Priority {
    Low,
    Normal,
    High,
    Critical,
}

/// 查询结构
///
/// 定义记忆检索查询的参数。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Query {
    /// 查询文本
    pub text: String,
    /// 查询类型
    pub query_type: QueryType,
    /// 查询过滤器
    pub filters: QueryFilters,
    /// 结果数量限制
    pub limit: Option<usize>,
    /// 偏移量
    pub offset: Option<usize>,
    /// 排序方式
    pub sort_by: Option<SortBy>,
    /// 查询权重
    pub weights: QueryWeights,
}

/// 查询类型枚举
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum QueryType {
    /// 语义检索
    Semantic,
    /// 时间检索
    Temporal,
    /// 因果检索
    Causal,
    /// 主题检索
    Thematic,
    /// 混合检索
    Mixed,
    /// 图遍历检索
    GraphTraversal,
    /// 个性化PageRank检索
    PersonalizedPageRank,
}

/// 查询过滤器
///
/// 定义查询的各种过滤条件。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct QueryFilters {
    /// 标签过滤
    pub tags: Option<Vec<String>>,
    /// 时间范围过滤
    pub time_range: Option<(DateTime<Utc>, DateTime<Utc>)>,
    /// 重要性阈值
    pub importance_threshold: Option<f32>,
    /// 来源过滤
    pub source_filter: Option<String>,
    /// 语言过滤
    pub language_filter: Option<String>,
    /// 情感过滤
    pub emotion_filter: Option<String>,
    /// 置信度阈值
    pub confidence_threshold: Option<f32>,
    /// 自定义过滤器
    pub custom_filters: HashMap<String, Value>,
}

/// 排序方式
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SortBy {
    /// 按相关性排序
    Relevance,
    /// 按时间排序
    Time,
    /// 按重要性排序
    Importance,
    /// 按访问频率排序
    AccessCount,
    /// 按置信度排序
    Confidence,
    /// 自定义排序
    Custom(String),
}

/// 查询权重
///
/// 定义不同检索维度的权重。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryWeights {
    /// 语义权重
    pub semantic_weight: f32,
    /// 时间权重
    pub temporal_weight: f32,
    /// 重要性权重
    pub importance_weight: f32,
    /// 频率权重
    pub frequency_weight: f32,
    /// 个性化权重
    pub personalization_weight: f32,
}

/// 连接结构
///
/// 表示两个记忆之间的连接关系。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Connection {
    /// 连接ID
    pub id: ConnectionId,
    /// 源记忆ID
    pub from_memory: MemoryId,
    /// 目标记忆ID
    pub to_memory: MemoryId,
    /// 连接类型
    pub connection_type: ConnectionType,
    /// 连接强度 (0.0 - 1.0)
    pub strength: f32,
    /// 创建时间
    pub created_at: DateTime<Utc>,
    /// 最后更新时间
    pub updated_at: DateTime<Utc>,
    /// 连接属性
    pub properties: HashMap<String, Value>,
    /// 是否双向连接
    pub bidirectional: bool,
}

/// 连接类型枚举
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ConnectionType {
    /// 语义连接
    Semantic,
    /// 时间连接
    Temporal,
    /// 因果连接
    Causal,
    /// 主题连接
    Thematic,
    /// 层次连接
    Hierarchical,
    /// 引用连接
    Reference,
    /// 自定义连接
    Custom(String),
}

/// 演化触发器
///
/// 定义触发记忆演化的各种事件。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EvolutionTrigger {
    /// 新记忆添加
    NewMemoryAdded(MemoryId),
    /// 访问模式变化
    AccessPatternChanged(MemoryId),
    /// 时间衰减
    TimeDecay,
    /// 用户反馈
    UserFeedback(MemoryId, f32),
    /// 连接强度变化
    ConnectionStrengthChanged(ConnectionId),
    /// 批量更新
    BatchUpdate(Vec<MemoryId>),
    /// 系统维护
    SystemMaintenance,
}

/// 更新操作
///
/// 定义对记忆的各种更新操作。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Update {
    /// 更新ID
    pub id: String,
    /// 目标记忆ID
    pub memory_id: MemoryId,
    /// 更新类型
    pub update_type: UpdateType,
    /// 时间戳
    pub timestamp: DateTime<Utc>,
    /// 更新原因
    pub reason: Option<String>,
    /// 更新者信息
    pub updater_info: Option<String>,
}

/// 更新类型枚举
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UpdateType {
    /// 重要性调整
    ImportanceAdjustment(f32),
    /// 连接添加
    ConnectionAdded(Connection),
    /// 连接移除
    ConnectionRemoved(ConnectionId),
    /// 属性更新
    AttributeUpdate(String, Value),
    /// 内容更新
    ContentUpdate(String),
    /// 标签更新
    TagUpdate(Vec<String>),
    /// 元数据更新
    MetadataUpdate(HashMap<String, Value>),
}

/// 交互记录
///
/// 记录用户与系统的交互信息。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Interaction {
    /// 交互ID
    pub id: String,
    /// 用户ID
    pub user_id: String,
    /// 会话ID
    pub session_id: Option<SessionId>,
    /// 查询内容
    pub query: String,
    /// 检索到的记忆ID列表
    pub retrieved_memories: Vec<MemoryId>,
    /// 用户反馈评分
    pub user_feedback: Option<f32>,
    /// 交互类型
    pub interaction_type: InteractionType,
    /// 时间戳
    pub timestamp: DateTime<Utc>,
    /// 响应时间（毫秒）
    pub response_time_ms: Option<u64>,
    /// 额外信息
    pub additional_info: HashMap<String, Value>,
}

/// 交互类型枚举
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum InteractionType {
    /// 查询
    Query,
    /// 创建记忆
    CreateMemory,
    /// 更新记忆
    UpdateMemory,
    /// 删除记忆
    DeleteMemory,
    /// 反馈
    Feedback,
    /// 浏览
    Browse,
}

/// 统计信息
///
/// 系统运行统计数据。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Statistics {
    /// 总记忆数量
    pub total_memories: u64,
    /// 总连接数量
    pub total_connections: u64,
    /// 平均重要性
    pub average_importance: f32,
    /// 最近访问的记忆数量
    pub recently_accessed_count: u64,
    /// 系统启动时间
    pub system_start_time: DateTime<Utc>,
    /// 最后更新时间
    pub last_update_time: DateTime<Utc>,
    /// 用户统计
    pub user_stats: HashMap<String, UserStatistics>,
    /// 性能指标
    pub performance_metrics: PerformanceMetrics,
}

/// 用户统计信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserStatistics {
    /// 用户ID
    pub user_id: String,
    /// 记忆数量
    pub memory_count: u64,
    /// 查询次数
    pub query_count: u64,
    /// 平均反馈评分
    pub average_feedback: f32,
    /// 最后活跃时间
    pub last_active: DateTime<Utc>,
    /// 偏好标签
    pub preferred_tags: Vec<String>,
}

/// 性能指标
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PerformanceMetrics {
    /// 平均查询时间（毫秒）
    pub average_query_time_ms: f64,
    /// 平均创建时间（毫秒）
    pub average_creation_time_ms: f64,
    /// 缓存命中率
    pub cache_hit_rate: f32,
    /// 内存使用量（字节）
    pub memory_usage_bytes: u64,
    /// 磁盘使用量（字节）
    pub disk_usage_bytes: u64,
    /// 每秒查询数
    pub queries_per_second: f64,
}

// 默认实现
impl Default for MemoryAttributes {
    fn default() -> Self {
        Self {
            keywords: Vec::new(),
            tags: Vec::new(),
            context: String::new(),
            importance: 0.5,
            emotion: None,
            source: None,
            confidence: 1.0,
            language: None,
            custom_attributes: HashMap::new(),
        }
    }
}

impl Default for Context {
    fn default() -> Self {
        Self {
            session_id: None,
            current_topic: None,
            recent_memories: Vec::new(),
            user_preferences: HashMap::new(),
            environment: HashMap::new(),
            time_window: None,
            priority: Priority::Normal,
        }
    }
}

impl Default for QueryWeights {
    fn default() -> Self {
        Self {
            semantic_weight: 0.4,
            temporal_weight: 0.2,
            importance_weight: 0.2,
            frequency_weight: 0.1,
            personalization_weight: 0.1,
        }
    }
}

impl Default for Priority {
    fn default() -> Self {
        Self::Normal
    }
}

// 辅助函数
impl Default for MemoryMetadata {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            created_at: now,
            updated_at: now,
            access_count: 0,
            last_accessed: now,
            version: 1,
            is_deleted: false,
            deleted_at: None,
            custom_metadata: HashMap::new(),
        }
    }
}

impl Default for Memory {
    fn default() -> Self {
        let _now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            content: String::new(),
            memory_type: MemoryType::Knowledge,
            embedding: Vec::new(),
            attributes: MemoryAttributes::default(),
            connections: MemoryConnections::default(),
            metadata: MemoryMetadata::default(),
        }
    }
}

impl Memory {
    /// 创建新的记忆
    pub fn new(
        content: String,
        memory_type: MemoryType,
        embedding: Vec<f32>,
        attributes: MemoryAttributes,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            content,
            memory_type,
            embedding,
            attributes,
            connections: MemoryConnections::default(),
            metadata: MemoryMetadata {
                created_at: now,
                updated_at: now,
                access_count: 0,
                last_accessed: now,
                version: 1,
                is_deleted: false,
                deleted_at: None,
                custom_metadata: HashMap::new(),
            },
        }
    }

    /// 更新访问信息
    pub fn update_access(&mut self) {
        self.metadata.access_count += 1;
        self.metadata.last_accessed = Utc::now();
    }

    /// 检查是否匹配过滤器
    pub fn matches_filters(&self, filters: &QueryFilters) -> bool {
        // 标签过滤
        if let Some(required_tags) = &filters.tags {
            if !required_tags
                .iter()
                .any(|tag| self.attributes.tags.contains(tag))
            {
                return false;
            }
        }

        // 时间范围过滤
        if let Some((start, end)) = filters.time_range {
            if self.metadata.created_at < start || self.metadata.created_at > end {
                return false;
            }
        }

        // 重要性阈值过滤
        if let Some(threshold) = filters.importance_threshold {
            if self.attributes.importance < threshold {
                return false;
            }
        }

        // 置信度阈值过滤
        if let Some(threshold) = filters.confidence_threshold {
            if self.attributes.confidence < threshold {
                return false;
            }
        }

        // 来源过滤
        if let Some(source) = &filters.source_filter {
            if self.attributes.source.as_ref() != Some(source) {
                return false;
            }
        }

        // 语言过滤
        if let Some(language) = &filters.language_filter {
            if self.attributes.language.as_ref() != Some(language) {
                return false;
            }
        }

        // 情感过滤
        if let Some(emotion) = &filters.emotion_filter {
            if self.attributes.emotion.as_ref() != Some(emotion) {
                return false;
            }
        }

        true
    }
}

impl Connection {
    /// 创建新的连接
    pub fn new(
        from_memory: MemoryId,
        to_memory: MemoryId,
        connection_type: ConnectionType,
        strength: f32,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            from_memory,
            to_memory,
            connection_type,
            strength: strength.clamp(0.0, 1.0),
            created_at: now,
            updated_at: now,
            properties: HashMap::new(),
            bidirectional: false,
        }
    }

    /// 更新连接强度
    pub fn update_strength(&mut self, new_strength: f32) {
        self.strength = new_strength.clamp(0.0, 1.0);
        self.updated_at = Utc::now();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_creation() {
        let memory = Memory::new(
            "Test content".to_string(),
            MemoryType::Knowledge,
            vec![0.1, 0.2, 0.3],
            MemoryAttributes::default(),
        );

        assert_eq!(memory.content, "Test content");
        assert_eq!(memory.embedding, vec![0.1, 0.2, 0.3]);
        assert_eq!(memory.metadata.access_count, 0);
        assert_eq!(memory.metadata.version, 1);
    }

    #[test]
    fn test_connection_creation() {
        let connection = Connection::new(
            "mem1".to_string(),
            "mem2".to_string(),
            ConnectionType::Semantic,
            0.8,
        );

        assert_eq!(connection.from_memory, "mem1");
        assert_eq!(connection.to_memory, "mem2");
        assert_eq!(connection.strength, 0.8);
        assert!(!connection.bidirectional);
    }

    #[test]
    fn test_memory_filters() {
        let memory = Memory::new(
            "Test content".to_string(),
            MemoryType::Knowledge,
            vec![0.1, 0.2, 0.3],
            MemoryAttributes {
                tags: vec!["test".to_string(), "example".to_string()],
                importance: 0.7,
                confidence: 0.9,
                ..Default::default()
            },
        );

        let filters = QueryFilters {
            tags: Some(vec!["test".to_string()]),
            importance_threshold: Some(0.5),
            confidence_threshold: Some(0.8),
            ..Default::default()
        };

        assert!(memory.matches_filters(&filters));

        let strict_filters = QueryFilters {
            importance_threshold: Some(0.9),
            ..Default::default()
        };

        assert!(!memory.matches_filters(&strict_filters));
    }

    #[test]
    fn test_connection_strength_clamping() {
        let mut connection = Connection::new(
            "mem1".to_string(),
            "mem2".to_string(),
            ConnectionType::Semantic,
            1.5, // 超出范围
        );

        assert_eq!(connection.strength, 1.0); // 应该被限制在1.0

        connection.update_strength(-0.5); // 负值
        assert_eq!(connection.strength, 0.0); // 应该被限制在0.0
    }
}

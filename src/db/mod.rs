//! 数据库模块
//! 提供统一的数据库接口和SQLite实现

pub mod config;
pub mod embedding;
pub mod manager;
pub mod performance;
pub mod query_optimizer;
pub mod sqlite;

// 重新导出主要类型
pub use config::{DatabaseConfig, DatabaseType};
pub use manager::DatabaseManager;
pub use performance::*;
pub use query_optimizer::*;
pub use sqlite::SqliteDatabase;

/// 数据库操作结果类型
pub type DbResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

/// 记忆事件（替代 conversations 表的最小可用记录）
#[derive(Debug, Clone)]
pub struct MemoryEvent {
    pub session_id: i64,
    pub agent_name: String,
    pub role: String, // "user" 或 "assistant"
    pub text: String,
    pub topic: Option<String>,
    pub sentiment: Option<f32>,
    pub importance: Option<f32>,
    pub decay: f32,
    pub embedding: Option<Vec<u8>>, // 序列化向量（可选）
}

/// 语义片段（长期总结）
#[derive(Debug, Clone)]
pub struct SemanticChunk {
    pub id: Option<i64>,
    pub title: Option<String>,
    pub summary: String,
    pub keywords: Option<String>,
    pub embedding: Option<Vec<u8>>,
    pub last_ref_ts: Option<String>, // 最后引用时间戳
    pub weight: f32,                 // 重要性权重
}

/// 语义片段到会话/事件的映射关系
#[derive(Debug, Clone)]
pub struct SemanticChunkMapping {
    pub id: Option<i64>,
    pub chunk_id: i64,              // 语义片段ID
    pub session_id: i64,            // 会话ID
    pub memory_event_ids: String,   // 关联的memory_event ID列表，JSON格式如 "[1,2,3]"
    pub created_ts: Option<String>, // 创建时间戳
}

/// 图节点（实体）
#[derive(Debug, Clone)]
pub struct GraphNode {
    pub id: Option<i64>,
    pub entity_type: String, // "person", "location", "topic", "preference" 等
    pub entity_name: String, // 实体名称
}

/// 图边（关系）
#[derive(Debug, Clone)]
pub struct GraphEdge {
    pub id: Option<i64>,
    pub from_node: i64,
    pub to_node: i64,
    pub relation_type: String, // "mentions", "likes", "works_at" 等
    pub weight: f32,           // 关系权重
}

/// 用户画像配置文件（聚合信息）
#[derive(Debug, Clone)]
pub struct PersonaProfile {
    pub id: Option<i64>,
    pub agent_name: String, // 关联的Agent，单用户多Agent设计
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

/// 用户画像特征/事实（原子画像信息）
#[derive(Debug, Clone)]
pub struct PersonaTrait {
    pub id: Option<i64>,
    pub agent_name: String,           // 关联的Agent
    pub trait_type: String, // "preference", "attribute", "taboo", "goal", "style", "context"
    pub trait_key: String,  // e.g., "food", "music", "work_hours"
    pub trait_value: String, // normalized text or JSON
    pub confidence: f32,    // 0.0 - 1.0, 置信度
    pub stability: f32,     // 0.0 - 1.0, 稳定性，随重复确认增长
    pub last_seen: Option<String>, // 最后观察到的时间
    pub source_event_id: Option<i64>, // 来源 memory_events.id
}

/// 数据库操作trait
#[async_trait::async_trait]
pub trait Database: Send + Sync + std::fmt::Debug {
    /// 初始化数据库
    async fn initialize(&mut self) -> DbResult<()>;

    /// 健康检查
    async fn health_check(&self) -> DbResult<bool>;

    /// 关闭数据库连接
    async fn close(&mut self) -> DbResult<()>;

    /// 数据库迁移
    async fn migrate(&mut self) -> DbResult<()> {
        // 默认空实现
        Ok(())
    }

    /// 备份数据库
    async fn backup(&self, backup_path: &str) -> DbResult<()> {
        let _ = backup_path; // 避免未使用参数警告
        Err("Backup not supported".into())
    }

    /// 转换为SQLite数据库实例（仅限SQLite实现）
    async fn as_sqlite(&self) -> DbResult<Option<&dyn std::any::Any>> {
        Ok(None)
    }

    // 会话管理（单用户，多Agent）
    async fn open_session(&self, agent_name: &str, title: Option<&str>) -> DbResult<i64>;
    async fn close_active_session(&self) -> DbResult<()>;
    async fn get_active_session(&self) -> DbResult<Option<i64>>;
    async fn upsert_session_title(&self, session_id: i64, title: &str) -> DbResult<()>;

    // 记忆事件（替代原 save_conversation）
    async fn insert_memory_event(&self, event: MemoryEvent) -> DbResult<i64>;
    async fn list_memory_events(&self, session_id: i64) -> DbResult<Vec<MemoryEvent>>;
    async fn clear_all_memory_events(&self) -> DbResult<()>; // 清理所有记忆事件（调试用）

    // 阶段3: 长期语义片段与图谱
    async fn insert_semantic_chunk(&self, chunk: SemanticChunk) -> DbResult<i64>;
    async fn list_semantic_chunks(&self, limit: Option<i32>) -> DbResult<Vec<SemanticChunk>>;
    async fn update_semantic_chunk_ref_time(&self, chunk_id: i64) -> DbResult<()>;

    // 语义片段映射管理
    async fn insert_semantic_chunk_mapping(&self, mapping: SemanticChunkMapping) -> DbResult<i64>;
    async fn get_chunk_mappings_by_chunk_id(
        &self,
        chunk_id: i64,
    ) -> DbResult<Vec<SemanticChunkMapping>>;
    async fn get_chunk_mappings_by_session_id(
        &self,
        session_id: i64,
    ) -> DbResult<Vec<SemanticChunkMapping>>;

    // 图谱管理
    async fn upsert_graph_node(&self, node: GraphNode) -> DbResult<i64>;
    async fn upsert_graph_edge(&self, edge: GraphEdge) -> DbResult<i64>;
    /// 累积边权重版本的 upsert_graph_edge
    async fn upsert_graph_edge_with_accumulation(&self, edge: GraphEdge) -> DbResult<i64>;
    async fn get_graph_nodes(&self) -> DbResult<Vec<GraphNode>>;
    async fn get_graph_edges(&self) -> DbResult<Vec<GraphEdge>>;
    async fn get_node_by_entity(
        &self,
        entity_type: &str,
        entity_name: &str,
    ) -> DbResult<Option<GraphNode>>;

    // 阶段5: 画像/Persona 管理
    async fn upsert_persona_profile(&self, profile: PersonaProfile) -> DbResult<i64>;
    async fn get_persona_profile(&self, agent_name: &str) -> DbResult<Option<PersonaProfile>>;
    async fn upsert_persona_trait(&self, trait_item: PersonaTrait) -> DbResult<i64>;
    async fn list_persona_traits(
        &self,
        agent_name: &str,
        trait_type: Option<&str>,
        top_k: Option<usize>,
    ) -> DbResult<Vec<PersonaTrait>>;
    async fn get_relevant_persona_facts(
        &self,
        agent_name: &str,
        query: &str,
        top_k: usize,
    ) -> DbResult<Vec<PersonaTrait>>;
}

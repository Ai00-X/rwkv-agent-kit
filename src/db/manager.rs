//! 数据库管理器
//! 提供统一的数据库管理接口

use super::config::{DatabaseConfig, DatabaseType};
use super::embedding::{get_global_embedding_service, EmbeddingService};
use super::sqlite::SqliteDatabase;
use super::{
    Database, DbResult, GraphEdge, GraphNode, MemoryEvent, PersonaProfile, PersonaTrait,
    SemanticChunk,
};
use std::sync::Arc;
use tokio::sync::Mutex;

/// 数据库实例枚举
#[derive(Debug)]
enum DatabaseInstance {
    Sqlite(SqliteDatabase),
}

/// 数据库管理器
#[derive(Clone, Debug)]
pub struct DatabaseManager {
    database: Arc<Mutex<DatabaseInstance>>,
    config: DatabaseConfig,
}

impl DatabaseManager {
    /// 创建新的数据库管理器
    pub async fn new(config: DatabaseConfig) -> DbResult<Self> {
        let database = match config.db_type {
            DatabaseType::Sqlite => {
                let mut db = SqliteDatabase::new(config.clone())?;
                db.initialize().await?;
                DatabaseInstance::Sqlite(db)
            }
            DatabaseType::Memory => {
                let mut config = config.clone();
                config.db_path = ":memory:".into();
                let mut db = SqliteDatabase::new(config)?;
                db.initialize().await?;
                DatabaseInstance::Sqlite(db)
            }
        };

        Ok(Self {
            database: Arc::new(Mutex::new(database)),
            config,
        })
    }

    /// 获取数据库配置
    pub fn config(&self) -> &DatabaseConfig {
        &self.config
    }

    /// 检查数据库健康状态
    pub async fn health_check(&self) -> DbResult<bool> {
        let db = self.database.lock().await;
        match &*db {
            DatabaseInstance::Sqlite(sqlite_db) => sqlite_db.health_check().await,
        }
    }

    /// 关闭数据库连接
    pub async fn close(&self) -> DbResult<()> {
        let mut db = self.database.lock().await;
        match &mut *db {
            DatabaseInstance::Sqlite(sqlite_db) => sqlite_db.close().await,
        }
    }

    /// 获取SQLite数据库实例（如果是SQLite类型）
    pub async fn as_sqlite(&self) -> Option<SqliteDatabase> {
        match self.config.db_type {
            DatabaseType::Sqlite => {
                let db = self.database.lock().await;
                match &*db {
                    DatabaseInstance::Sqlite(sqlite_db) => Some(sqlite_db.clone()),
                }
            }
            _ => None,
        }
    }

    /// 执行数据库迁移
    pub async fn migrate(&mut self) -> DbResult<()> {
        let mut db = self.database.lock().await;
        match &mut *db {
            DatabaseInstance::Sqlite(sqlite_db) => sqlite_db.migrate().await,
        }
    }

    /// 备份数据库
    pub async fn backup<P: AsRef<std::path::Path>>(&self, backup_path: P) -> DbResult<()> {
        match self.config.db_type {
            DatabaseType::Sqlite => {
                let db = self.database.lock().await;
                let path_str = backup_path.as_ref().to_string_lossy().into_owned();
                match &*db {
                    DatabaseInstance::Sqlite(sqlite_db) => sqlite_db.backup(&path_str).await,
                }
            }
            _ => Err("Backup not supported for this database type".into()),
        }
    }

    /// 打开会话
    pub async fn open_session(&self, agent_name: &str, title: Option<&str>) -> DbResult<i64> {
        let db = self.database.lock().await;
        match &*db {
            DatabaseInstance::Sqlite(sqlite_db) => sqlite_db.open_session(agent_name, title).await,
        }
    }

    /// 关闭活跃会话
    pub async fn close_active_session(&self) -> DbResult<()> {
        let db = self.database.lock().await;
        match &*db {
            DatabaseInstance::Sqlite(sqlite_db) => sqlite_db.close_active_session().await,
        }
    }

    /// 获取活跃会话
    pub async fn get_active_session(&self) -> DbResult<Option<i64>> {
        let db = self.database.lock().await;
        match &*db {
            DatabaseInstance::Sqlite(sqlite_db) => sqlite_db.get_active_session().await,
        }
    }

    /// 更新会话标题
    pub async fn upsert_session_title(&self, session_id: i64, title: &str) -> DbResult<()> {
        let db = self.database.lock().await;
        match &*db {
            DatabaseInstance::Sqlite(sqlite_db) => {
                sqlite_db.upsert_session_title(session_id, title).await
            }
        }
    }

    /// 插入记忆事件
    pub async fn insert_memory_event(&self, event: MemoryEvent) -> DbResult<i64> {
        let db = self.database.lock().await;
        match &*db {
            DatabaseInstance::Sqlite(sqlite_db) => sqlite_db.insert_memory_event(event).await,
        }
    }

    /// 列出会话的记忆事件
    pub async fn list_memory_events(&self, session_id: i64) -> DbResult<Vec<MemoryEvent>> {
        let db = self.database.lock().await;
        match &*db {
            DatabaseInstance::Sqlite(sqlite_db) => sqlite_db.list_memory_events(session_id).await,
        }
    }

    /// 清理所有记忆事件（用于调试）
    pub async fn clear_all_memory_events(&self) -> DbResult<()> {
        let db = self.database.lock().await;
        match &*db {
            DatabaseInstance::Sqlite(sqlite_db) => sqlite_db.clear_all_memory_events().await,
        }
    }

    /// 插入语义片段
    pub async fn insert_semantic_chunk(&self, chunk: SemanticChunk) -> DbResult<i64> {
        let db = self.database.lock().await;
        match &*db {
            DatabaseInstance::Sqlite(sqlite_db) => sqlite_db.insert_semantic_chunk(chunk).await,
        }
    }

    /// 列出语义片段
    pub async fn list_semantic_chunks(&self, limit: Option<i32>) -> DbResult<Vec<SemanticChunk>> {
        let db = self.database.lock().await;
        match &*db {
            DatabaseInstance::Sqlite(sqlite_db) => sqlite_db.list_semantic_chunks(limit).await,
        }
    }

    /// 更新语义片段最近引用时间
    pub async fn update_semantic_chunk_ref_time(&self, chunk_id: i64) -> DbResult<()> {
        let db = self.database.lock().await;
        match &*db {
            DatabaseInstance::Sqlite(sqlite_db) => {
                sqlite_db.update_semantic_chunk_ref_time(chunk_id).await
            }
        }
    }

    /// upsert 图节点
    pub async fn upsert_graph_node(&self, node: GraphNode) -> DbResult<i64> {
        let db = self.database.lock().await;
        match &*db {
            DatabaseInstance::Sqlite(sqlite_db) => sqlite_db.upsert_graph_node(node).await,
        }
    }

    /// upsert 图边
    pub async fn upsert_graph_edge(&self, edge: GraphEdge) -> DbResult<i64> {
        let db = self.database.lock().await;
        match &*db {
            DatabaseInstance::Sqlite(sqlite_db) => sqlite_db.upsert_graph_edge(edge).await,
        }
    }

    /// 获取所有图节点
    pub async fn get_graph_nodes(&self) -> DbResult<Vec<GraphNode>> {
        let db = self.database.lock().await;
        match &*db {
            DatabaseInstance::Sqlite(sqlite_db) => sqlite_db.get_graph_nodes().await,
        }
    }

    /// 获取所有图边
    pub async fn get_graph_edges(&self) -> DbResult<Vec<GraphEdge>> {
        let db = self.database.lock().await;
        match &*db {
            DatabaseInstance::Sqlite(sqlite_db) => sqlite_db.get_graph_edges().await,
        }
    }

    /// 根据实体获取节点
    pub async fn get_node_by_entity(
        &self,
        entity_type: &str,
        entity_name: &str,
    ) -> DbResult<Option<GraphNode>> {
        let db = self.database.lock().await;
        match &*db {
            DatabaseInstance::Sqlite(sqlite_db) => {
                sqlite_db.get_node_by_entity(entity_type, entity_name).await
            }
        }
    }

    /// 插入语义片段映射
    pub async fn insert_semantic_chunk_mapping(
        &self,
        mapping: super::SemanticChunkMapping,
    ) -> DbResult<i64> {
        let db = self.database.lock().await;
        match &*db {
            DatabaseInstance::Sqlite(sqlite_db) => {
                sqlite_db.insert_semantic_chunk_mapping(mapping).await
            }
        }
    }

    /// 根据片段ID获取映射
    pub async fn get_chunk_mappings_by_chunk_id(
        &self,
        chunk_id: i64,
    ) -> DbResult<Vec<super::SemanticChunkMapping>> {
        let db = self.database.lock().await;
        match &*db {
            DatabaseInstance::Sqlite(sqlite_db) => {
                sqlite_db.get_chunk_mappings_by_chunk_id(chunk_id).await
            }
        }
    }

    /// 根据会话ID获取映射
    pub async fn get_chunk_mappings_by_session_id(
        &self,
        session_id: i64,
    ) -> DbResult<Vec<super::SemanticChunkMapping>> {
        let db = self.database.lock().await;
        match &*db {
            DatabaseInstance::Sqlite(sqlite_db) => {
                sqlite_db.get_chunk_mappings_by_session_id(session_id).await
            }
        }
    }

    /// 累积边权重版本的 upsert_graph_edge
    pub async fn upsert_graph_edge_with_accumulation(&self, edge: GraphEdge) -> DbResult<i64> {
        let db = self.database.lock().await;
        match &*db {
            DatabaseInstance::Sqlite(sqlite_db) => {
                sqlite_db.upsert_graph_edge_with_accumulation(edge).await
            }
        }
    }

    // ===== 阶段5：画像/Persona 便捷方法 =====
    /// 确保存在 persona_profile 记录（若不存在则创建），返回 profile id
    pub async fn upsert_persona_profile(&self, profile: PersonaProfile) -> DbResult<i64> {
        let db = self.database.lock().await;
        match &*db {
            DatabaseInstance::Sqlite(sqlite_db) => sqlite_db.upsert_persona_profile(profile).await,
        }
    }

    /// 获取指定智能体的 persona_profile（如果存在）
    pub async fn get_persona_profile(&self, agent_name: &str) -> DbResult<Option<PersonaProfile>> {
        let db = self.database.lock().await;
        match &*db {
            DatabaseInstance::Sqlite(sqlite_db) => sqlite_db.get_persona_profile(agent_name).await,
        }
    }

    /// 插入或更新 persona_trait（基于 agent_name + trait_type + trait_key 唯一约束）
    pub async fn upsert_persona_trait(&self, trait_item: PersonaTrait) -> DbResult<i64> {
        let db = self.database.lock().await;
        match &*db {
            DatabaseInstance::Sqlite(sqlite_db) => sqlite_db.upsert_persona_trait(trait_item).await,
        }
    }

    /// 列出指定智能体的画像特征
    pub async fn list_persona_traits(
        &self,
        agent_name: &str,
        trait_type: Option<&str>,
        top_k: Option<usize>,
    ) -> DbResult<Vec<PersonaTrait>> {
        let db = self.database.lock().await;
        match &*db {
            DatabaseInstance::Sqlite(sqlite_db) => {
                sqlite_db
                    .list_persona_traits(agent_name, trait_type, top_k)
                    .await
            }
        }
    }

    /// 基于查询字符串检索与之相关的画像事实
    pub async fn get_relevant_persona_facts(
        &self,
        agent_name: &str,
        query: &str,
        top_k: usize,
    ) -> DbResult<Vec<PersonaTrait>> {
        let db = self.database.lock().await;
        match &*db {
            DatabaseInstance::Sqlite(sqlite_db) => {
                sqlite_db
                    .get_relevant_persona_facts(agent_name, query, top_k)
                    .await
            }
        }
    }
}

/// 数据库统计信息
#[derive(Debug, Clone)]
pub struct DatabaseStats {
    pub db_size_bytes: u64,
    pub last_backup: Option<std::time::SystemTime>,
}

impl DatabaseManager {
    /// 检索记忆（阶段4.2）：
    /// - 从 memory_events 与 semantic_chunks 中暴力检索
    /// - 使用余弦相似度（bincode反序列化 Vec<f32>）
    /// - 应用时间衰减和重要性加权
    ///   返回用于提示词注入的格式化文本
    pub async fn retrieve_memories(
        &self,
        agent_name: &str,
        query: &str,
        top_k: usize,
        time_decay_factor: f32,
        importance_weight: f32,
        max_chars: usize,
    ) -> DbResult<String> {
        // 计算查询向量（如果嵌入服务可用），否则使用词重叠度作为回退
        let query_tokens: Vec<&str> = query.split_whitespace().collect();
        let token_set: std::collections::HashSet<&str> = query_tokens.iter().copied().collect();
        let token_overlap = |text: &str| -> f32 {
            let mut cnt = 0f32;
            for t in text.split_whitespace() {
                if token_set.contains(t) {
                    cnt += 1.0;
                }
            }
            if query_tokens.is_empty() {
                0.0
            } else {
                cnt / (query_tokens.len() as f32)
            }
        };

        let mut query_embedding: Option<Vec<f32>> = None;
        if let Ok(svc) = get_global_embedding_service() {
            if let Ok(emb) = svc.lock().await.encode_single(query).await {
                query_embedding = Some(emb);
            }
        }

        // 1) 读取当前活跃会话的 memory_events
        let session_id_opt = { self.get_active_session().await? };
        let mut candidates: Vec<(f32, String)> = Vec::new();

        // 2) 遍历 memory_events
        if let Some(session_id) = session_id_opt {
            let events = self.list_memory_events(session_id).await?;
            for ev in events {
                // 余弦相似度（若有embedding），否则回退到词重叠度
                let mut score = if let (Some(qe), Some(bytes)) =
                    (query_embedding.as_ref(), ev.embedding.as_ref())
                {
                    if let Ok(ev_emb) = EmbeddingService::deserialize_embedding(bytes) {
                        EmbeddingService::cosine_similarity(qe, &ev_emb)
                    } else {
                        token_overlap(&ev.text)
                    }
                } else {
                    token_overlap(&ev.text)
                };
                // importance 加权
                if let Some(imp) = ev.importance {
                    score *= 1.0 + importance_weight * imp;
                }
                // 记录文本
                candidates.push((score, format!("[Event] {}: {}", ev.role, ev.text)));
            }
        }

        // 3) 遍历 semantic_chunks（按最近引用已排序，但这里仍计算相似度）
        let chunks = self
            .list_semantic_chunks(Some(100))
            .await
            .unwrap_or_default();
        for ch in chunks {
            let base_sim = if let (Some(qe), Some(bytes)) =
                (query_embedding.as_ref(), ch.embedding.as_ref())
            {
                if let Ok(ch_emb) = EmbeddingService::deserialize_embedding(bytes) {
                    EmbeddingService::cosine_similarity(qe, &ch_emb)
                } else {
                    token_overlap(ch.summary.as_str())
                }
            } else {
                token_overlap(ch.summary.as_str())
            };
            let mut score = base_sim * (1.0 + ch.weight);
            // 时间衰减（如果有 last_ref_ts）
            if let Some(ts) = ch.last_ref_ts.as_ref() {
                if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(ts, "%Y-%m-%d %H:%M:%S") {
                    let now = chrono::Local::now().naive_local();
                    let delta_h = (now - dt).num_minutes() as f32 / 60.0;
                    let decay = (-delta_h / time_decay_factor.max(1e-3)).exp();
                    score *= decay;
                }
            }
            let title = ch.title.unwrap_or_else(|| "Semantic Chunk".to_string());
            candidates.push((score, format!("[Chunk] {}: {}", title, ch.summary)));
        }

        // 3.5) 检索与当前查询相关的 Persona facts，并优先注入
        let mut persona_section = String::new();
        if let Ok(persona_facts) = self
            .get_relevant_persona_facts(agent_name, query, std::cmp::min(5, top_k))
            .await
        {
            if !persona_facts.is_empty() {
                persona_section.push_str("[Persona Facts]\n");
                for pf in persona_facts {
                    let conf = format!("{:.2}", pf.confidence);
                    let line = format!(
                        "[Persona] {}::{} = {} (conf: {})\n",
                        pf.trait_type, pf.trait_key, pf.trait_value, conf
                    );
                    // 先不超过上限写入，避免超长
                    if persona_section.len() + line.len() <= max_chars {
                        persona_section.push_str(&line);
                    } else {
                        break;
                    }
                }
                // 添加一个空行分隔
                if persona_section.len() < max_chars {
                    persona_section.push('\n');
                }
            }
        }

        // 4) 排序取TopK并裁剪长度（在 persona 段之后追加）
        candidates.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
        let mut out = String::new();
        // 先写 persona 段
        if !persona_section.is_empty() {
            out.push_str(&persona_section);
        }
        for (_s, txt) in candidates.into_iter().take(top_k) {
            if out.len() + txt.len() + 1 > max_chars {
                break;
            }
            out.push_str(&txt);
            out.push('\n');
        }
        Ok(out)
    }
}

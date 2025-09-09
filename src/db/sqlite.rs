//! SQLite数据库实现 - 使用 sqlx

use super::config::DatabaseConfig;
use super::{Database, DbResult, MemoryEvent};
use sqlx::Row;
use sqlx::{sqlite::SqliteConnectOptions, ConnectOptions, SqlitePool};
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::Mutex;

/// SQLite数据库实现
#[derive(Debug, Clone)]
pub struct SqliteDatabase {
    pool: Arc<Mutex<Option<SqlitePool>>>,
    config: DatabaseConfig,
}

impl SqliteDatabase {
    /// 创建新的SQLite数据库实例
    pub fn new(config: DatabaseConfig) -> DbResult<Self> {
        Ok(Self {
            pool: Arc::new(Mutex::new(None)),
            config,
        })
    }

    /// 获取数据库连接池
    async fn get_pool(&self) -> DbResult<SqlitePool> {
        let mut pool_guard = self.pool.lock().await;

        if let Some(ref pool) = *pool_guard {
            return Ok(pool.clone());
        }

        // 确保数据目录存在
        if let Some(parent) = self.config.db_path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create directory: {}", e))?;
        }

        // 配置 SQLite 连接选项，启用调试模式
        let options =
            SqliteConnectOptions::from_str(&format!("sqlite:{}", self.config.db_path.display()))
                .map_err(|e| format!("Failed to parse database URL: {}", e))?
                .create_if_missing(true)
                .pragma("journal_mode", "WAL")
                .pragma("synchronous", "NORMAL")
                .pragma("cache_size", "-64000")
                .pragma("foreign_keys", "ON")
                .pragma("temp_store", "MEMORY")
                .log_statements(log::LevelFilter::Debug); // 启用 SQL 语句调试日志

        let pool = SqlitePool::connect_with(options)
            .await
            .map_err(|e| format!("Failed to connect to database: {}", e))?;

        *pool_guard = Some(pool.clone());
        Ok(pool)
    }

    /// 执行SQL语句
    pub async fn execute(&self, sql: &str) -> DbResult<u64> {
        let pool = self.get_pool().await?;

        log::debug!("Executing SQL: {}", sql);

        let result = sqlx::query(sql)
            .execute(&pool)
            .await
            .map_err(|e| format!("Failed to execute SQL: {}", e))?;

        log::debug!(
            "SQL execution result: {} rows affected",
            result.rows_affected()
        );

        Ok(result.rows_affected())
    }

    /// 查询数据
    pub async fn query_raw(&self, sql: &str) -> DbResult<Vec<sqlx::sqlite::SqliteRow>> {
        let pool = self.get_pool().await?;

        log::debug!("Querying SQL: {}", sql);

        let rows = sqlx::query(sql)
            .fetch_all(&pool)
            .await
            .map_err(|e| format!("Failed to query: {}", e))?;

        log::debug!("Query returned {} rows", rows.len());

        Ok(rows)
    }

    /// 创建基础表结构
    async fn create_tables(&self) -> DbResult<()> {
        let pool = self.get_pool().await?;

        log::info!("Creating database tables...");

        // 阶段1: 单用户会话表（移除user_id，新增agent_name）
        let create_sessions_sql = r#"
            CREATE TABLE IF NOT EXISTS sessions (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT,
                agent_name TEXT,
                started_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                ended_at DATETIME,
                is_active BOOLEAN NOT NULL DEFAULT 1,
                metadata TEXT
            )
        "#;
        log::debug!("Creating sessions table...");
        sqlx::query(create_sessions_sql)
            .execute(&pool)
            .await
            .map_err(|e| format!("Failed to create sessions table: {}", e))?;

        // 会话表索引
        sqlx::query("CREATE INDEX IF NOT EXISTS idx_sessions_agent_name ON sessions(agent_name)")
            .execute(&pool)
            .await
            .map_err(|e| format!("Failed to create sessions agent_name index: {}", e))?;

        sqlx::query("CREATE INDEX IF NOT EXISTS idx_sessions_started_at ON sessions(started_at)")
            .execute(&pool)
            .await
            .map_err(|e| format!("Failed to create sessions started_at index: {}", e))?;

        sqlx::query("CREATE INDEX IF NOT EXISTS idx_sessions_is_active ON sessions(is_active)")
            .execute(&pool)
            .await
            .map_err(|e| format!("Failed to create sessions is_active index: {}", e))?;

        // 阶段2: 记忆事件表（替代conversations）
        let create_memory_events_sql = r#"
            CREATE TABLE IF NOT EXISTS memory_events (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                session_id INTEGER,
                agent_name TEXT NOT NULL,
                ts DATETIME DEFAULT (datetime('now')),
                role TEXT NOT NULL,
                text TEXT NOT NULL,
                topic TEXT,
                sentiment REAL,
                importance REAL,
                decay REAL DEFAULT 1.0,
                embedding BLOB,
                FOREIGN KEY (session_id) REFERENCES sessions(id)
            )
        "#;
        log::debug!("Creating memory_events table...");
        sqlx::query(create_memory_events_sql)
            .execute(&pool)
            .await
            .map_err(|e| format!("Failed to create memory_events table: {}", e))?;

        // 记忆事件表索引
        sqlx::query(
            "CREATE INDEX IF NOT EXISTS idx_memory_events_session ON memory_events(session_id)",
        )
        .execute(&pool)
        .await
        .map_err(|e| format!("Failed to create memory_events session index: {}", e))?;

        sqlx::query("CREATE INDEX IF NOT EXISTS idx_memory_events_ts ON memory_events(ts)")
            .execute(&pool)
            .await
            .map_err(|e| format!("Failed to create memory_events ts index: {}", e))?;

        sqlx::query(
            "CREATE INDEX IF NOT EXISTS idx_memory_events_agent ON memory_events(agent_name)",
        )
        .execute(&pool)
        .await
        .map_err(|e| format!("Failed to create memory_events agent index: {}", e))?;

        // 阶段3: 语义片段与图谱
        let create_semantic_chunks_sql = r#"
            CREATE TABLE IF NOT EXISTS semantic_chunks (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT,
                summary TEXT NOT NULL,
                keywords TEXT,
                embedding BLOB,
                last_ref_ts DATETIME,
                weight REAL DEFAULT 1.0
            )
        "#;
        log::debug!("Creating semantic_chunks table...");
        sqlx::query(create_semantic_chunks_sql)
            .execute(&pool)
            .await
            .map_err(|e| format!("Failed to create semantic_chunks table: {}", e))?;

        sqlx::query(
            "CREATE INDEX IF NOT EXISTS idx_semantic_chunks_weight ON semantic_chunks(weight)",
        )
        .execute(&pool)
        .await
        .map_err(|e| format!("Failed to create semantic_chunks weight index: {}", e))?;

        let create_graph_nodes_sql = r#"
            CREATE TABLE IF NOT EXISTS graph_nodes (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                entity_type TEXT NOT NULL,
                entity_name TEXT NOT NULL
            )
        "#;
        log::debug!("Creating graph_nodes table...");
        sqlx::query(create_graph_nodes_sql)
            .execute(&pool)
            .await
            .map_err(|e| format!("Failed to create graph_nodes table: {}", e))?;

        sqlx::query("CREATE UNIQUE INDEX IF NOT EXISTS idx_graph_nodes_unique ON graph_nodes(entity_type, entity_name)")
            .execute(&pool)
            .await
            .map_err(|e| format!("Failed to create graph_nodes unique index: {}", e))?;

        let create_graph_edges_sql = r#"
            CREATE TABLE IF NOT EXISTS graph_edges (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                from_node INTEGER NOT NULL,
                to_node INTEGER NOT NULL,
                relation_type TEXT NOT NULL,
                weight REAL DEFAULT 1.0,
                FOREIGN KEY(from_node) REFERENCES graph_nodes(id),
                FOREIGN KEY(to_node) REFERENCES graph_nodes(id)
            )
        "#;
        log::debug!("Creating graph_edges table...");
        sqlx::query(create_graph_edges_sql)
            .execute(&pool)
            .await
            .map_err(|e| format!("Failed to create graph_edges table: {}", e))?;

        sqlx::query(
            "CREATE INDEX IF NOT EXISTS idx_graph_edges_from_to ON graph_edges(from_node, to_node)",
        )
        .execute(&pool)
        .await
        .map_err(|e| format!("Failed to create graph_edges from_to index: {}", e))?;

        // 语义片段映射表
        let create_semantic_chunk_mappings_sql = r#"
            CREATE TABLE IF NOT EXISTS semantic_chunk_mappings (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                chunk_id INTEGER NOT NULL,
                session_id INTEGER NOT NULL,
                memory_event_ids TEXT NOT NULL,
                created_ts DATETIME DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY(chunk_id) REFERENCES semantic_chunks(id),
                FOREIGN KEY(session_id) REFERENCES sessions(id)
            )
        "#;
        log::debug!("Creating semantic_chunk_mappings table...");
        sqlx::query(create_semantic_chunk_mappings_sql)
            .execute(&pool)
            .await
            .map_err(|e| format!("Failed to create semantic_chunk_mappings table: {}", e))?;

        sqlx::query("CREATE INDEX IF NOT EXISTS idx_semantic_chunk_mappings_chunk_id ON semantic_chunk_mappings(chunk_id)")
            .execute(&pool)
            .await
            .map_err(|e| format!("Failed to create semantic_chunk_mappings chunk_id index: {}", e))?;

        sqlx::query("CREATE INDEX IF NOT EXISTS idx_semantic_chunk_mappings_session_id ON semantic_chunk_mappings(session_id)")
            .execute(&pool)
            .await
            .map_err(|e| format!("Failed to create semantic_chunk_mappings session_id index: {}", e))?;

        // 阶段5: 画像/Persona 表（正式路径，单用户多Agent设计）
        let create_persona_profiles_sql = r#"
            CREATE TABLE IF NOT EXISTS persona_profiles (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                agent_name TEXT UNIQUE NOT NULL,
                created_at DATETIME DEFAULT (datetime('now')),
                updated_at DATETIME
            )
        "#;
        log::debug!("Creating persona_profiles table...");
        sqlx::query(create_persona_profiles_sql)
            .execute(&pool)
            .await
            .map_err(|e| format!("Failed to create persona_profiles table: {}", e))?;

        sqlx::query(
            "CREATE INDEX IF NOT EXISTS idx_persona_profiles_agent ON persona_profiles(agent_name)",
        )
        .execute(&pool)
        .await
        .map_err(|e| format!("Failed to create persona_profiles agent index: {}", e))?;

        let create_persona_traits_sql = r#"
            CREATE TABLE IF NOT EXISTS persona_traits (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                agent_name TEXT NOT NULL,
                trait_type TEXT,
                trait_key TEXT,
                trait_value TEXT,
                confidence REAL DEFAULT 0.5,
                stability REAL DEFAULT 0.0,
                last_seen DATETIME,
                source_event_id INTEGER,
                UNIQUE(agent_name, trait_type, trait_key),
                FOREIGN KEY(source_event_id) REFERENCES memory_events(id)
            )
        "#;
        log::debug!("Creating persona_traits table...");
        sqlx::query(create_persona_traits_sql)
            .execute(&pool)
            .await
            .map_err(|e| format!("Failed to create persona_traits table: {}", e))?;

        sqlx::query(
            "CREATE INDEX IF NOT EXISTS idx_persona_traits_agent ON persona_traits(agent_name)",
        )
        .execute(&pool)
        .await
        .map_err(|e| format!("Failed to create persona_traits agent index: {}", e))?;

        sqlx::query(
            "CREATE INDEX IF NOT EXISTS idx_persona_traits_type ON persona_traits(trait_type)",
        )
        .execute(&pool)
        .await
        .map_err(|e| format!("Failed to create persona_traits type index: {}", e))?;

        sqlx::query(
            "CREATE INDEX IF NOT EXISTS idx_persona_traits_last_seen ON persona_traits(last_seen)",
        )
        .execute(&pool)
        .await
        .map_err(|e| format!("Failed to create persona_traits last_seen index: {}", e))?;

        log::info!("Database tables created successfully");

        Ok(())
    }
}

#[async_trait::async_trait]
impl Database for SqliteDatabase {
    async fn initialize(&mut self) -> DbResult<()> {
        log::info!("Initializing SQLite database with sqlx...");
        self.create_tables().await
    }

    async fn health_check(&self) -> DbResult<bool> {
        log::debug!("Performing database health check...");

        match self.execute("SELECT 1").await {
            Ok(_) => {
                log::debug!("Database health check passed");
                Ok(true)
            }
            Err(e) => {
                log::warn!("Database health check failed: {}", e);
                Ok(false)
            }
        }
    }

    async fn close(&mut self) -> DbResult<()> {
        log::info!("Closing database connection...");

        let mut pool_guard = self.pool.lock().await;
        if let Some(pool) = pool_guard.take() {
            pool.close().await;
            log::info!("Database connection closed");
        }

        Ok(())
    }

    // 阶段1: 单用户会话管理（移除user_id，新增agent_name参数）
    async fn open_session(&self, agent_name: &str, title: Option<&str>) -> DbResult<i64> {
        let pool = self.get_pool().await?;
        // 关闭已有活跃会话（单用户，所以关闭所有活跃会话）
        sqlx::query("UPDATE sessions SET is_active=0, ended_at=datetime('now') WHERE is_active=1")
            .execute(&pool)
            .await
            .map_err(|e| format!("Failed to close previous active session: {}", e))?;
        // 新建会话
        let res = sqlx::query("INSERT INTO sessions (agent_name, title, started_at, is_active) VALUES (?1, ?2, datetime('now'), 1)")
            .bind(agent_name)
            .bind(title.unwrap_or(""))
            .execute(&pool)
            .await
            .map_err(|e| format!("Failed to open session: {}", e))?;
        Ok(res.last_insert_rowid())
    }

    async fn close_active_session(&self) -> DbResult<()> {
        let pool = self.get_pool().await?;
        sqlx::query("UPDATE sessions SET is_active=0, ended_at=datetime('now') WHERE is_active=1")
            .execute(&pool)
            .await
            .map_err(|e| format!("Failed to close active session: {}", e))?;
        Ok(())
    }

    async fn get_active_session(&self) -> DbResult<Option<i64>> {
        let pool = self.get_pool().await?;
        let row = sqlx::query(
            "SELECT id FROM sessions WHERE is_active=1 ORDER BY started_at DESC LIMIT 1",
        )
        .fetch_optional(&pool)
        .await
        .map_err(|e| format!("Failed to get active session: {}", e))?;
        Ok(row.map(|r| r.get::<i64, _>("id")))
    }

    async fn upsert_session_title(&self, session_id: i64, title: &str) -> DbResult<()> {
        let pool = self.get_pool().await?;
        sqlx::query("UPDATE sessions SET title=?1 WHERE id=?2")
            .bind(title)
            .bind(session_id)
            .execute(&pool)
            .await
            .map_err(|e| format!("Failed to update session title: {}", e))?;
        Ok(())
    }

    // 阶段2: 记忆事件管理（替代save_conversation）
    async fn insert_memory_event(&self, event: MemoryEvent) -> DbResult<i64> {
        let pool = self.get_pool().await?;

        let sql = r#"
            INSERT INTO memory_events (session_id, agent_name, role, text, topic, sentiment, importance, decay, embedding, ts)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, datetime('now'))
        "#;

        log::debug!(
            "Inserting memory event for agent: {}, role: {}",
            event.agent_name,
            event.role
        );

        let result = sqlx::query(sql)
            .bind(event.session_id)
            .bind(&event.agent_name)
            .bind(&event.role)
            .bind(&event.text)
            .bind(&event.topic)
            .bind(event.sentiment)
            .bind(event.importance)
            .bind(event.decay)
            .bind(&event.embedding)
            .execute(&pool)
            .await
            .map_err(|e| format!("Failed to insert memory event: {}", e))?;

        log::info!(
            "Memory event saved successfully, ID: {}",
            result.last_insert_rowid()
        );

        Ok(result.last_insert_rowid())
    }

    async fn list_memory_events(&self, session_id: i64) -> DbResult<Vec<MemoryEvent>> {
        let pool = self.get_pool().await?;

        let rows = sqlx::query(r#"
            SELECT session_id, agent_name, role, text, topic, sentiment, importance, decay, embedding
            FROM memory_events 
            WHERE session_id = ?1 
            ORDER BY ts ASC
        "#)
            .bind(session_id)
            .fetch_all(&pool)
            .await
            .map_err(|e| format!("Failed to list memory events: {}", e))?;

        let mut events = Vec::new();
        for row in rows {
            events.push(MemoryEvent {
                session_id: row.get("session_id"),
                agent_name: row.get("agent_name"),
                role: row.get("role"),
                text: row.get("text"),
                topic: row.get("topic"),
                sentiment: row.get("sentiment"),
                importance: row.get("importance"),
                decay: row.get("decay"),
                embedding: row.get("embedding"),
            });
        }

        Ok(events)
    }

    async fn clear_all_memory_events(&self) -> DbResult<()> {
        let pool = self.get_pool().await?;

        sqlx::query("DELETE FROM memory_events")
            .execute(&pool)
            .await
            .map_err(|e| format!("Failed to clear all memory events: {}", e))?;

        log::info!("Cleared all memory events from database");
        Ok(())
    }

    // 阶段3: 语义片段 DAO
    async fn insert_semantic_chunk(&self, chunk: super::SemanticChunk) -> DbResult<i64> {
        let pool = self.get_pool().await?;
        let sql = r#"
            INSERT INTO semantic_chunks (title, summary, keywords, embedding, last_ref_ts, weight)
            VALUES (?1, ?2, ?3, ?4, datetime('now'), COALESCE(?5, 1.0))
        "#;
        let result = sqlx::query(sql)
            .bind(&chunk.title)
            .bind(&chunk.summary)
            .bind(&chunk.keywords)
            .bind(&chunk.embedding)
            .bind(chunk.weight)
            .execute(&pool)
            .await
            .map_err(|e| format!("Failed to insert semantic_chunk: {}", e))?;
        Ok(result.last_insert_rowid())
    }

    async fn list_semantic_chunks(
        &self,
        limit: Option<i32>,
    ) -> DbResult<Vec<super::SemanticChunk>> {
        let pool = self.get_pool().await?;
        let sql = if limit.is_some() {
            "SELECT id, title, summary, keywords, embedding, last_ref_ts, weight FROM semantic_chunks ORDER BY (last_ref_ts IS NULL), last_ref_ts DESC, id DESC LIMIT ?1"
        } else {
            "SELECT id, title, summary, keywords, embedding, last_ref_ts, weight FROM semantic_chunks ORDER BY (last_ref_ts IS NULL), last_ref_ts DESC, id DESC"
        };
        let mut q = sqlx::query(sql);
        if let Some(lim) = limit {
            q = q.bind(lim);
        }
        let rows = q
            .fetch_all(&pool)
            .await
            .map_err(|e| format!("Failed to list semantic_chunks: {}", e))?;
        let mut chunks = Vec::new();
        for row in rows {
            chunks.push(super::SemanticChunk {
                id: row.get("id"),
                title: row.get("title"),
                summary: row.get("summary"),
                keywords: row.get("keywords"),
                embedding: row.get("embedding"),
                last_ref_ts: row.get("last_ref_ts"),
                weight: row.get("weight"),
            });
        }
        Ok(chunks)
    }

    async fn update_semantic_chunk_ref_time(&self, chunk_id: i64) -> DbResult<()> {
        let pool = self.get_pool().await?;
        sqlx::query("UPDATE semantic_chunks SET last_ref_ts=datetime('now') WHERE id=?1")
            .bind(chunk_id)
            .execute(&pool)
            .await
            .map_err(|e| format!("Failed to update semantic_chunk ref time: {}", e))?;
        Ok(())
    }

    // 阶段3: 图谱 DAO
    async fn upsert_graph_node(&self, node: super::GraphNode) -> DbResult<i64> {
        let pool = self.get_pool().await?;
        // 尝试查找
        if let Some(existing) = self
            .get_node_by_entity(&node.entity_type, &node.entity_name)
            .await?
        {
            return Ok(existing.id.unwrap());
        }
        // 插入
        let res = sqlx::query("INSERT INTO graph_nodes (entity_type, entity_name) VALUES (?1, ?2)")
            .bind(&node.entity_type)
            .bind(&node.entity_name)
            .execute(&pool)
            .await
            .map_err(|e| format!("Failed to insert graph_node: {}", e))?;
        Ok(res.last_insert_rowid())
    }

    async fn upsert_graph_edge(&self, edge: super::GraphEdge) -> DbResult<i64> {
        let pool = self.get_pool().await?;
        // 边不做严格唯一约束（允许多次强化），但可尝试查重
        let row = sqlx::query(
            "SELECT id FROM graph_edges WHERE from_node=?1 AND to_node=?2 AND relation_type=?3",
        )
        .bind(edge.from_node)
        .bind(edge.to_node)
        .bind(&edge.relation_type)
        .fetch_optional(&pool)
        .await
        .map_err(|e| format!("Failed to query graph_edge: {}", e))?;
        if let Some(r) = row {
            return Ok(r.get::<i64, _>("id"));
        }
        let res = sqlx::query(
            "INSERT INTO graph_edges (from_node, to_node, relation_type, weight) VALUES (?1, ?2, ?3, COALESCE(?4, 1.0))"
        )
        .bind(edge.from_node)
        .bind(edge.to_node)
        .bind(&edge.relation_type)
        .bind(edge.weight)
        .execute(&pool)
        .await
        .map_err(|e| format!("Failed to insert graph_edge: {}", e))?;
        Ok(res.last_insert_rowid())
    }

    async fn get_graph_nodes(&self) -> DbResult<Vec<super::GraphNode>> {
        let pool = self.get_pool().await?;
        let rows = sqlx::query("SELECT id, entity_type, entity_name FROM graph_nodes")
            .fetch_all(&pool)
            .await
            .map_err(|e| format!("Failed to list graph_nodes: {}", e))?;
        let mut nodes = Vec::new();
        for row in rows {
            nodes.push(super::GraphNode {
                id: row.get("id"),
                entity_type: row.get("entity_type"),
                entity_name: row.get("entity_name"),
            });
        }
        Ok(nodes)
    }

    async fn get_graph_edges(&self) -> DbResult<Vec<super::GraphEdge>> {
        let pool = self.get_pool().await?;
        let rows =
            sqlx::query("SELECT id, from_node, to_node, relation_type, weight FROM graph_edges")
                .fetch_all(&pool)
                .await
                .map_err(|e| format!("Failed to list graph_edges: {}", e))?;
        let mut edges = Vec::new();
        for row in rows {
            edges.push(super::GraphEdge {
                id: row.get("id"),
                from_node: row.get("from_node"),
                to_node: row.get("to_node"),
                relation_type: row.get("relation_type"),
                weight: row.get("weight"),
            });
        }
        Ok(edges)
    }

    async fn get_node_by_entity(
        &self,
        entity_type: &str,
        entity_name: &str,
    ) -> DbResult<Option<super::GraphNode>> {
        let pool = self.get_pool().await?;
        let rows = sqlx::query("SELECT id, entity_type, entity_name FROM graph_nodes WHERE entity_type = ?1 AND entity_name = ?2")
            .bind(entity_type)
            .bind(entity_name)
            .fetch_all(&pool)
            .await?;
        if let Some(row) = rows.into_iter().next() {
            Ok(Some(super::GraphNode {
                id: Some(row.get("id")),
                entity_type: row.get("entity_type"),
                entity_name: row.get("entity_name"),
            }))
        } else {
            Ok(None)
        }
    }

    // 阶段5: 画像/Persona 管理
    async fn upsert_persona_profile(&self, profile: super::PersonaProfile) -> DbResult<i64> {
        let pool = self.get_pool().await?;
        // Upsert by agent_name
        sqlx::query("INSERT INTO persona_profiles (agent_name, created_at, updated_at) VALUES (?1, datetime('now'), datetime('now')) \
                     ON CONFLICT(agent_name) DO UPDATE SET updated_at = excluded.updated_at")
            .bind(&profile.agent_name)
            .execute(&pool)
            .await?;
        // Return id
        let row = sqlx::query("SELECT id FROM persona_profiles WHERE agent_name = ?1")
            .bind(&profile.agent_name)
            .fetch_one(&pool)
            .await?;
        Ok(row.get::<i64, _>("id"))
    }

    async fn get_persona_profile(
        &self,
        agent_name: &str,
    ) -> DbResult<Option<super::PersonaProfile>> {
        let pool = self.get_pool().await?;
        let rows = sqlx::query("SELECT id, agent_name, created_at, updated_at FROM persona_profiles WHERE agent_name = ?1")
            .bind(agent_name)
            .fetch_all(&pool)
            .await?;
        if let Some(row) = rows.into_iter().next() {
            Ok(Some(super::PersonaProfile {
                id: Some(row.get("id")),
                agent_name: row.get("agent_name"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            }))
        } else {
            Ok(None)
        }
    }

    async fn upsert_persona_trait(&self, trait_item: super::PersonaTrait) -> DbResult<i64> {
        let pool = self.get_pool().await?;
        // Upsert on (agent_name, trait_type, trait_key)
        sqlx::query(
            "INSERT INTO persona_traits (agent_name, trait_type, trait_key, trait_value, confidence, stability, last_seen, source_event_id) \
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, datetime('now'), ?7) \
             ON CONFLICT(agent_name, trait_type, trait_key) DO UPDATE SET \
                trait_value=excluded.trait_value, \
                confidence=excluded.confidence, \
                stability=excluded.stability, \
                last_seen=excluded.last_seen, \
                source_event_id=excluded.source_event_id"
        )
        .bind(&trait_item.agent_name)
        .bind(&trait_item.trait_type)
        .bind(&trait_item.trait_key)
        .bind(&trait_item.trait_value)
        .bind(trait_item.confidence)
        .bind(trait_item.stability)
        .bind(trait_item.source_event_id)
        .execute(&pool)
        .await?;

        let row = sqlx::query("SELECT id FROM persona_traits WHERE agent_name = ?1 AND trait_type = ?2 AND trait_key = ?3")
            .bind(&trait_item.agent_name)
            .bind(&trait_item.trait_type)
            .bind(&trait_item.trait_key)
            .fetch_one(&pool)
            .await?;
        Ok(row.get::<i64, _>("id"))
    }

    async fn list_persona_traits(
        &self,
        agent_name: &str,
        trait_type: Option<&str>,
        top_k: Option<usize>,
    ) -> DbResult<Vec<super::PersonaTrait>> {
        let pool = self.get_pool().await?;
        let mut query = String::from("SELECT id, agent_name, trait_type, trait_key, trait_value, confidence, stability, last_seen, source_event_id FROM persona_traits WHERE agent_name = ?1");
        if trait_type.is_some() {
            query.push_str(" AND trait_type = ?2");
        }
        query.push_str(" ORDER BY stability DESC, confidence DESC, last_seen DESC");
        if let Some(k) = top_k {
            query.push_str(&format!(" LIMIT {}", k));
        }
        let mut q = sqlx::query(&query).bind(agent_name);
        if let Some(t) = trait_type {
            q = q.bind(t);
        }
        let rows = q.fetch_all(&pool).await?;
        let mut res = Vec::new();
        for row in rows {
            res.push(super::PersonaTrait {
                id: Some(row.get("id")),
                agent_name: row.get("agent_name"),
                trait_type: row.get("trait_type"),
                trait_key: row.get("trait_key"),
                trait_value: row.get("trait_value"),
                confidence: row.get("confidence"),
                stability: row.get("stability"),
                last_seen: row.get("last_seen"),
                source_event_id: row.get("source_event_id"),
            });
        }
        Ok(res)
    }

    async fn get_relevant_persona_facts(
        &self,
        agent_name: &str,
        query: &str,
        top_k: usize,
    ) -> DbResult<Vec<super::PersonaTrait>> {
        let pool = self.get_pool().await?;
        // 先用简单的 LIKE 匹配 trait_key/value，后续可以接入嵌入相似度或规则
        let like = format!("%{}%", query);
        let rows = sqlx::query(
            "SELECT id, agent_name, trait_type, trait_key, trait_value, confidence, stability, last_seen, source_event_id \
             FROM persona_traits WHERE agent_name = ?1 AND (trait_key LIKE ?2 OR trait_value LIKE ?2) \
             ORDER BY stability DESC, confidence DESC, last_seen DESC LIMIT ?3"
        )
        .bind(agent_name)
        .bind(like)
        .bind(top_k as i64)
        .fetch_all(&pool)
        .await?;
        let mut res = Vec::new();
        for row in rows {
            res.push(super::PersonaTrait {
                id: Some(row.get("id")),
                agent_name: row.get("agent_name"),
                trait_type: row.get("trait_type"),
                trait_key: row.get("trait_key"),
                trait_value: row.get("trait_value"),
                confidence: row.get("confidence"),
                stability: row.get("stability"),
                last_seen: row.get("last_seen"),
                source_event_id: row.get("source_event_id"),
            });
        }
        Ok(res)
    }

    // 语义片段映射管理
    async fn insert_semantic_chunk_mapping(
        &self,
        mapping: super::SemanticChunkMapping,
    ) -> DbResult<i64> {
        let pool = self.get_pool().await?;
        let res = sqlx::query(
            "INSERT INTO semantic_chunk_mappings (chunk_id, session_id, memory_event_ids) VALUES (?1, ?2, ?3)"
        )
        .bind(mapping.chunk_id)
        .bind(mapping.session_id)
        .bind(&mapping.memory_event_ids)
        .execute(&pool)
        .await
        .map_err(|e| format!("Failed to insert semantic_chunk_mapping: {}", e))?;
        Ok(res.last_insert_rowid())
    }

    async fn get_chunk_mappings_by_chunk_id(
        &self,
        chunk_id: i64,
    ) -> DbResult<Vec<super::SemanticChunkMapping>> {
        let pool = self.get_pool().await?;
        let rows = sqlx::query(
            "SELECT id, chunk_id, session_id, memory_event_ids, created_ts FROM semantic_chunk_mappings WHERE chunk_id = ?1"
        )
        .bind(chunk_id)
        .fetch_all(&pool)
        .await
        .map_err(|e| format!("Failed to get chunk mappings by chunk_id: {}", e))?;

        let mut mappings = Vec::new();
        for row in rows {
            mappings.push(super::SemanticChunkMapping {
                id: row.get("id"),
                chunk_id: row.get("chunk_id"),
                session_id: row.get("session_id"),
                memory_event_ids: row.get("memory_event_ids"),
                created_ts: row.get("created_ts"),
            });
        }
        Ok(mappings)
    }

    async fn get_chunk_mappings_by_session_id(
        &self,
        session_id: i64,
    ) -> DbResult<Vec<super::SemanticChunkMapping>> {
        let pool = self.get_pool().await?;
        let rows = sqlx::query(
            "SELECT id, chunk_id, session_id, memory_event_ids, created_ts FROM semantic_chunk_mappings WHERE session_id = ?1"
        )
        .bind(session_id)
        .fetch_all(&pool)
        .await
        .map_err(|e| format!("Failed to get chunk mappings by session_id: {}", e))?;

        let mut mappings = Vec::new();
        for row in rows {
            mappings.push(super::SemanticChunkMapping {
                id: row.get("id"),
                chunk_id: row.get("chunk_id"),
                session_id: row.get("session_id"),
                memory_event_ids: row.get("memory_event_ids"),
                created_ts: row.get("created_ts"),
            });
        }
        Ok(mappings)
    }

    /// 累积边权重版本的 upsert_graph_edge
    async fn upsert_graph_edge_with_accumulation(&self, edge: super::GraphEdge) -> DbResult<i64> {
        let pool = self.get_pool().await?;

        // 查找现有边
        let existing_row = sqlx::query(
            "SELECT id, weight FROM graph_edges WHERE from_node=?1 AND to_node=?2 AND relation_type=?3"
        )
        .bind(edge.from_node)
        .bind(edge.to_node)
        .bind(&edge.relation_type)
        .fetch_optional(&pool)
        .await
        .map_err(|e| format!("Failed to query graph_edge for accumulation: {}", e))?;

        if let Some(row) = existing_row {
            // 累积权重
            let existing_id: i64 = row.get("id");
            let existing_weight: f32 = row.get("weight");
            let new_weight = existing_weight + edge.weight;

            sqlx::query("UPDATE graph_edges SET weight = ?1 WHERE id = ?2")
                .bind(new_weight)
                .bind(existing_id)
                .execute(&pool)
                .await
                .map_err(|e| format!("Failed to update graph_edge weight: {}", e))?;

            Ok(existing_id)
        } else {
            // 插入新边
            let res = sqlx::query(
                "INSERT INTO graph_edges (from_node, to_node, relation_type, weight) VALUES (?1, ?2, ?3, ?4)"
            )
            .bind(edge.from_node)
            .bind(edge.to_node)
            .bind(&edge.relation_type)
            .bind(edge.weight)
            .execute(&pool)
            .await
            .map_err(|e| format!("Failed to insert graph_edge: {}", e))?;
            Ok(res.last_insert_rowid())
        }
    }

    async fn migrate(&mut self) -> DbResult<()> {
        log::info!("Running SQLite migrations (idempotent)...");
        self.create_tables().await
    }

    async fn backup(&self, backup_path: &str) -> DbResult<()> {
        let path = std::path::Path::new(backup_path);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create backup directory: {}", e))?;
        }
        // Use SQLite VACUUM INTO to create a consistent backup even with WAL
        let pool = self.get_pool().await?;
        let dest = path.to_string_lossy().replace('\'', "''");
        let sql = format!("VACUUM INTO '{}'", dest);
        log::info!("Backing up SQLite database to {}", path.display());
        sqlx::query(&sql)
            .execute(&pool)
            .await
            .map_err(|e| format!("Failed to backup database: {}", e))?;
        Ok(())
    }
    async fn as_sqlite(&self) -> DbResult<Option<&dyn std::any::Any>> {
        Ok(Some(self as &dyn std::any::Any))
    }
}

//! 数据库模�?//!
//! 本模块提供向量数据库和图数据库的统一接口，支持SQLite、PostgreSQL和MySQL�?//! 包含向量存储、图节点和边的管理、以及高效的查询功能�?
use crate::config::{Config, DatabaseType};
use crate::error::{MemoryError, Result};
use chrono::{DateTime, Utc};
use lru::LruCache;
use serde::{Deserialize, Serialize};

use sqlx::{Pool, Row, Sqlite};
use std::collections::HashMap;
use std::num::NonZeroUsize;
use std::sync::Arc;
use tokio::sync::RwLock;

/// 数据库连接池枚举
#[derive(Debug, Clone)]
pub enum DatabasePool {
    Sqlite(Pool<Sqlite>),
}

/// 向量数据结构
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Vector {
    pub id: String,
    pub embedding: Vec<f32>,
    pub metadata: HashMap<String, serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 图节点数据结构
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GraphNode {
    pub id: String,
    pub node_type: String,
    pub properties: HashMap<String, serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 图边数据结构
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GraphEdge {
    pub id: String,
    pub from_node: String,
    pub to_node: String,
    pub edge_type: String,
    pub weight: f32,
    pub properties: HashMap<String, serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 向量查询请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorQueryRequest {
    pub query_vector: Vec<f32>,
    pub limit: Option<usize>,
    pub threshold: Option<f32>,
    pub filters: Option<HashMap<String, serde_json::Value>>,
}

/// 向量查询结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorQueryResult {
    pub vector: Vector,
    pub similarity: f32,
    pub distance: f32,
}

/// 图查询请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphQueryRequest {
    pub start_nodes: Vec<String>,
    pub edge_types: Option<Vec<String>>,
    pub max_depth: Option<usize>,
    pub limit: Option<usize>,
    pub filters: Option<HashMap<String, serde_json::Value>>,
}

/// 图查询结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphQueryResult {
    pub nodes: Vec<GraphNode>,
    pub edges: Vec<GraphEdge>,
    pub paths: Vec<Vec<String>>,
}

/// 内存缓存
#[derive(Debug)]
pub struct MemoryCache {
    vectors: Arc<RwLock<LruCache<String, Vector>>>,
    nodes: Arc<RwLock<LruCache<String, GraphNode>>>,
    edges: Arc<RwLock<LruCache<String, GraphEdge>>>,
    query_cache: Arc<RwLock<LruCache<String, VectorQueryResult>>>,
}

impl MemoryCache {
    pub fn new(capacity: usize) -> Self {
        let cache_capacity =
            NonZeroUsize::new(capacity).unwrap_or(NonZeroUsize::new(1000).unwrap());

        Self {
            vectors: Arc::new(RwLock::new(LruCache::new(cache_capacity))),
            nodes: Arc::new(RwLock::new(LruCache::new(cache_capacity))),
            edges: Arc::new(RwLock::new(LruCache::new(cache_capacity))),
            query_cache: Arc::new(RwLock::new(LruCache::new(cache_capacity))),
        }
    }

    pub async fn get_vector(&self, id: &str) -> Option<Vector> {
        self.vectors.read().await.peek(id).cloned()
    }

    pub async fn put_vector(&self, id: String, vector: Vector) {
        self.vectors.write().await.put(id, vector);
    }

    pub async fn get_node(&self, id: &str) -> Option<GraphNode> {
        self.nodes.read().await.peek(id).cloned()
    }

    pub async fn put_node(&self, id: String, node: GraphNode) {
        self.nodes.write().await.put(id, node);
    }

    pub async fn get_edge(&self, id: &str) -> Option<GraphEdge> {
        self.edges.read().await.peek(id).cloned()
    }

    pub async fn put_edge(&self, id: String, edge: GraphEdge) {
        self.edges.write().await.put(id, edge);
    }

    pub async fn clear(&self) {
        self.vectors.write().await.clear();
        self.nodes.write().await.clear();
        self.edges.write().await.clear();
        self.query_cache.write().await.clear();
    }

    pub async fn get_stats(&self) -> (usize, usize, usize, usize) {
        let vectors_len = self.vectors.read().await.len();
        let nodes_len = self.nodes.read().await.len();
        let edges_len = self.edges.read().await.len();
        let query_cache_len = self.query_cache.read().await.len();
        (vectors_len, nodes_len, edges_len, query_cache_len)
    }
}

/// 向量图数据库
#[derive(Debug)]
pub struct VectorGraphDB {
    pool: DatabasePool,
    cache: MemoryCache,
    config: Config,
}

impl VectorGraphDB {
    /// 创建新的数据库实例
    pub async fn new(config: Config) -> Result<Self> {
        let pool = Self::create_pool(&config).await?;
        let cache = MemoryCache::new(config.cache.lru_capacity);

        let db = Self {
            pool,
            cache,
            config,
        };

        // 初始化数据库表
        db.initialize_tables().await?;

        Ok(db)
    }

    /// 创建数据库连接池
    async fn create_pool(config: &Config) -> Result<DatabasePool> {
        match config.database.database_type {
            DatabaseType::SQLite => {
                let pool = sqlx::SqlitePool::connect(&config.database.url)
                    .await
                    .map_err(MemoryError::Database)?;
                Ok(DatabasePool::Sqlite(pool))
            }
        }
    }

    /// 初始化数据库表
    async fn initialize_tables(&self) -> Result<()> {
        match &self.pool {
            DatabasePool::Sqlite(pool) => {
                self.create_sqlite_tables(pool).await?;
            }
        }
        Ok(())
    }

    /// 创建SQLite表
    async fn create_sqlite_tables(&self, pool: &Pool<Sqlite>) -> Result<()> {
        let prefix = &self.config.database.table_prefix;

        // 向量表
        sqlx::query(&format!(
            r#"
            CREATE TABLE IF NOT EXISTS {}vectors (
                id TEXT PRIMARY KEY,
                embedding BLOB NOT NULL,
                metadata TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )
            "#,
            prefix
        ))
        .execute(pool)
        .await
        .map_err(MemoryError::Database)?;

        // 图节点表
        sqlx::query(&format!(
            r#"
            CREATE TABLE IF NOT EXISTS {}graph_nodes (
                id TEXT PRIMARY KEY,
                node_type TEXT NOT NULL,
                properties TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )
            "#,
            prefix
        ))
        .execute(pool)
        .await
        .map_err(MemoryError::Database)?;

        // 图边表
        sqlx::query(&format!(
            r#"
            CREATE TABLE IF NOT EXISTS {}graph_edges (
                id TEXT PRIMARY KEY,
                from_node TEXT NOT NULL,
                to_node TEXT NOT NULL,
                edge_type TEXT NOT NULL,
                weight REAL NOT NULL,
                properties TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                FOREIGN KEY (from_node) REFERENCES {}graph_nodes(id),
                FOREIGN KEY (to_node) REFERENCES {}graph_nodes(id)
            )
            "#,
            prefix, prefix, prefix
        ))
        .execute(pool)
        .await
        .map_err(MemoryError::Database)?;

        // 创建索引
        self.create_sqlite_indexes(pool).await?;

        Ok(())
    }

    /// 创建SQLite索引
    async fn create_sqlite_indexes(&self, pool: &Pool<Sqlite>) -> Result<()> {
        let prefix = &self.config.database.table_prefix;

        let indexes = vec![
            format!("CREATE INDEX IF NOT EXISTS idx_{prefix}vectors_created_at ON {prefix}vectors(created_at)"),
            format!("CREATE INDEX IF NOT EXISTS idx_{prefix}nodes_type ON {prefix}graph_nodes(node_type)"),
            format!("CREATE INDEX IF NOT EXISTS idx_{prefix}nodes_created_at ON {prefix}graph_nodes(created_at)"),
            format!("CREATE INDEX IF NOT EXISTS idx_{prefix}edges_from_node ON {prefix}graph_edges(from_node)"),
            format!("CREATE INDEX IF NOT EXISTS idx_{prefix}edges_to_node ON {prefix}graph_edges(to_node)"),
            format!("CREATE INDEX IF NOT EXISTS idx_{prefix}edges_type ON {prefix}graph_edges(edge_type)"),
            format!("CREATE INDEX IF NOT EXISTS idx_{prefix}edges_weight ON {prefix}graph_edges(weight)"),
        ];

        for index_sql in indexes {
            sqlx::query(&index_sql)
                .execute(pool)
                .await
                .map_err(MemoryError::Database)?;
        }

        Ok(())
    }

    /// 插入向量
    pub async fn insert_vector(&self, vector: &Vector) -> Result<()> {
        let embedding_bytes = self.vector_to_bytes(&vector.embedding)?;
        let metadata_json =
            serde_json::to_string(&vector.metadata).map_err(MemoryError::Serialization)?;

        match &self.pool {
            DatabasePool::Sqlite(pool) => {
                sqlx::query(&format!(
                    "INSERT INTO {}vectors (id, embedding, metadata, created_at, updated_at) VALUES (?, ?, ?, ?, ?)",
                    self.config.database.table_prefix
                ))
                .bind(&vector.id)
                .bind(&embedding_bytes)
                .bind(&metadata_json)
                .bind(vector.created_at.to_rfc3339())
                .bind(vector.updated_at.to_rfc3339())
                .execute(pool)
                .await
                .map_err(MemoryError::Database)?;
            }
        }

        // 更新缓存
        self.cache
            .put_vector(vector.id.clone(), vector.clone())
            .await;

        Ok(())
    }

    /// 查询向量
    pub async fn query_vectors(
        &self,
        request: &VectorQueryRequest,
    ) -> Result<Vec<VectorQueryResult>> {
        let limit = request.limit.unwrap_or(10);
        let threshold = request.threshold.unwrap_or(0.0);

        // 从数据库获取所有向量（在实际应用中应该使用向量索引）
        let vectors = self.get_all_vectors().await?;

        let mut results = Vec::new();

        for vector in vectors {
            let similarity = self.cosine_similarity(&request.query_vector, &vector.embedding);

            if similarity >= threshold {
                // 应用过滤器
                if let Some(filters) = &request.filters {
                    if !self.matches_filters(&vector.metadata, filters) {
                        continue;
                    }
                }

                results.push(VectorQueryResult {
                    vector,
                    similarity,
                    distance: 1.0 - similarity,
                });
            }
        }

        // 按相似度排序
        results.sort_by(|a, b| b.similarity.partial_cmp(&a.similarity).unwrap());
        results.truncate(limit);

        Ok(results)
    }

    /// 获取所有向量
    async fn get_all_vectors(&self) -> Result<Vec<Vector>> {
        let mut vectors = Vec::new();

        match &self.pool {
            DatabasePool::Sqlite(pool) => {
                let rows = sqlx::query(&format!(
                    "SELECT id, embedding, metadata, created_at, updated_at FROM {}vectors",
                    self.config.database.table_prefix
                ))
                .fetch_all(pool)
                .await
                .map_err(MemoryError::Database)?;

                for row in rows {
                    let id: String = row.get("id");
                    let embedding_bytes: Vec<u8> = row.get("embedding");
                    let metadata_str: String = row.get("metadata");
                    let created_at_str: String = row.get("created_at");
                    let updated_at_str: String = row.get("updated_at");

                    let embedding = self.bytes_to_vector(&embedding_bytes)?;
                    let metadata: HashMap<String, serde_json::Value> =
                        serde_json::from_str(&metadata_str).map_err(MemoryError::Serialization)?;
                    let created_at = DateTime::parse_from_rfc3339(&created_at_str)
                        .map_err(|e| MemoryError::Internal {
                            message: format!("DateTime parse failed: {}", e),
                        })?
                        .with_timezone(&Utc);
                    let updated_at = DateTime::parse_from_rfc3339(&updated_at_str)
                        .map_err(|e| MemoryError::Internal {
                            message: format!("DateTime parse failed: {}", e),
                        })?
                        .with_timezone(&Utc);

                    vectors.push(Vector {
                        id,
                        embedding,
                        metadata,
                        created_at,
                        updated_at,
                    });
                }
            }
        }

        Ok(vectors)
    }

    /// 插入图节点
    pub async fn insert_node(&self, node: &GraphNode) -> Result<()> {
        let properties_json =
            serde_json::to_string(&node.properties).map_err(MemoryError::Serialization)?;

        match &self.pool {
            DatabasePool::Sqlite(pool) => {
                sqlx::query(&format!(
                    "INSERT INTO {}graph_nodes (id, node_type, properties, created_at, updated_at) VALUES (?, ?, ?, ?, ?)",
                    self.config.database.table_prefix
                ))
                .bind(&node.id)
                .bind(&node.node_type)
                .bind(&properties_json)
                .bind(node.created_at.to_rfc3339())
                .bind(node.updated_at.to_rfc3339())
                .execute(pool)
                .await
                .map_err(MemoryError::Database)?;
            }
        }

        // 更新缓存
        self.cache.put_node(node.id.clone(), node.clone()).await;

        Ok(())
    }

    /// 插入图边
    pub async fn insert_edge(&self, edge: &GraphEdge) -> Result<()> {
        let properties_json =
            serde_json::to_string(&edge.properties).map_err(MemoryError::Serialization)?;

        match &self.pool {
            DatabasePool::Sqlite(pool) => {
                sqlx::query(&format!(
                    "INSERT INTO {}graph_edges (id, from_node, to_node, edge_type, weight, properties, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
                    self.config.database.table_prefix
                ))
                .bind(&edge.id)
                .bind(&edge.from_node)
                .bind(&edge.to_node)
                .bind(&edge.edge_type)
                .bind(edge.weight)
                .bind(&properties_json)
                .bind(edge.created_at.to_rfc3339())
                .bind(edge.updated_at.to_rfc3339())
                .execute(pool)
                .await
                .map_err(MemoryError::Database)?;
            }
        }

        // 更新缓存
        self.cache.put_edge(edge.id.clone(), edge.clone()).await;

        Ok(())
    }

    /// 查询图
    pub async fn query_graph(&self, request: &GraphQueryRequest) -> Result<GraphQueryResult> {
        let mut nodes = Vec::new();
        let mut edges = Vec::new();
        let mut paths = Vec::new();

        // 简单的图遍历实现（在实际应用中应该使用更高效的算法）
        for start_node in &request.start_nodes {
            let (node_results, edge_results, path_results) = self
                .traverse_graph(
                    start_node,
                    request.max_depth.unwrap_or(3),
                    &request.edge_types,
                    &request.filters,
                )
                .await?;

            nodes.extend(node_results);
            edges.extend(edge_results);
            paths.extend(path_results);
        }

        // 去重
        nodes.sort_by(|a, b| a.id.cmp(&b.id));
        nodes.dedup_by(|a, b| a.id == b.id);

        edges.sort_by(|a, b| a.id.cmp(&b.id));
        edges.dedup_by(|a, b| a.id == b.id);

        // 应用限制
        if let Some(limit) = request.limit {
            nodes.truncate(limit);
            edges.truncate(limit);
            paths.truncate(limit);
        }

        Ok(GraphQueryResult {
            nodes,
            edges,
            paths,
        })
    }

    /// 图遍历
    async fn traverse_graph(
        &self,
        start_node: &str,
        max_depth: usize,
        edge_types: &Option<Vec<String>>,
        filters: &Option<HashMap<String, serde_json::Value>>,
    ) -> Result<(Vec<GraphNode>, Vec<GraphEdge>, Vec<Vec<String>>)> {
        let mut visited_nodes = std::collections::HashSet::new();
        let mut result_nodes = Vec::new();
        let mut result_edges = Vec::new();
        let mut paths = Vec::new();

        let mut queue = std::collections::VecDeque::new();
        queue.push_back((start_node.to_string(), 0, vec![start_node.to_string()]));

        while let Some((current_node, depth, path)) = queue.pop_front() {
            if depth > max_depth || visited_nodes.contains(&current_node) {
                continue;
            }

            visited_nodes.insert(current_node.clone());

            // 获取当前节点
            if let Ok(node) = self.get_node(&current_node).await {
                // 应用过滤器
                if let Some(filters) = filters {
                    if !self.matches_filters(&node.properties, filters) {
                        continue;
                    }
                }
                result_nodes.push(node);
            }

            // 获取相邻边
            let adjacent_edges = self.get_adjacent_edges(&current_node, edge_types).await?;

            for edge in adjacent_edges {
                result_edges.push(edge.clone());

                let next_node = if edge.from_node == current_node {
                    &edge.to_node
                } else {
                    &edge.from_node
                };

                if !visited_nodes.contains(next_node) {
                    let mut new_path = path.clone();
                    new_path.push(next_node.clone());
                    queue.push_back((next_node.clone(), depth + 1, new_path.clone()));

                    if depth < max_depth {
                        paths.push(new_path);
                    }
                }
            }
        }

        Ok((result_nodes, result_edges, paths))
    }

    /// 更新向量
    pub async fn update_vector(&self, vector: &Vector) -> Result<()> {
        let embedding_bytes = self.vector_to_bytes(&vector.embedding)?;
        let metadata_json =
            serde_json::to_string(&vector.metadata).map_err(MemoryError::Serialization)?;

        match &self.pool {
            DatabasePool::Sqlite(pool) => {
                sqlx::query(&format!(
                    "UPDATE {}vectors SET embedding = ?, metadata = ?, updated_at = ? WHERE id = ?",
                    self.config.database.table_prefix
                ))
                .bind(&embedding_bytes)
                .bind(&metadata_json)
                .bind(vector.updated_at.to_rfc3339())
                .bind(&vector.id)
                .execute(pool)
                .await
                .map_err(MemoryError::Database)?;
            }
        }

        // 更新缓存
        self.cache
            .put_vector(vector.id.clone(), vector.clone())
            .await;

        Ok(())
    }

    /// 更新节点
    pub async fn update_node(&self, node: &GraphNode) -> Result<()> {
        let properties_json =
            serde_json::to_string(&node.properties).map_err(MemoryError::Serialization)?;

        match &self.pool {
            DatabasePool::Sqlite(pool) => {
                sqlx::query(&format!(
                    "UPDATE {}graph_nodes SET node_type = ?, properties = ?, updated_at = ? WHERE id = ?",
                    self.config.database.table_prefix
                ))
                .bind(&node.node_type)
                .bind(&properties_json)
                .bind(node.updated_at.to_rfc3339())
                .bind(&node.id)
                .execute(pool)
                .await
                .map_err(MemoryError::Database)?;
            }
        }

        // 更新缓存
        self.cache.put_node(node.id.clone(), node.clone()).await;

        Ok(())
    }

    /// 获取向量
    pub async fn get_vector(&self, vector_id: &str) -> Result<Vector> {
        // 先检查缓存
        if let Some(vector) = self.cache.get_vector(vector_id).await {
            return Ok(vector);
        }

        match &self.pool {
            DatabasePool::Sqlite(pool) => {
                let row = sqlx::query(&format!(
                    "SELECT id, embedding, metadata, created_at, updated_at FROM {}vectors WHERE id = ?",
                    self.config.database.table_prefix
                ))
                .bind(vector_id)
                .fetch_one(pool)
                .await
                .map_err(MemoryError::Database)?;

                let id: String = row.get("id");
                let embedding_bytes: Vec<u8> = row.get("embedding");
                let metadata_str: String = row.get("metadata");
                let created_at_str: String = row.get("created_at");
                let updated_at_str: String = row.get("updated_at");

                let embedding = self.bytes_to_vector(&embedding_bytes)?;
                let metadata: HashMap<String, serde_json::Value> =
                    serde_json::from_str(&metadata_str).map_err(MemoryError::Serialization)?;
                let created_at = DateTime::parse_from_rfc3339(&created_at_str)
                    .map_err(|e| MemoryError::Internal {
                        message: format!("DateTime parse failed: {}", e),
                    })?
                    .with_timezone(&Utc);
                let updated_at = DateTime::parse_from_rfc3339(&updated_at_str)
                    .map_err(|e| MemoryError::Internal {
                        message: format!("DateTime parse failed: {}", e),
                    })?
                    .with_timezone(&Utc);

                let vector = Vector {
                    id,
                    embedding,
                    metadata,
                    created_at,
                    updated_at,
                };

                // 更新缓存
                self.cache
                    .put_vector(vector.id.clone(), vector.clone())
                    .await;

                Ok(vector)
            }
        }
    }

    /// 获取节点
    async fn get_node(&self, node_id: &str) -> Result<GraphNode> {
        // 先检查缓存
        if let Some(node) = self.cache.get_node(node_id).await {
            return Ok(node);
        }

        match &self.pool {
            DatabasePool::Sqlite(pool) => {
                let row = sqlx::query(&format!(
                    "SELECT id, node_type, properties, created_at, updated_at FROM {}graph_nodes WHERE id = ?",
                    self.config.database.table_prefix
                ))
                .bind(node_id)
                .fetch_one(pool)
                .await
                .map_err(MemoryError::Database)?;

                let id: String = row.get("id");
                let node_type: String = row.get("node_type");
                let properties_str: String = row.get("properties");
                let created_at_str: String = row.get("created_at");
                let updated_at_str: String = row.get("updated_at");

                let properties: HashMap<String, serde_json::Value> =
                    serde_json::from_str(&properties_str).map_err(MemoryError::Serialization)?;
                let created_at = DateTime::parse_from_rfc3339(&created_at_str)
                    .map_err(|e| MemoryError::Internal {
                        message: format!("DateTime parse failed: {}", e),
                    })?
                    .with_timezone(&Utc);
                let updated_at = DateTime::parse_from_rfc3339(&updated_at_str)
                    .map_err(|e| MemoryError::Internal {
                        message: format!("DateTime parse failed: {}", e),
                    })?
                    .with_timezone(&Utc);

                let node = GraphNode {
                    id,
                    node_type,
                    properties,
                    created_at,
                    updated_at,
                };

                // 更新缓存
                self.cache.put_node(node.id.clone(), node.clone()).await;

                Ok(node)
            }
        }
    }

    /// 获取相邻边
    async fn get_adjacent_edges(
        &self,
        node_id: &str,
        edge_types: &Option<Vec<String>>,
    ) -> Result<Vec<GraphEdge>> {
        let mut edges = Vec::new();

        let type_filter = if let Some(types) = edge_types {
            format!(
                " AND edge_type IN ({})",
                types.iter().map(|_| "?").collect::<Vec<_>>().join(", ")
            )
        } else {
            String::new()
        };

        match &self.pool {
            DatabasePool::Sqlite(pool) => {
                let sql = format!(
                    "SELECT id, from_node, to_node, edge_type, weight, properties, created_at, updated_at FROM {}graph_edges WHERE (from_node = ? OR to_node = ?){}",
                    self.config.database.table_prefix, type_filter
                );

                let mut query = sqlx::query(&sql).bind(node_id).bind(node_id);

                if let Some(types) = edge_types {
                    for edge_type in types {
                        query = query.bind(edge_type);
                    }
                }

                let rows = query.fetch_all(pool).await.map_err(MemoryError::Database)?;

                for row in rows {
                    let id: String = row.get("id");
                    let from_node: String = row.get("from_node");
                    let to_node: String = row.get("to_node");
                    let edge_type: String = row.get("edge_type");
                    let weight: f32 = row.get("weight");
                    let properties_str: String = row.get("properties");
                    let created_at_str: String = row.get("created_at");
                    let updated_at_str: String = row.get("updated_at");

                    let properties: HashMap<String, serde_json::Value> =
                        serde_json::from_str(&properties_str)
                            .map_err(MemoryError::Serialization)?;
                    let created_at = DateTime::parse_from_rfc3339(&created_at_str)
                        .map_err(|e| MemoryError::Internal {
                            message: format!("DateTime parse failed: {}", e),
                        })?
                        .with_timezone(&Utc);
                    let updated_at = DateTime::parse_from_rfc3339(&updated_at_str)
                        .map_err(|e| MemoryError::Internal {
                            message: format!("DateTime parse failed: {}", e),
                        })?
                        .with_timezone(&Utc);

                    edges.push(GraphEdge {
                        id,
                        from_node,
                        to_node,
                        edge_type,
                        weight,
                        properties,
                        created_at,
                        updated_at,
                    });
                }
            }
        }

        Ok(edges)
    }

    /// 清除缓存
    pub async fn clear_cache(&self) {
        self.cache.clear().await;
    }

    /// 获取统计信息
    pub async fn get_stats(&self) -> Result<(u64, u64, u64, (usize, usize, usize, usize))> {
        let (vector_count, node_count, edge_count) = match &self.pool {
            DatabasePool::Sqlite(pool) => {
                let prefix = &self.config.database.table_prefix;

                let vector_count: i64 =
                    sqlx::query_scalar(&format!("SELECT COUNT(*) FROM {}vectors", prefix))
                        .fetch_one(pool)
                        .await
                        .map_err(MemoryError::Database)?;

                let node_count: i64 =
                    sqlx::query_scalar(&format!("SELECT COUNT(*) FROM {}graph_nodes", prefix))
                        .fetch_one(pool)
                        .await
                        .map_err(MemoryError::Database)?;

                let edge_count: i64 =
                    sqlx::query_scalar(&format!("SELECT COUNT(*) FROM {}graph_edges", prefix))
                        .fetch_one(pool)
                        .await
                        .map_err(MemoryError::Database)?;

                (vector_count as u64, node_count as u64, edge_count as u64)
            }
        };

        let cache_stats = self.cache.get_stats().await;

        Ok((vector_count, node_count, edge_count, cache_stats))
    }

    // 辅助函数

    /// 计算余弦相似度
    fn cosine_similarity(&self, a: &[f32], b: &[f32]) -> f32 {
        if a.len() != b.len() {
            return 0.0;
        }

        let dot_product: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
        let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
        let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();

        if norm_a == 0.0 || norm_b == 0.0 {
            0.0
        } else {
            dot_product / (norm_a * norm_b)
        }
    }

    /// 向量转字节
    fn vector_to_bytes(&self, vector: &[f32]) -> Result<Vec<u8>> {
        let mut bytes = Vec::with_capacity(vector.len() * 4);
        for &value in vector {
            bytes.extend_from_slice(&value.to_le_bytes());
        }
        Ok(bytes)
    }

    /// 字节转向量
    fn bytes_to_vector(&self, bytes: &[u8]) -> Result<Vec<f32>> {
        if bytes.len() % 4 != 0 {
            return Err(MemoryError::InvalidVectorDimension {
                expected: bytes.len() - (bytes.len() % 4),
                actual: bytes.len(),
            });
        }

        let mut vector = Vec::with_capacity(bytes.len() / 4);
        for chunk in bytes.chunks_exact(4) {
            let value = f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
            vector.push(value);
        }

        Ok(vector)
    }

    /// 检查过滤器匹配
    fn matches_filters(
        &self,
        metadata: &HashMap<String, serde_json::Value>,
        filters: &HashMap<String, serde_json::Value>,
    ) -> bool {
        for (key, expected_value) in filters {
            if let Some(actual_value) = metadata.get(key) {
                if actual_value != expected_value {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    async fn create_test_db() -> VectorGraphDB {
        let temp_file = NamedTempFile::new().unwrap();
        let db_url = format!("sqlite://{}", temp_file.path().display());

        let config = Config {
            database: crate::config::DatabaseConfig {
                url: db_url,
                ..Default::default()
            },
            ..Default::default()
        };

        VectorGraphDB::new(config).await.unwrap()
    }

    #[tokio::test]
    async fn test_vector_operations() {
        let db = create_test_db().await;

        let vector = Vector {
            id: "test_vector".to_string(),
            embedding: vec![0.1, 0.2, 0.3, 0.4],
            metadata: HashMap::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        // 插入向量
        db.insert_vector(&vector).await.unwrap();

        // 查询向量
        let query_request = VectorQueryRequest {
            query_vector: vec![0.1, 0.2, 0.3, 0.4],
            limit: Some(10),
            threshold: Some(0.5),
            filters: None,
        };

        let results = db.query_vectors(&query_request).await.unwrap();
        assert!(!results.is_empty());
        assert_eq!(results[0].vector.id, "test_vector");
    }

    #[tokio::test]
    async fn test_graph_operations() {
        let db = create_test_db().await;

        // 创建节点
        let node1 = GraphNode {
            id: "node1".to_string(),
            node_type: "test".to_string(),
            properties: HashMap::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let node2 = GraphNode {
            id: "node2".to_string(),
            node_type: "test".to_string(),
            properties: HashMap::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        // 插入节点
        db.insert_node(&node1).await.unwrap();
        db.insert_node(&node2).await.unwrap();

        // 创建边
        let edge = GraphEdge {
            id: "edge1".to_string(),
            from_node: "node1".to_string(),
            to_node: "node2".to_string(),
            edge_type: "connects".to_string(),
            weight: 0.8,
            properties: HashMap::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        // 插入边
        db.insert_edge(&edge).await.unwrap();

        // 查询图
        let query_request = GraphQueryRequest {
            start_nodes: vec!["node1".to_string()],
            edge_types: None,
            max_depth: Some(2),
            limit: Some(10),
            filters: None,
        };

        let results = db.query_graph(&query_request).await.unwrap();
        assert!(!results.nodes.is_empty());
        assert!(!results.edges.is_empty());
    }

    #[tokio::test]
    async fn test_cosine_similarity() {
        let db = create_test_db().await;

        let vec1 = vec![1.0, 0.0, 0.0];
        let vec2 = vec![1.0, 0.0, 0.0];
        let vec3 = vec![0.0, 1.0, 0.0];

        assert!((db.cosine_similarity(&vec1, &vec2) - 1.0).abs() < 1e-6);
        assert!((db.cosine_similarity(&vec1, &vec3) - 0.0).abs() < 1e-6);
    }

    #[tokio::test]
    async fn test_vector_serialization() {
        let db = create_test_db().await;

        let original_vector = vec![1.5, -2.3, 0.0, 42.7];
        let bytes = db.vector_to_bytes(&original_vector).unwrap();
        let restored_vector = db.bytes_to_vector(&bytes).unwrap();

        assert_eq!(original_vector, restored_vector);
    }
}

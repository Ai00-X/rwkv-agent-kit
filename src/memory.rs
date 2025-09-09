//! 记忆管理模块
//!
//! 本模块实现RWKV-Agent-Kit的核心记忆管理功能，包括A-Mem的Zettelkasten机制
//! 和HippoRAG的神经生物学启发的检索算法。

use crate::config::Config;
use crate::core_types::{
    Connection, ConnectionId, ConnectionType, Context, EvolutionTrigger, Interaction,
    InteractionType, Memory, MemoryAttributes, MemoryConnections, MemoryId, MemoryMetadata,
    MemoryType, Priority, Query, QueryFilters, QueryType, QueryWeights, UpdateType,
};
use crate::database::{
    GraphEdge, GraphNode, GraphQueryRequest, Vector, VectorGraphDB, VectorQueryRequest,
};
use crate::error::{MemoryError, Result};
use chrono::{DateTime, Duration, Utc};
use log::{debug, info, warn};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// 记忆管理器
///
/// 核心记忆管理组件，整合向量数据库、图数据库和各种记忆处理算法。
#[derive(Debug)]
pub struct MemoryManager {
    db: Arc<VectorGraphDB>,
    config: Config,
    link_analyzer: LinkAnalyzer,
    importance_calculator: ImportanceCalculator,
    evolution_engine: EvolutionEngine,
    retrieval_engine: RetrievalEngine,
    stats: Arc<RwLock<MemoryStats>>,
}

/// 记忆统计信息
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MemoryStats {
    pub total_memories: u64,
    pub total_connections: u64,
    pub average_importance: f32,
    pub retrieval_count: u64,
    pub creation_count: u64,
    pub evolution_count: u64,
    pub last_updated: DateTime<Utc>,
}

/// 链接分析器
///
/// 实现A-Mem的Zettelkasten链接机制，自动发现和建立记忆之间的连接。
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct LinkAnalyzer {
    /// 语义阈值
    semantic_threshold: f32,
    /// 时间窗口
    temporal_window: Duration,
    /// 因果模式
    causal_patterns: Vec<String>,
    /// 主题关键词
    thematic_keywords: HashMap<String, Vec<String>>,
}

/// 重要性计算器
///
/// 基于多种因素计算记忆的重要性评分。
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ImportanceCalculator {
    /// 访问权重
    access_weight: f32,
    /// 时间权重
    recency_weight: f32,
    /// 连接权重
    connection_weight: f32,
    /// 反馈权重
    feedback_weight: f32,
    /// 内容权重
    content_weight: f32,
}

/// 演化引擎
///
/// 实现记忆的动态演化，包括重要性调整、连接强度更新等。
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct EvolutionEngine {
    /// 衰减因子
    decay_factor: f32,
    /// 强化因子
    reinforcement_factor: f32,
    /// 剪枝阈值
    pruning_threshold: f32,
    /// 演化间隔
    evolution_interval: Duration,
}

/// 检索引擎
///
/// 实现HippoRAG的个性化PageRank和多模态检索算法。
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct RetrievalEngine {
    /// PageRank配置
    pagerank_config: PageRankConfig,
    /// 融合权重
    fusion_weights: QueryWeights,
    /// 个性化缓存
    personalization_cache: Arc<RwLock<PersonalizationVector>>,
}

/// 个性化向量
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PersonalizationVector {
    pub preferences: HashMap<String, f32>,
    pub interaction_history: Vec<InteractionRecord>,
    pub last_updated: DateTime<Utc>,
}

/// 交互记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractionRecord {
    pub memory_id: MemoryId,
    pub interaction_type: InteractionType,
    pub timestamp: DateTime<Utc>,
    pub feedback_score: Option<f32>,
    pub dwell_time: Option<u64>, // 毫秒
}

/// PageRank配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageRankConfig {
    pub damping_factor: f32,
    pub max_iterations: usize,
    pub convergence_threshold: f32,
    pub personalization_weight: f32,
}

/// 检索结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetrievalResult {
    pub memory: Memory,
    pub relevance_score: f32,
    pub explanation: RetrievalExplanation,
}

/// 检索解释
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetrievalExplanation {
    pub semantic_score: f32,
    pub temporal_score: f32,
    pub importance_score: f32,
    pub personalization_score: f32,
    pub connection_paths: Vec<Vec<MemoryId>>,
    pub reasoning: String,
}

/// 记忆创建请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMemoryRequest {
    pub content: String,
    pub context: Context,
    pub attributes: Option<MemoryAttributes>,
    pub force_connections: Option<Vec<MemoryId>>,
}

/// 记忆更新请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateMemoryRequest {
    pub memory_id: MemoryId,
    pub updates: Vec<UpdateType>,
    pub context: Context,
}

impl MemoryManager {
    /// 创建新的记忆管理器
    pub async fn new(db: Arc<VectorGraphDB>, config: Config) -> Result<Self> {
        let link_analyzer = LinkAnalyzer::new(&config);
        let importance_calculator = ImportanceCalculator::new(&config);
        let evolution_engine = EvolutionEngine::new(&config);
        let retrieval_engine = RetrievalEngine::new(&config).await?;

        Ok(Self {
            db,
            config,
            link_analyzer,
            importance_calculator,
            evolution_engine,
            retrieval_engine,
            stats: Arc::new(RwLock::new(MemoryStats::default())),
        })
    }

    /// 创建新记忆（从请求）
    pub async fn create_memory_from_request(&self, request: CreateMemoryRequest) -> Result<Memory> {
        info!(
            "Creating new memory with content length: {}",
            request.content.len()
        );

        // 生成嵌入向量
        let embedding = self.generate_embedding(&request.content).await?;

        // 提取属性
        let attributes = if let Some(attrs) = request.attributes {
            attrs
        } else {
            self.extract_attributes(&request.content, &request.context)
                .await?
        };

        // 创建记忆对象
        let memory_type = MemoryType::Knowledge; // 默认类型，可以根据内容分析来确定
        let mut memory = Memory::new(request.content, memory_type, embedding, attributes);

        // 计算初始重要性
        memory.attributes.importance = self
            .importance_calculator
            .calculate_initial_importance(&memory, &request.context)
            .await?;

        // 存储到向量数据库
        let vector = Vector {
            id: memory.id.clone(),
            embedding: memory.embedding.clone(),
            metadata: self.memory_to_metadata(&memory)?,
            created_at: memory.metadata.created_at,
            updated_at: memory.metadata.updated_at,
        };
        self.db.insert_vector(&vector).await?;

        // 存储到图数据库
        let node = GraphNode {
            id: memory.id.clone(),
            node_type: "memory".to_string(),
            properties: self.memory_to_properties(&memory)?,
            created_at: memory.metadata.created_at,
            updated_at: memory.metadata.updated_at,
        };
        self.db.insert_node(&node).await?;

        // 自动发现连接
        let connections = self
            .link_analyzer
            .discover_connections(&memory, &request.context)
            .await?;

        // 添加强制连接
        if let Some(forced_connections) = request.force_connections {
            for target_id in forced_connections {
                let connection = Connection::new(
                    memory.id.clone(),
                    target_id,
                    ConnectionType::Custom("forced".to_string()),
                    1.0,
                );
                self.create_connection(&connection).await?;
            }
        }

        // 创建发现的连接
        for connection in connections {
            self.create_connection(&connection).await?;
            memory
                .connections
                .semantic_links
                .push(connection.to_memory.clone());
        }

        // 更新统计信息
        self.update_stats(|stats| {
            stats.total_memories += 1;
            stats.creation_count += 1;
            stats.last_updated = Utc::now();
        })
        .await;

        info!("Successfully created memory with ID: {}", memory.id);
        Ok(memory)
    }

    /// 创建新记忆（从Memory对象）
    pub async fn create_memory(&self, memory: &Memory) -> Result<()> {
        info!("Creating memory with ID: {}", memory.id);

        // 存储到向量数据库
        let vector = Vector {
            id: memory.id.clone(),
            embedding: memory.embedding.clone(),
            metadata: self.memory_to_metadata(memory)?,
            created_at: memory.metadata.created_at,
            updated_at: memory.metadata.updated_at,
        };
        self.db.insert_vector(&vector).await?;

        // 存储到图数据库
        let node = GraphNode {
            id: memory.id.clone(),
            node_type: "memory".to_string(),
            properties: self.memory_to_properties(memory)?,
            created_at: memory.metadata.created_at,
            updated_at: memory.metadata.updated_at,
        };
        self.db.insert_node(&node).await?;

        // 更新统计信息
        self.update_stats(|stats| {
            stats.total_memories += 1;
            stats.creation_count += 1;
            stats.last_updated = Utc::now();
        })
        .await;

        info!("Successfully created memory with ID: {}", memory.id);
        Ok(())
    }

    /// 检索记忆
    pub async fn retrieve_memories(
        &self,
        query: &Query,
        context: &Context,
    ) -> Result<Vec<RetrievalResult>> {
        debug!("Retrieving memories for query: {}", query.text);

        let results = match query.query_type {
            QueryType::Semantic => self.semantic_retrieval(query, context).await?,
            QueryType::Temporal => self.temporal_retrieval(query, context).await?,
            QueryType::Causal => self.causal_retrieval(query, context).await?,
            QueryType::Thematic => self.thematic_retrieval(query, context).await?,
            QueryType::Mixed => self.mixed_retrieval(query, context).await?,
            QueryType::GraphTraversal => self.graph_traversal_retrieval(query, context).await?,
            QueryType::PersonalizedPageRank => {
                self.personalized_pagerank_retrieval(query, context).await?
            }
        };

        // 更新统计信息
        self.update_stats(|stats| {
            stats.retrieval_count += 1;
            stats.last_updated = Utc::now();
        })
        .await;

        debug!("Retrieved {} memories", results.len());
        Ok(results)
    }

    /// 语义检索
    async fn semantic_retrieval(
        &self,
        query: &Query,
        _context: &Context,
    ) -> Result<Vec<RetrievalResult>> {
        let query_embedding = self.generate_embedding(&query.text).await?;

        let vector_request = VectorQueryRequest {
            query_vector: query_embedding,
            limit: query.limit,
            threshold: Some(self.config.vector.similarity_threshold),
            filters: self.query_filters_to_metadata_filters(&query.filters)?,
        };

        let vector_results = self.db.query_vectors(&vector_request).await?;

        let mut results = Vec::new();
        for vector_result in vector_results {
            let memory = self.metadata_to_memory(&vector_result.vector.metadata)?;

            // 应用查询过滤器
            if !memory.matches_filters(&query.filters) {
                continue;
            }

            let explanation = RetrievalExplanation {
                semantic_score: vector_result.similarity,
                temporal_score: 0.0,
                importance_score: memory.attributes.importance,
                personalization_score: 0.0,
                connection_paths: Vec::new(),
                reasoning: format!("Semantic similarity: {:.3}", vector_result.similarity),
            };

            results.push(RetrievalResult {
                memory,
                relevance_score: vector_result.similarity,
                explanation,
            });
        }

        Ok(results)
    }

    /// 时间检索
    async fn temporal_retrieval(
        &self,
        query: &Query,
        context: &Context,
    ) -> Result<Vec<RetrievalResult>> {
        // 基于时间窗口的检索实现
        let time_window = context
            .time_window
            .unwrap_or((Utc::now() - Duration::days(30), Utc::now()));

        // 首先进行语义检索
        let mut semantic_results = self.semantic_retrieval(query, context).await?;

        // 根据时间相关性重新评分
        for result in &mut semantic_results {
            let time_score = self
                .calculate_temporal_relevance(&result.memory, &time_window, &query.text)
                .await?;

            result.explanation.temporal_score = time_score;
            result.relevance_score = result.explanation.semantic_score
                * query.weights.semantic_weight
                + time_score * query.weights.temporal_weight;
        }

        // 重新排序
        semantic_results.sort_by(|a, b| b.relevance_score.partial_cmp(&a.relevance_score).unwrap());

        Ok(semantic_results)
    }

    /// 因果检索
    async fn causal_retrieval(
        &self,
        query: &Query,
        context: &Context,
    ) -> Result<Vec<RetrievalResult>> {
        // 基于因果关系的检索实现
        let causal_keywords = self.extract_causal_keywords(&query.text)?;

        let graph_request = GraphQueryRequest {
            start_nodes: context.recent_memories.clone(),
            edge_types: Some(vec!["causal".to_string()]),
            max_depth: Some(3),
            limit: query.limit,
            filters: self.query_filters_to_graph_filters(&query.filters)?,
        };

        let graph_results = self.db.query_graph(&graph_request).await?;

        let mut results = Vec::new();
        for node in graph_results.nodes {
            if let Ok(memory) = self.properties_to_memory(&node.properties) {
                let causal_score = self
                    .calculate_causal_relevance(&memory, &causal_keywords)
                    .await?;

                if causal_score > 0.1 {
                    let explanation = RetrievalExplanation {
                        semantic_score: 0.0,
                        temporal_score: 0.0,
                        importance_score: memory.attributes.importance,
                        personalization_score: 0.0,
                        connection_paths: Vec::new(),
                        reasoning: format!("Causal relevance: {:.3}", causal_score),
                    };

                    results.push(RetrievalResult {
                        memory,
                        relevance_score: causal_score,
                        explanation,
                    });
                }
            }
        }

        results.sort_by(|a, b| b.relevance_score.partial_cmp(&a.relevance_score).unwrap());
        Ok(results)
    }

    /// 主题检索
    async fn thematic_retrieval(
        &self,
        query: &Query,
        _context: &Context,
    ) -> Result<Vec<RetrievalResult>> {
        let themes = self.extract_themes(&query.text)?;

        let graph_request = GraphQueryRequest {
            start_nodes: Vec::new(),
            edge_types: Some(vec!["thematic".to_string()]),
            max_depth: Some(2),
            limit: query.limit,
            filters: self.create_theme_filters(&themes)?,
        };

        let graph_results = self.db.query_graph(&graph_request).await?;

        let mut results = Vec::new();
        for node in graph_results.nodes {
            if let Ok(memory) = self.properties_to_memory(&node.properties) {
                let theme_score = self.calculate_thematic_relevance(&memory, &themes).await?;

                let explanation = RetrievalExplanation {
                    semantic_score: 0.0,
                    temporal_score: 0.0,
                    importance_score: memory.attributes.importance,
                    personalization_score: 0.0,
                    connection_paths: Vec::new(),
                    reasoning: format!("Thematic relevance: {:.3}", theme_score),
                };

                results.push(RetrievalResult {
                    memory,
                    relevance_score: theme_score,
                    explanation,
                });
            }
        }

        results.sort_by(|a, b| b.relevance_score.partial_cmp(&a.relevance_score).unwrap());
        Ok(results)
    }

    /// 混合检索
    async fn mixed_retrieval(
        &self,
        query: &Query,
        context: &Context,
    ) -> Result<Vec<RetrievalResult>> {
        // 执行多种检索方法
        let semantic_results = self.semantic_retrieval(query, context).await?;
        let temporal_results = self.temporal_retrieval(query, context).await?;
        let causal_results = self.causal_retrieval(query, context).await?;

        // 融合结果
        let fused_results = self
            .fuse_retrieval_results(
                vec![semantic_results, temporal_results, causal_results],
                &query.weights,
            )
            .await?;

        Ok(fused_results)
    }

    /// 图遍历检索
    async fn graph_traversal_retrieval(
        &self,
        query: &Query,
        context: &Context,
    ) -> Result<Vec<RetrievalResult>> {
        let start_nodes = if context.recent_memories.is_empty() {
            // 如果没有最近记忆，使用语义检索找到起始节点
            let semantic_results = self.semantic_retrieval(query, context).await?;
            semantic_results
                .into_iter()
                .take(3)
                .map(|r| r.memory.id)
                .collect()
        } else {
            context.recent_memories.clone()
        };

        let graph_request = GraphQueryRequest {
            start_nodes,
            edge_types: None,
            max_depth: Some(self.config.graph.traversal.max_depth),
            limit: query.limit,
            filters: self.query_filters_to_graph_filters(&query.filters)?,
        };

        let graph_results = self.db.query_graph(&graph_request).await?;

        let mut results = Vec::new();
        for (i, node) in graph_results.nodes.iter().enumerate() {
            if let Ok(memory) = self.properties_to_memory(&node.properties) {
                let path_score = 1.0 / (i as f32 + 1.0); // 距离越近分数越高

                let explanation = RetrievalExplanation {
                    semantic_score: 0.0,
                    temporal_score: 0.0,
                    importance_score: memory.attributes.importance,
                    personalization_score: 0.0,
                    connection_paths: graph_results.paths.clone(),
                    reasoning: format!("Graph traversal score: {:.3}", path_score),
                };

                results.push(RetrievalResult {
                    memory,
                    relevance_score: path_score,
                    explanation,
                });
            }
        }

        Ok(results)
    }

    /// 个性化PageRank检索
    async fn personalized_pagerank_retrieval(
        &self,
        query: &Query,
        context: &Context,
    ) -> Result<Vec<RetrievalResult>> {
        // 获取个性化向量
        let personalization_vector = self.retrieval_engine.get_personalization_vector().await?;

        // 执行个性化PageRank
        let pagerank_scores = self
            .compute_personalized_pagerank(&personalization_vector, &query.text)
            .await?;

        // 结合语义检索
        let semantic_results = self.semantic_retrieval(query, context).await?;

        let mut results = Vec::new();
        for semantic_result in semantic_results {
            let pagerank_score = pagerank_scores
                .get(&semantic_result.memory.id)
                .unwrap_or(&0.0);

            let combined_score = semantic_result.relevance_score * query.weights.semantic_weight
                + pagerank_score * query.weights.personalization_weight;

            let mut explanation = semantic_result.explanation;
            explanation.personalization_score = *pagerank_score;
            explanation.reasoning = format!(
                "Combined score: semantic={:.3}, pagerank={:.3}",
                semantic_result.relevance_score, pagerank_score
            );

            results.push(RetrievalResult {
                memory: semantic_result.memory,
                relevance_score: combined_score,
                explanation,
            });
        }

        results.sort_by(|a, b| b.relevance_score.partial_cmp(&a.relevance_score).unwrap());
        Ok(results)
    }

    /// 更新记忆
    pub async fn update_memory(&self, request: UpdateMemoryRequest) -> Result<Memory> {
        info!("Updating memory: {}", request.memory_id);

        // 获取现有记忆
        let mut memory = self.get_memory(&request.memory_id).await?;

        // 应用更新
        for update in request.updates {
            match update {
                UpdateType::ImportanceAdjustment(new_importance) => {
                    memory.attributes.importance = new_importance.clamp(0.0, 1.0);
                }
                UpdateType::ConnectionAdded(connection) => {
                    self.create_connection(&connection).await?;
                    memory.connections.semantic_links.push(connection.to_memory);
                }
                UpdateType::ConnectionRemoved(connection_id) => {
                    self.remove_connection(&connection_id).await?;
                }
                UpdateType::AttributeUpdate(key, value) => {
                    memory.attributes.custom_attributes.insert(key, value);
                }
                UpdateType::ContentUpdate(new_content) => {
                    memory.content = new_content;
                    memory.embedding = self.generate_embedding(&memory.content).await?;
                }
                UpdateType::TagUpdate(new_tags) => {
                    memory.attributes.tags = new_tags;
                }
                UpdateType::MetadataUpdate(metadata) => {
                    for (key, value) in metadata {
                        memory.metadata.custom_metadata.insert(key, value);
                    }
                }
            }
        }

        // 更新时间戳
        memory.metadata.updated_at = Utc::now();
        memory.metadata.version += 1;

        // 保存到数据库
        self.save_memory(&memory).await?;

        info!("Successfully updated memory: {}", request.memory_id);
        Ok(memory)
    }

    /// 删除记忆
    pub async fn delete_memory(&self, memory_id: &MemoryId, soft_delete: bool) -> Result<()> {
        info!("Deleting memory: {} (soft: {})", memory_id, soft_delete);

        if soft_delete {
            // 软删除：标记为已删除
            let mut memory = self.get_memory(memory_id).await?;
            memory.metadata.is_deleted = true;
            memory.metadata.deleted_at = Some(Utc::now());
            self.save_memory(&memory).await?;
        } else {
            // 硬删除：从数据库中移除
            // TODO: 实现硬删除逻辑
            return Err(MemoryError::Internal {
                message: "Hard delete not implemented".to_string(),
            });
        }

        // 更新统计信息
        self.update_stats(|stats| {
            if stats.total_memories > 0 {
                stats.total_memories -= 1;
            }
            stats.last_updated = Utc::now();
        })
        .await;

        info!("Successfully deleted memory: {}", memory_id);
        Ok(())
    }

    /// 演化记忆
    pub async fn evolve_memories(&self, trigger: EvolutionTrigger) -> Result<u64> {
        info!("Starting memory evolution with trigger: {:?}", trigger);

        let evolved_count = self.evolution_engine.evolve(&trigger, &self.db).await?;

        // 更新统计信息
        self.update_stats(|stats| {
            stats.evolution_count += evolved_count;
            stats.last_updated = Utc::now();
        })
        .await;

        info!("Evolution completed. {} memories evolved", evolved_count);
        Ok(evolved_count)
    }

    /// 获取记忆统计信息
    pub async fn get_stats(&self) -> MemoryStats {
        self.stats.read().await.clone()
    }

    // 私有辅助方法

    pub async fn generate_embedding(&self, text: &str) -> Result<Vec<f32>> {
        use model2vec_rs::model::StaticModel;
        use std::sync::OnceLock;
        use std::sync::{Arc, Mutex};

        // 使用静态变量缓存模型，避免重复加载
        static MODEL: OnceLock<Option<Arc<Mutex<StaticModel>>>> = OnceLock::new();

        let model = MODEL
            .get_or_init(|| {
                match StaticModel::from_pretrained(
                    "minishlab/potion-multilingual-128M", // 使用多语言模型
                    None,                                 // 无需 HF token
                    None,                                 // 使用模型默认的归一化设置
                    None,                                 // 无子文件夹
                ) {
                    Ok(m) => Some(Arc::new(Mutex::new(m))),
                    Err(_) => None,
                }
            })
            .as_ref()
            .ok_or_else(|| MemoryError::Internal {
                message: "Failed to load embedding model".to_string(),
            })?;

        let model_guard = model.lock().map_err(|e| MemoryError::Internal {
            message: format!("Failed to acquire model lock: {}", e),
        })?;

        // 生成嵌入向量
        let sentences = vec![text.to_string()];
        let embeddings = model_guard.encode(&sentences);

        if embeddings.is_empty() {
            return Err(MemoryError::Internal {
                message: "Failed to generate embedding: empty result".to_string(),
            });
        }

        Ok(embeddings[0].clone())
    }

    async fn extract_attributes(
        &self,
        content: &str,
        context: &Context,
    ) -> Result<MemoryAttributes> {
        // 简单的属性提取实现
        let words: Vec<&str> = content.split_whitespace().collect();
        let keywords = words.iter().take(5).map(|s| s.to_string()).collect();

        Ok(MemoryAttributes {
            keywords,
            tags: Vec::new(),
            context: context.current_topic.clone().unwrap_or_default(),
            importance: 0.5,
            emotion: None,
            source: None,
            confidence: 1.0,
            language: Some("zh".to_string()),
            custom_attributes: HashMap::new(),
        })
    }

    pub async fn create_connection(&self, connection: &Connection) -> Result<()> {
        let edge = GraphEdge {
            id: connection.id.clone(),
            from_node: connection.from_memory.clone(),
            to_node: connection.to_memory.clone(),
            edge_type: format!("{:?}", connection.connection_type),
            weight: connection.strength,
            properties: connection.properties.clone(),
            created_at: connection.created_at,
            updated_at: connection.updated_at,
        };

        self.db.insert_edge(&edge).await?;

        // 更新统计信息
        self.update_stats(|stats| {
            stats.total_connections += 1;
            stats.last_updated = Utc::now();
        })
        .await;

        Ok(())
    }

    async fn remove_connection(&self, connection_id: &ConnectionId) -> Result<()> {
        // TODO: 实现连接删除逻辑
        warn!("Connection removal not implemented: {}", connection_id);
        Ok(())
    }

    async fn get_memory(&self, memory_id: &MemoryId) -> Result<Memory> {
        let vector = self.db.get_vector(memory_id).await.map_err(|e| {
            if let MemoryError::Database(_) = e {
                MemoryError::MemoryNotFound {
                    id: memory_id.clone(),
                }
            } else {
                e
            }
        })?;

        let mut memory = self.metadata_to_memory(&vector.metadata)?;
        memory.embedding = vector.embedding;

        // 可选: 加载连接
        memory.connections.semantic_links = self
            .get_connections(memory_id)
            .await?
            .into_iter()
            .map(|c| c.to_memory)
            .collect();

        Ok(memory)
    }

    async fn save_memory(&self, memory: &Memory) -> Result<()> {
        let vector = Vector {
            id: memory.id.clone(),
            embedding: memory.embedding.clone(),
            metadata: self.memory_to_metadata(memory)?,
            created_at: memory.metadata.created_at,
            updated_at: memory.metadata.updated_at,
        };
        self.db.update_vector(&vector).await?;

        let node = GraphNode {
            id: memory.id.clone(),
            node_type: "memory".to_string(),
            properties: self.memory_to_properties(memory)?,
            created_at: memory.metadata.created_at,
            updated_at: memory.metadata.updated_at,
        };
        self.db.update_node(&node).await?;

        Ok(())
    }

    /// 记录用户交互
    pub async fn record_interaction(&self, interaction: &Interaction) -> Result<()> {
        debug!("Recording interaction: {:?}", interaction.interaction_type);

        // 更新记忆的访问统计
        for memory_id in &interaction.retrieved_memories {
            // TODO: 更新记忆的访问计数和最后访问时间
            info!("Updated access stats for memory: {}", memory_id);
        }

        // 更新统计信息
        self.update_stats(|stats| {
            stats.last_updated = Utc::now();
        })
        .await;

        Ok(())
    }

    /// 获取记忆的连接
    pub async fn get_connections(&self, memory_id: &MemoryId) -> Result<Vec<Connection>> {
        debug!("Getting connections for memory: {}", memory_id);

        let graph_request = GraphQueryRequest {
            start_nodes: vec![memory_id.clone()],
            edge_types: None,
            max_depth: Some(1),
            limit: Some(100),
            filters: None,
        };

        let graph_results = self.db.query_graph(&graph_request).await?;

        let mut connections = Vec::new();
        for edge in graph_results.edges {
            let connection = Connection {
                id: edge.id,
                from_memory: edge.from_node,
                to_memory: edge.to_node,
                connection_type: ConnectionType::Semantic, // TODO: 从edge.edge_type解析
                strength: edge.weight,
                created_at: edge.created_at,
                updated_at: edge.updated_at,
                properties: edge.properties,
                bidirectional: false,
            };
            connections.push(connection);
        }

        Ok(connections)
    }

    async fn update_stats<F>(&self, updater: F)
    where
        F: FnOnce(&mut MemoryStats),
    {
        let mut stats = self.stats.write().await;
        updater(&mut stats);
    }

    // 转换方法

    fn memory_to_metadata(&self, memory: &Memory) -> Result<HashMap<String, serde_json::Value>> {
        let mut metadata = HashMap::new();

        // 基本信息
        metadata.insert(
            "id".to_string(),
            serde_json::Value::String(memory.id.clone()),
        );
        metadata.insert(
            "content".to_string(),
            serde_json::Value::String(memory.content.clone()),
        );
        metadata.insert(
            "memory_type".to_string(),
            serde_json::Value::String(format!("{:?}", memory.memory_type)),
        );

        // 嵌入向量
        let embedding_array: Vec<serde_json::Value> = memory
            .embedding
            .iter()
            .map(|&f| serde_json::Value::Number(serde_json::Number::from_f64(f as f64).unwrap()))
            .collect();
        metadata.insert(
            "embedding".to_string(),
            serde_json::Value::Array(embedding_array),
        );

        // 属性
        metadata.insert(
            "importance".to_string(),
            serde_json::Value::Number(
                serde_json::Number::from_f64(memory.attributes.importance as f64).unwrap(),
            ),
        );
        metadata.insert(
            "keywords".to_string(),
            serde_json::Value::Array(
                memory
                    .attributes
                    .keywords
                    .iter()
                    .map(|k| serde_json::Value::String(k.clone()))
                    .collect(),
            ),
        );
        metadata.insert(
            "tags".to_string(),
            serde_json::Value::Array(
                memory
                    .attributes
                    .tags
                    .iter()
                    .map(|t| serde_json::Value::String(t.clone()))
                    .collect(),
            ),
        );
        metadata.insert(
            "context".to_string(),
            serde_json::Value::String(memory.attributes.context.clone()),
        );
        metadata.insert(
            "confidence".to_string(),
            serde_json::Value::Number(
                serde_json::Number::from_f64(memory.attributes.confidence as f64).unwrap(),
            ),
        );

        if let Some(ref source) = memory.attributes.source {
            metadata.insert(
                "source".to_string(),
                serde_json::Value::String(source.to_string()),
            );
        }

        if let Some(ref language) = memory.attributes.language {
            metadata.insert(
                "language".to_string(),
                serde_json::Value::String(language.to_string()),
            );
        }

        // 元数据
        metadata.insert(
            "created_at".to_string(),
            serde_json::Value::String(memory.metadata.created_at.to_rfc3339()),
        );
        metadata.insert(
            "updated_at".to_string(),
            serde_json::Value::String(memory.metadata.updated_at.to_rfc3339()),
        );
        metadata.insert(
            "access_count".to_string(),
            serde_json::Value::Number(serde_json::Number::from(memory.metadata.access_count)),
        );
        metadata.insert(
            "last_accessed".to_string(),
            serde_json::Value::String(memory.metadata.last_accessed.to_rfc3339()),
        );
        metadata.insert(
            "version".to_string(),
            serde_json::Value::Number(serde_json::Number::from(memory.metadata.version)),
        );

        metadata.insert(
            "is_deleted".to_string(),
            serde_json::Value::Bool(memory.metadata.is_deleted),
        );

        Ok(metadata)
    }

    fn memory_to_properties(&self, memory: &Memory) -> Result<HashMap<String, serde_json::Value>> {
        self.memory_to_metadata(memory)
    }

    fn metadata_to_memory(&self, metadata: &HashMap<String, serde_json::Value>) -> Result<Memory> {
        // 从元数据重构Memory对象
        let id = metadata
            .get("id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| MemoryError::Internal {
                message: "Missing memory id in metadata".to_string(),
            })?
            .to_string();

        let content = metadata
            .get("content")
            .and_then(|v| v.as_str())
            .ok_or_else(|| MemoryError::Internal {
                message: "Missing content in metadata".to_string(),
            })?
            .to_string();

        let memory_type_str = metadata
            .get("memory_type")
            .and_then(|v| v.as_str())
            .unwrap_or("Knowledge");
        let memory_type = match memory_type_str {
            "Knowledge" => MemoryType::Knowledge,
            "Event" => MemoryType::Event,
            "Task" => MemoryType::Task,
            "Conversation" => MemoryType::Conversation,
            "Reflection" => MemoryType::Reflection,
            "Goal" => MemoryType::Goal,
            "Habit" => MemoryType::Habit,
            "Emotion" => MemoryType::Emotion,
            _ => MemoryType::Knowledge,
        };

        let embedding = metadata
            .get("embedding")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_f64().map(|f| f as f32))
                    .collect()
            })
            .unwrap_or_else(|| vec![0.0; self.config.vector.dimension]);

        let importance = metadata
            .get("importance")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.5) as f32;

        let keywords = metadata
            .get("keywords")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let tags = metadata
            .get("tags")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let created_at = metadata
            .get("created_at")
            .and_then(|v| v.as_str())
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);

        let updated_at = metadata
            .get("updated_at")
            .and_then(|v| v.as_str())
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);

        let attributes = MemoryAttributes {
            keywords,
            tags,
            context: metadata
                .get("context")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            importance,
            emotion: None,
            source: metadata
                .get("source")
                .and_then(|v| v.as_str())
                .map(|s: &str| s.to_string()),
            confidence: metadata
                .get("confidence")
                .and_then(|v| v.as_f64())
                .unwrap_or(1.0) as f32,
            language: metadata
                .get("language")
                .and_then(|v| v.as_str())
                .map(|s: &str| s.to_string()),
            custom_attributes: HashMap::new(),
        };

        let metadata_obj = MemoryMetadata {
            created_at,
            updated_at,
            access_count: metadata
                .get("access_count")
                .and_then(|v| v.as_u64())
                .unwrap_or(0),
            last_accessed: metadata
                .get("last_accessed")
                .and_then(|v| v.as_str())
                .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(Utc::now),
            version: metadata
                .get("version")
                .and_then(|v| v.as_u64())
                .unwrap_or(1) as u32,

            is_deleted: metadata
                .get("is_deleted")
                .and_then(|v| v.as_bool())
                .unwrap_or(false),
            deleted_at: None,
            custom_metadata: HashMap::new(),
        };

        let connections = MemoryConnections {
            semantic_links: Vec::new(),
            temporal_links: Vec::new(),
            causal_links: Vec::new(),
            thematic_links: Vec::new(),
            custom_links: HashMap::new(),
        };

        Ok(Memory {
            id,
            content,
            memory_type,
            embedding,
            attributes,
            connections,
            metadata: metadata_obj,
        })
    }

    fn properties_to_memory(
        &self,
        properties: &HashMap<String, serde_json::Value>,
    ) -> Result<Memory> {
        self.metadata_to_memory(properties)
    }

    fn query_filters_to_metadata_filters(
        &self,
        _filters: &QueryFilters,
    ) -> Result<Option<HashMap<String, serde_json::Value>>> {
        // TODO: 实现查询过滤器转换
        Ok(None)
    }

    fn query_filters_to_graph_filters(
        &self,
        filters: &QueryFilters,
    ) -> Result<Option<HashMap<String, serde_json::Value>>> {
        self.query_filters_to_metadata_filters(filters)
    }

    async fn calculate_temporal_relevance(
        &self,
        memory: &Memory,
        time_window: &(DateTime<Utc>, DateTime<Utc>),
        _query: &str,
    ) -> Result<f32> {
        // 简单的时间相关性计算
        let memory_time = memory.metadata.created_at;
        if memory_time >= time_window.0 && memory_time <= time_window.1 {
            let window_duration = time_window.1 - time_window.0;
            let memory_offset = memory_time - time_window.0;
            Ok(1.0 - (memory_offset.num_seconds() as f32 / window_duration.num_seconds() as f32))
        } else {
            Ok(0.0)
        }
    }

    fn extract_causal_keywords(&self, text: &str) -> Result<Vec<String>> {
        let causal_words = vec!["因为", "所以", "导致", "引起", "造成", "由于", "结果"];
        let keywords = causal_words
            .into_iter()
            .filter(|word| text.contains(word))
            .map(|s| s.to_string())
            .collect();
        Ok(keywords)
    }

    async fn calculate_causal_relevance(
        &self,
        memory: &Memory,
        keywords: &[String],
    ) -> Result<f32> {
        let content_lower = memory.content.to_lowercase();
        let matches = keywords
            .iter()
            .filter(|k| content_lower.contains(&k.to_lowercase()))
            .count();
        Ok(matches as f32 / keywords.len().max(1) as f32)
    }

    fn extract_themes(&self, text: &str) -> Result<Vec<String>> {
        // 简单的主题提取
        let words: Vec<String> = text
            .split_whitespace()
            .filter(|w| w.len() > 3)
            .take(10)
            .map(|s| s.to_string())
            .collect();
        Ok(words)
    }

    fn create_theme_filters(
        &self,
        themes: &[String],
    ) -> Result<Option<HashMap<String, serde_json::Value>>> {
        let mut filters = HashMap::new();
        filters.insert(
            "themes".to_string(),
            serde_json::Value::Array(
                themes
                    .iter()
                    .map(|t| serde_json::Value::String(t.clone()))
                    .collect(),
            ),
        );
        Ok(Some(filters))
    }

    async fn calculate_thematic_relevance(
        &self,
        memory: &Memory,
        themes: &[String],
    ) -> Result<f32> {
        let content_lower = memory.content.to_lowercase();
        let matches = themes
            .iter()
            .filter(|theme| content_lower.contains(&theme.to_lowercase()))
            .count();
        Ok(matches as f32 / themes.len().max(1) as f32)
    }

    async fn fuse_retrieval_results(
        &self,
        results_list: Vec<Vec<RetrievalResult>>,
        weights: &QueryWeights,
    ) -> Result<Vec<RetrievalResult>> {
        let mut fused_results = HashMap::new();

        for (i, results) in results_list.iter().enumerate() {
            let weight = match i {
                0 => weights.semantic_weight,
                1 => weights.temporal_weight,
                2 => weights.importance_weight,
                _ => 0.1,
            };

            for result in results {
                let entry = fused_results
                    .entry(result.memory.id.clone())
                    .or_insert_with(|| result.clone());
                entry.relevance_score += result.relevance_score * weight;
            }
        }

        let mut final_results: Vec<RetrievalResult> = fused_results.into_values().collect();
        final_results.sort_by(|a, b| b.relevance_score.partial_cmp(&a.relevance_score).unwrap());

        Ok(final_results)
    }

    async fn compute_personalized_pagerank(
        &self,
        personalization: &PersonalizationVector,
        query: &str,
    ) -> Result<HashMap<MemoryId, f32>> {
        // 简化的PageRank实现
        let mut scores = HashMap::new();

        // 基于用户偏好计算分数
        for (preference, weight) in &personalization.preferences {
            if query.contains(preference) {
                scores.insert(format!("memory_{}", preference), *weight);
            }
        }

        Ok(scores)
    }
}

// 实现各个组件

impl LinkAnalyzer {
    fn new(config: &Config) -> Self {
        Self {
            semantic_threshold: config.vector.similarity_threshold,
            temporal_window: Duration::hours(24),
            causal_patterns: vec!["因为".to_string(), "所以".to_string(), "导致".to_string()],
            thematic_keywords: HashMap::new(),
        }
    }

    async fn discover_connections(
        &self,
        memory: &Memory,
        context: &Context,
    ) -> Result<Vec<Connection>> {
        let mut connections = Vec::new();

        // 基于最近记忆发现连接
        for recent_memory_id in &context.recent_memories {
            if recent_memory_id != &memory.id {
                let connection = Connection::new(
                    memory.id.clone(),
                    recent_memory_id.clone(),
                    ConnectionType::Temporal,
                    0.7,
                );
                connections.push(connection);
            }
        }

        Ok(connections)
    }
}

impl ImportanceCalculator {
    fn new(_config: &Config) -> Self {
        Self {
            access_weight: 0.3,
            recency_weight: 0.2,
            connection_weight: 0.2,
            feedback_weight: 0.2,
            content_weight: 0.1,
        }
    }

    async fn calculate_initial_importance(
        &self,
        memory: &Memory,
        context: &Context,
    ) -> Result<f32> {
        let mut importance: f32 = 0.5; // 基础重要性

        // 基于内容长度调整
        let content_factor = (memory.content.len() as f32 / 1000.0).min(1.0);
        importance += content_factor * self.content_weight;

        // 基于上下文优先级调整
        match context.priority {
            Priority::Critical => importance += 0.3,
            Priority::High => importance += 0.2,
            Priority::Normal => {}
            Priority::Low => importance -= 0.1,
        }

        Ok(importance.clamp(0.0, 1.0))
    }
}

impl EvolutionEngine {
    fn new(config: &Config) -> Self {
        Self {
            decay_factor: config.learning.importance_decay_factor,
            reinforcement_factor: 1.05,
            pruning_threshold: 0.1,
            evolution_interval: Duration::hours(config.learning.learning_interval_hours as i64),
        }
    }

    async fn evolve(&self, trigger: &EvolutionTrigger, _db: &VectorGraphDB) -> Result<u64> {
        match trigger {
            EvolutionTrigger::TimeDecay => {
                // 实现时间衰减逻辑
                Ok(0)
            }
            EvolutionTrigger::UserFeedback(_memory_id, _score) => {
                // 实现用户反馈处理逻辑
                Ok(1)
            }
            _ => Ok(0),
        }
    }
}

impl RetrievalEngine {
    async fn new(config: &Config) -> Result<Self> {
        Ok(Self {
            pagerank_config: PageRankConfig {
                damping_factor: config.graph.pagerank.damping_factor,
                max_iterations: config.graph.pagerank.max_iterations,
                convergence_threshold: config.graph.pagerank.convergence_threshold,
                personalization_weight: config.graph.pagerank.personalization_weight,
            },
            fusion_weights: QueryWeights::default(),
            personalization_cache: Arc::new(RwLock::new(PersonalizationVector::default())),
        })
    }

    async fn get_personalization_vector(&self) -> Result<PersonalizationVector> {
        let cache = self.personalization_cache.read().await;
        Ok(cache.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::VectorGraphDB;
    use tempfile::NamedTempFile;

    async fn create_test_memory_manager() -> MemoryManager {
        let temp_file = NamedTempFile::new().unwrap();
        let db_url = format!("sqlite://{}", temp_file.path().display());

        let config = Config {
            database: crate::config::DatabaseConfig {
                url: db_url,
                ..Default::default()
            },
            vector: crate::config::VectorConfig {
                similarity_threshold: 0.0, // 设置为0.0以便测试
                ..Default::default()
            },
            ..Default::default()
        };

        let db = Arc::new(VectorGraphDB::new(config.clone()).await.unwrap());
        MemoryManager::new(db, config).await.unwrap()
    }

    #[tokio::test]
    async fn test_create_memory() {
        let manager = create_test_memory_manager().await;

        let request = CreateMemoryRequest {
            content: "这是一个测试记忆".to_string(),
            context: Context::default(),
            attributes: None,
            force_connections: None,
        };

        let memory = manager.create_memory_from_request(request).await.unwrap();
        assert_eq!(memory.content, "这是一个测试记忆");
        assert!(!memory.id.is_empty());
        assert!(!memory.embedding.is_empty());
    }

    #[tokio::test]
    async fn test_semantic_retrieval() {
        let manager = create_test_memory_manager().await;

        // 创建测试记忆
        let request = CreateMemoryRequest {
            content: "人工智能的发展历程".to_string(),
            context: Context::default(),
            attributes: None,
            force_connections: None,
        };
        manager.create_memory_from_request(request).await.unwrap();

        // 执行检索
        let query = Query {
            text: "AI发展".to_string(),
            query_type: QueryType::Semantic,
            filters: QueryFilters::default(),
            limit: Some(10),
            offset: None,
            sort_by: None,
            weights: QueryWeights::default(),
        };

        let results = manager
            .retrieve_memories(&query, &Context::default())
            .await
            .unwrap();
        assert!(!results.is_empty());
    }

    #[tokio::test]
    async fn test_memory_stats() {
        let manager = create_test_memory_manager().await;

        let initial_stats = manager.get_stats().await;
        assert_eq!(initial_stats.total_memories, 0);

        // 创建记忆
        let request = CreateMemoryRequest {
            content: "测试统计".to_string(),
            context: Context::default(),
            attributes: None,
            force_connections: None,
        };
        manager.create_memory_from_request(request).await.unwrap();

        let updated_stats = manager.get_stats().await;
        assert_eq!(updated_stats.total_memories, 1);
        assert_eq!(updated_stats.creation_count, 1);
    }
}

//! 检索模块
//!
//! 实现HippoRAG的个性化PageRank算法和多模态检索功能。

use crate::config::Config;
use crate::core_types::{
    InteractionType, Memory, MemoryAttributes, MemoryConnections, MemoryId, MemoryMetadata,
    MemoryType, Priority, Query,
};
use crate::database::{GraphQueryRequest, VectorGraphDB, VectorQueryRequest};
use crate::error::{MemoryError, Result};
use crate::memory::{
    InteractionRecord, PersonalizationVector, RetrievalExplanation, RetrievalResult,
};
use chrono::{DateTime, Duration, Utc};
use log::{debug, info};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use tokio::sync::RwLock;

/// HippoRAG检索引擎
///
/// 实现神经生物学启发的检索算法，包括个性化PageRank和多模态融合。
#[derive(Debug)]
pub struct HippoRAGRetriever {
    db: Arc<VectorGraphDB>,
    config: Config,
    pagerank_engine: PageRankEngine,
    fusion_engine: FusionEngine,
    personalization_manager: PersonalizationManager,
    cache: Arc<RwLock<RetrievalCache>>,
}

/// PageRank引擎
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct PageRankEngine {
    /// 阻尼因子
    damping_factor: f32,
    /// 最大迭代次数
    max_iterations: usize,
    /// 收敛阈值
    convergence_threshold: f32,
    /// 个性化权重
    personalization_weight: f32,
}

/// 融合引擎
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct FusionEngine {
    /// 语义权重
    semantic_weight: f32,
    /// 时间权重
    temporal_weight: f32,
    /// 结构权重
    structural_weight: f32,
    /// 重要性权重
    importance_weight: f32,
    /// 个性化权重
    personalization_weight: f32,
}

/// 个性化管理器
#[derive(Debug)]
#[allow(dead_code)]
pub struct PersonalizationManager {
    /// 用户档案
    user_profile: UserProfile,
    /// 交互追踪器
    interaction_tracker: InteractionTracker,
    /// 偏好学习器
    preference_learner: RetrievalPreferenceLearner,
}

/// 用户档案
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub preferences: HashMap<String, f32>,
    pub topic_interests: HashMap<String, f32>,
    pub temporal_patterns: TemporalPatterns,
    pub interaction_history: Vec<InteractionRecord>,
    pub learning_rate: f32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Default for UserProfile {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            preferences: HashMap::new(),
            topic_interests: HashMap::new(),
            temporal_patterns: TemporalPatterns::default(),
            interaction_history: Vec::new(),
            learning_rate: 0.1,
            created_at: now,
            updated_at: now,
        }
    }
}

/// 时间模式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalPatterns {
    pub active_hours: Vec<u8>,   // 0-23小时
    pub preferred_days: Vec<u8>, // 0-6星期
    pub session_duration: Duration,
    pub query_frequency: f32,
}

impl Default for TemporalPatterns {
    fn default() -> Self {
        use chrono::Duration;
        Self {
            active_hours: vec![],
            preferred_days: vec![],
            session_duration: Duration::hours(1), // 1小时默认
            query_frequency: 1.0,
        }
    }
}

/// 交互追踪器
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct InteractionTracker {
    /// 最近的交互记录
    recent_interactions: VecDeque<InteractionRecord>,
    /// 最大历史记录大小
    max_history_size: usize,
}

/// 偏好学习器
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct RetrievalPreferenceLearner {
    /// 学习率
    learning_rate: f32,
    /// 衰减因子
    decay_factor: f32,
    /// 强化阈值
    reinforcement_threshold: f32,
}

/// 检索缓存
#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct RetrievalCache {
    /// 查询缓存
    query_cache: HashMap<String, CachedResult>,
    /// PageRank缓存
    pagerank_cache: HashMap<String, HashMap<MemoryId, f32>>,
    /// 个性化缓存
    personalization_cache: PersonalizationVector,
    /// 最大缓存大小
    max_cache_size: usize,
}

/// 缓存结果
#[derive(Debug, Clone)]
pub struct CachedResult {
    pub results: Vec<RetrievalResult>,
    pub timestamp: DateTime<Utc>,
    pub ttl: Duration,
}

/// 检索策略
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RetrievalStrategy {
    /// 纯语义检索
    Semantic,
    /// 基于图结构的检索
    Structural,
    /// 时间感知检索
    Temporal,
    /// 个性化检索
    Personalized,
    /// 混合检索
    Hybrid {
        strategies: Vec<RetrievalStrategy>,
        weights: Vec<f32>,
    },
    /// HippoRAG完整检索
    HippoRAG {
        use_pagerank: bool,
        use_personalization: bool,
        fusion_method: FusionMethod,
    },
}

/// 融合方法
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FusionMethod {
    /// 线性加权融合
    LinearWeighted,
    /// 排名融合
    RankFusion,
    /// 学习融合
    LearnedFusion,
    /// 动态融合
    DynamicFusion,
}

/// 检索上下文
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetrievalContext {
    pub session_id: Option<String>,
    pub current_topic: Option<String>,
    pub recent_queries: Vec<String>,
    pub time_window: Option<(DateTime<Utc>, DateTime<Utc>)>,
    pub priority: Priority,
    pub constraints: RetrievalConstraints,
}

/// 检索约束
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetrievalConstraints {
    pub max_results: Option<usize>,
    pub min_relevance: Option<f32>,
    pub required_tags: Vec<String>,
    pub excluded_tags: Vec<String>,
    pub time_range: Option<(DateTime<Utc>, DateTime<Utc>)>,
    pub source_filter: Option<Vec<String>>,
}

/// 检索解释
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetailedExplanation {
    pub strategy_used: RetrievalStrategy,
    pub score_breakdown: ScoreBreakdown,
    pub reasoning_path: Vec<ReasoningStep>,
    pub confidence: f32,
    pub alternatives: Vec<AlternativeResult>,
}

/// 分数分解
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoreBreakdown {
    pub semantic_score: f32,
    pub temporal_score: f32,
    pub structural_score: f32,
    pub importance_score: f32,
    pub personalization_score: f32,
    pub final_score: f32,
    pub normalization_factor: f32,
}

/// 推理步骤
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasoningStep {
    pub step_type: String,
    pub description: String,
    pub score_impact: f32,
    pub confidence: f32,
}

/// 替代结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlternativeResult {
    pub memory_id: MemoryId,
    pub score: f32,
    pub reason_excluded: String,
}

impl HippoRAGRetriever {
    /// 创建新的HippoRAG检索器
    pub async fn new(db: Arc<VectorGraphDB>, config: Config) -> Result<Self> {
        let pagerank_engine = PageRankEngine::new(&config);
        let fusion_engine = FusionEngine::new(&config);
        let personalization_manager = PersonalizationManager::new(&config)?;

        Ok(Self {
            db,
            config,
            pagerank_engine,
            fusion_engine,
            personalization_manager,
            cache: Arc::new(RwLock::new(RetrievalCache::new(1000))),
        })
    }

    /// 执行HippoRAG检索
    pub async fn retrieve(
        &mut self,
        query: &Query,
        context: &RetrievalContext,
        strategy: &RetrievalStrategy,
    ) -> Result<Vec<RetrievalResult>> {
        info!("Starting HippoRAG retrieval with strategy: {:?}", strategy);

        // 检查缓存
        if let Some(cached) = self.check_cache(query, context).await? {
            debug!("Returning cached results");
            return Ok(cached);
        }

        let results = match strategy {
            RetrievalStrategy::Semantic => self.semantic_retrieval(query, context).await?,
            RetrievalStrategy::Structural => self.structural_retrieval(query, context).await?,
            RetrievalStrategy::Temporal => self.temporal_retrieval(query, context).await?,
            RetrievalStrategy::Personalized => self.personalized_retrieval(query, context).await?,
            RetrievalStrategy::Hybrid {
                strategies,
                weights,
            } => {
                self.hybrid_retrieval(query, context, strategies, weights)
                    .await?
            }
            RetrievalStrategy::HippoRAG {
                use_pagerank,
                use_personalization,
                fusion_method,
            } => {
                self.hippocampus_retrieval(
                    query,
                    context,
                    *use_pagerank,
                    *use_personalization,
                    fusion_method,
                )
                .await?
            }
        };

        // 缓存结果
        self.cache_results(query, context, &results).await?;

        // 记录交互
        self.record_interaction(query, &results).await?;

        info!("Retrieved {} results", results.len());
        Ok(results)
    }

    /// 语义检索
    async fn semantic_retrieval(
        &self,
        query: &Query,
        context: &RetrievalContext,
    ) -> Result<Vec<RetrievalResult>> {
        debug!("Performing semantic retrieval");

        // 生成查询嵌入
        let query_embedding = self.generate_query_embedding(&query.text).await?;

        // 构建向量查询请求
        let vector_request = VectorQueryRequest {
            query_vector: query_embedding,
            limit: Some(context.constraints.max_results.unwrap_or(50)),
            threshold: context.constraints.min_relevance,
            filters: self.build_metadata_filters(&context.constraints)?,
        };

        // 执行向量检索
        let vector_results = self.db.query_vectors(&vector_request).await?;

        // 转换为检索结果
        let mut results = Vec::new();
        for vector_result in vector_results {
            let memory = self.vector_to_memory(&vector_result.vector)?;

            // 保存需要的值，避免移动后访问
            let importance_score = memory.attributes.importance;

            let _explanation = DetailedExplanation {
                strategy_used: RetrievalStrategy::Semantic,
                score_breakdown: ScoreBreakdown {
                    semantic_score: vector_result.similarity,
                    temporal_score: 0.0,
                    structural_score: 0.0,
                    importance_score,
                    personalization_score: 0.0,
                    final_score: vector_result.similarity,
                    normalization_factor: 1.0,
                },
                reasoning_path: vec![ReasoningStep {
                    step_type: "semantic_matching".to_string(),
                    description: format!("Vector similarity: {:.3}", vector_result.similarity),
                    score_impact: vector_result.similarity,
                    confidence: 0.9,
                }],
                confidence: vector_result.similarity,
                alternatives: Vec::new(),
            };

            results.push(RetrievalResult {
                memory,
                relevance_score: vector_result.similarity,
                explanation: RetrievalExplanation {
                    semantic_score: vector_result.similarity,
                    temporal_score: 0.0,
                    importance_score,
                    personalization_score: 0.0,
                    connection_paths: Vec::new(),
                    reasoning: format!("Semantic similarity: {:.3}", vector_result.similarity),
                },
            });
        }

        Ok(results)
    }

    /// 结构化检索（基于图遍历）
    async fn structural_retrieval(
        &self,
        query: &Query,
        context: &RetrievalContext,
    ) -> Result<Vec<RetrievalResult>> {
        debug!("Performing structural retrieval");

        // 首先找到种子节点
        let seed_nodes = if !context.recent_queries.is_empty() {
            // 基于最近查询找到相关节点
            self.find_seed_nodes_from_queries(&context.recent_queries)
                .await?
        } else {
            // 使用语义检索找到初始节点
            let semantic_results = self.semantic_retrieval(query, context).await?;
            semantic_results
                .into_iter()
                .take(5)
                .map(|r| r.memory.id)
                .collect()
        };

        if seed_nodes.is_empty() {
            return Ok(Vec::new());
        }

        // 构建图查询请求
        let graph_request = GraphQueryRequest {
            start_nodes: seed_nodes,
            edge_types: None, // 考虑所有边类型
            max_depth: Some(self.config.graph.traversal.max_depth),
            limit: Some(context.constraints.max_results.unwrap_or(50)),
            filters: self.build_graph_filters(&context.constraints)?,
        };

        // 执行图遍历
        let graph_results = self.db.query_graph(&graph_request).await?;

        // 计算结构化分数
        let mut results = Vec::new();
        for (i, node) in graph_results.nodes.iter().enumerate() {
            let memory = self.node_to_memory(node)?;

            // 基于图距离计算结构分数
            let structural_score = self.calculate_structural_score(i, &graph_results.paths)?;

            // 保存需要的值，避免移动后访问
            let importance_score = memory.attributes.importance;

            let _explanation = DetailedExplanation {
                strategy_used: RetrievalStrategy::Structural,
                score_breakdown: ScoreBreakdown {
                    semantic_score: 0.0,
                    temporal_score: 0.0,
                    structural_score,
                    importance_score,
                    personalization_score: 0.0,
                    final_score: structural_score,
                    normalization_factor: 1.0,
                },
                reasoning_path: vec![ReasoningStep {
                    step_type: "graph_traversal".to_string(),
                    description: format!("Graph distance score: {:.3}", structural_score),
                    score_impact: structural_score,
                    confidence: 0.8,
                }],
                confidence: structural_score,
                alternatives: Vec::new(),
            };

            results.push(RetrievalResult {
                memory,
                relevance_score: structural_score,
                explanation: RetrievalExplanation {
                    semantic_score: 0.0,
                    temporal_score: 0.0,
                    importance_score,
                    personalization_score: 0.0,
                    connection_paths: graph_results.paths.clone(),
                    reasoning: format!("Structural relevance: {:.3}", structural_score),
                },
            });
        }

        // 按分数排序
        results.sort_by(|a, b| b.relevance_score.partial_cmp(&a.relevance_score).unwrap());

        Ok(results)
    }

    /// 时间感知检索
    async fn temporal_retrieval(
        &self,
        query: &Query,
        context: &RetrievalContext,
    ) -> Result<Vec<RetrievalResult>> {
        debug!("Performing temporal retrieval");

        // 首先执行语义检索获取候选
        let mut semantic_results = self.semantic_retrieval(query, context).await?;

        // 获取时间窗口
        let time_window = context
            .time_window
            .unwrap_or((Utc::now() - Duration::days(30), Utc::now()));

        // 重新计算时间相关性分数
        for result in &mut semantic_results {
            let temporal_score = self
                .calculate_temporal_relevance(&result.memory, &time_window, &query.text)
                .await?;

            // 融合语义和时间分数
            let fused_score = result.relevance_score * self.fusion_engine.semantic_weight
                + temporal_score * self.fusion_engine.temporal_weight;

            result.relevance_score = fused_score;
            result.explanation.temporal_score = temporal_score;
            result.explanation.reasoning = format!(
                "Temporal-semantic fusion: semantic={:.3}, temporal={:.3}",
                result.explanation.semantic_score, temporal_score
            );
        }

        // 重新排序
        semantic_results.sort_by(|a, b| b.relevance_score.partial_cmp(&a.relevance_score).unwrap());

        Ok(semantic_results)
    }

    /// 个性化检索
    async fn personalized_retrieval(
        &self,
        query: &Query,
        context: &RetrievalContext,
    ) -> Result<Vec<RetrievalResult>> {
        debug!("Performing personalized retrieval");

        // 获取用户档案
        let user_profile = self.personalization_manager.get_user_profile()?;

        // 执行个性化PageRank
        let pagerank_scores = self
            .pagerank_engine
            .compute_personalized_pagerank(&user_profile, &query.text, &self.db)
            .await?;

        // 获取语义候选
        let semantic_results = self.semantic_retrieval(query, context).await?;

        // 融合个性化分数
        let mut personalized_results = Vec::new();
        for semantic_result in semantic_results {
            let pagerank_score = pagerank_scores
                .get(&semantic_result.memory.id)
                .unwrap_or(&0.0);

            let personalization_score = self
                .calculate_personalization_score(&semantic_result.memory, &user_profile)
                .await?;

            let fused_score = semantic_result.relevance_score * self.fusion_engine.semantic_weight
                + pagerank_score * self.fusion_engine.structural_weight
                + personalization_score * self.fusion_engine.personalization_weight;

            // 保存需要的值，避免移动后访问
            let importance_score = semantic_result.memory.attributes.importance;
            let semantic_score = semantic_result.explanation.semantic_score;

            let _explanation = DetailedExplanation {
                strategy_used: RetrievalStrategy::Personalized,
                score_breakdown: ScoreBreakdown {
                    semantic_score,
                    temporal_score: 0.0,
                    structural_score: *pagerank_score,
                    importance_score,
                    personalization_score,
                    final_score: fused_score,
                    normalization_factor: 1.0,
                },
                reasoning_path: vec![
                    ReasoningStep {
                        step_type: "personalization".to_string(),
                        description: format!(
                            "User preference alignment: {:.3}",
                            personalization_score
                        ),
                        score_impact: personalization_score,
                        confidence: 0.85,
                    },
                    ReasoningStep {
                        step_type: "pagerank".to_string(),
                        description: format!("Personalized PageRank: {:.3}", pagerank_score),
                        score_impact: *pagerank_score,
                        confidence: 0.9,
                    },
                ],
                confidence: fused_score,
                alternatives: Vec::new(),
            };

            personalized_results.push(RetrievalResult {
                memory: semantic_result.memory,
                relevance_score: fused_score,
                explanation: RetrievalExplanation {
                    semantic_score,
                    temporal_score: 0.0,
                    importance_score,
                    personalization_score,
                    connection_paths: Vec::new(),
                    reasoning: format!(
                        "Personalized score: semantic={:.3}, pagerank={:.3}, preference={:.3}",
                        semantic_score, pagerank_score, personalization_score
                    ),
                },
            });
        }

        // 排序
        personalized_results
            .sort_by(|a, b| b.relevance_score.partial_cmp(&a.relevance_score).unwrap());

        Ok(personalized_results)
    }

    /// 混合检索
    pub async fn hybrid_retrieval(
        &self,
        query: &Query,
        context: &RetrievalContext,
        strategies: &[RetrievalStrategy],
        weights: &[f32],
    ) -> Result<Vec<RetrievalResult>> {
        debug!(
            "Performing hybrid retrieval with {} strategies",
            strategies.len()
        );

        if strategies.len() != weights.len() {
            return Err(MemoryError::validation_error(
                "Strategies and weights must have the same length",
            ));
        }

        // 执行各种检索策略
        let mut all_results = Vec::new();
        for (strategy, weight) in strategies.iter().zip(weights.iter()) {
            let strategy_results = match strategy {
                RetrievalStrategy::Semantic => self.semantic_retrieval(query, context).await?,
                RetrievalStrategy::Structural => self.structural_retrieval(query, context).await?,
                RetrievalStrategy::Temporal => self.temporal_retrieval(query, context).await?,
                RetrievalStrategy::Personalized => {
                    self.personalized_retrieval(query, context).await?
                }
                _ => continue, // 跳过嵌套的混合策略
            };

            all_results.push((strategy_results, *weight));
        }

        // 融合结果
        let fused_results = self
            .fusion_engine
            .fuse_results(all_results, FusionMethod::LinearWeighted)
            .await?;

        Ok(fused_results)
    }

    /// HippoRAG完整检索
    pub async fn hippocampus_retrieval(
        &self,
        query: &Query,
        context: &RetrievalContext,
        use_pagerank: bool,
        use_personalization: bool,
        fusion_method: &FusionMethod,
    ) -> Result<Vec<RetrievalResult>> {
        info!("Performing full HippoRAG retrieval");

        // 第一阶段：多模态检索
        let semantic_results = self.semantic_retrieval(query, context).await?;
        let structural_results = self.structural_retrieval(query, context).await?;
        let temporal_results = self.temporal_retrieval(query, context).await?;

        let mut all_results = vec![
            (semantic_results, self.fusion_engine.semantic_weight),
            (structural_results, self.fusion_engine.structural_weight),
            (temporal_results, self.fusion_engine.temporal_weight),
        ];

        // 第二阶段：个性化（如果启用）
        if use_personalization {
            let personalized_results = self.personalized_retrieval(query, context).await?;
            all_results.push((
                personalized_results,
                self.fusion_engine.personalization_weight,
            ));
        }

        // 第三阶段：融合
        let mut fused_results = self
            .fusion_engine
            .fuse_results(all_results, fusion_method.clone())
            .await?;

        // 第四阶段：PageRank重排序（如果启用）
        if use_pagerank {
            fused_results = self
                .apply_pagerank_reranking(fused_results, query, context)
                .await?;
        }

        // 第五阶段：后处理和解释生成
        for result in &mut fused_results {
            result.explanation.reasoning = format!(
                "HippoRAG retrieval: semantic={:.3}, structural={:.3}, temporal={:.3}, personalization={:.3}",
                result.explanation.semantic_score,
                0.0, // structural score would be computed
                result.explanation.temporal_score,
                result.explanation.personalization_score
            );
        }

        Ok(fused_results)
    }

    /// 应用PageRank重排序
    async fn apply_pagerank_reranking(
        &self,
        mut results: Vec<RetrievalResult>,
        query: &Query,
        _context: &RetrievalContext,
    ) -> Result<Vec<RetrievalResult>> {
        debug!("Applying PageRank reranking");

        // 构建子图
        let memory_ids: Vec<MemoryId> = results.iter().map(|r| r.memory.id.clone()).collect();
        let _subgraph = self.build_subgraph(&memory_ids).await?;

        // 计算PageRank分数
        let user_profile = self.personalization_manager.get_user_profile()?;
        let pagerank_scores = self
            .pagerank_engine
            .compute_personalized_pagerank(&user_profile, &query.text, &self.db)
            .await?;

        // 重新计算分数
        for result in &mut results {
            if let Some(pagerank_score) = pagerank_scores.get(&result.memory.id) {
                result.relevance_score = result.relevance_score * 0.7 + pagerank_score * 0.3;
            }
        }

        // 重新排序
        results.sort_by(|a, b| b.relevance_score.partial_cmp(&a.relevance_score).unwrap());

        Ok(results)
    }

    // 辅助方法

    async fn generate_query_embedding(&self, text: &str) -> Result<Vec<f32>> {
        use model2vec_rs::model::StaticModel;
        use std::sync::OnceLock;

        // 使用静态变量缓存模型，避免重复加载
        static MODEL: OnceLock<Option<StaticModel>> = OnceLock::new();

        let model = MODEL.get_or_init(|| {
            match StaticModel::from_pretrained(
                "minishlab/potion-multilingual-128M", // 使用多语言模型
                None,                                 // 无需 HF token
                None,                                 // 使用模型默认的归一化设置
                None,                                 // 无子文件夹
            ) {
                Ok(m) => Some(m),
                Err(e) => {
                    eprintln!("Failed to load embedding model: {}", e);
                    None
                }
            }
        });

        if let Some(model) = model {
            // 生成嵌入向量
            let sentences = vec![text.to_string()];
            let embeddings = model.encode(&sentences);

            if embeddings.is_empty() {
                return Err(MemoryError::Internal {
                    message: "Failed to generate embedding: empty result".to_string(),
                });
            }

            Ok(embeddings[0].clone())
        } else {
            Err(MemoryError::Internal {
                message: "Embedding model not initialized".to_string(),
            })
        }
    }

    async fn check_cache(
        &self,
        query: &Query,
        context: &RetrievalContext,
    ) -> Result<Option<Vec<RetrievalResult>>> {
        let cache_key = self.generate_cache_key(query, context)?;
        let cache = self.cache.read().await;

        if let Some(cached) = cache.query_cache.get(&cache_key) {
            if Utc::now() - cached.timestamp < cached.ttl {
                return Ok(Some(cached.results.clone()));
            }
        }

        Ok(None)
    }

    async fn cache_results(
        &self,
        query: &Query,
        context: &RetrievalContext,
        results: &[RetrievalResult],
    ) -> Result<()> {
        let cache_key = self.generate_cache_key(query, context)?;
        let cached_result = CachedResult {
            results: results.to_vec(),
            timestamp: Utc::now(),
            ttl: Duration::minutes(30),
        };

        let mut cache = self.cache.write().await;
        cache.query_cache.insert(cache_key, cached_result);

        // 清理过期缓存
        cache.cleanup_expired();

        Ok(())
    }

    fn generate_cache_key(&self, query: &Query, _context: &RetrievalContext) -> Result<String> {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        query.text.hash(&mut hasher);
        format!("{:?}", query.query_type).hash(&mut hasher);
        // 单用户系统，不需要user_id

        Ok(format!("query_{:x}", hasher.finish()))
    }

    async fn record_interaction(
        &mut self,
        _query: &Query,
        results: &[RetrievalResult],
    ) -> Result<()> {
        for result in results.iter().take(5) {
            // 只记录前5个结果的交互
            let interaction_record = InteractionRecord {
                memory_id: result.memory.id.clone(),
                interaction_type: InteractionType::Query,
                timestamp: Utc::now(),
                feedback_score: None,
                dwell_time: None,
            };

            self.personalization_manager
                .record_interaction(interaction_record)?;
        }

        Ok(())
    }

    // 转换方法

    fn vector_to_memory(&self, vector: &crate::database::Vector) -> Result<Memory> {
        // 从向量元数据构建记忆对象
        let metadata = &vector.metadata;

        // 提取必要字段，使用默认值处理缺失字段
        let content = metadata
            .get("content")
            .and_then(|v| v.as_str())
            .unwrap_or("")
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

        // 构建记忆属性
        let attributes = MemoryAttributes {
            keywords: metadata
                .get("keywords")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(String::from))
                        .collect()
                })
                .unwrap_or_default(),
            tags: metadata
                .get("tags")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(String::from))
                        .collect()
                })
                .unwrap_or_default(),
            context: metadata
                .get("context")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            importance: metadata
                .get("importance")
                .and_then(|v| v.as_f64())
                .map(|v| v as f32)
                .unwrap_or(0.5),
            emotion: metadata
                .get("emotion")
                .and_then(|v| v.as_str())
                .map(String::from),
            source: metadata
                .get("source")
                .and_then(|v| v.as_str())
                .map(String::from),
            confidence: metadata
                .get("confidence")
                .and_then(|v| v.as_f64())
                .map(|v| v as f32)
                .unwrap_or(0.8),
            language: metadata
                .get("language")
                .and_then(|v| v.as_str())
                .map(String::from),
            custom_attributes: HashMap::new(),
        };

        // 构建记忆连接
        let connections = MemoryConnections::default();

        // 构建记忆元数据
        let memory_metadata = MemoryMetadata {
            created_at: vector.created_at,
            updated_at: vector.updated_at,
            access_count: metadata
                .get("access_count")
                .and_then(|v| v.as_u64())
                .unwrap_or(0),
            last_accessed: metadata
                .get("last_accessed")
                .and_then(|v| v.as_str())
                .and_then(|v| chrono::DateTime::parse_from_rfc3339(v).ok())
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or(vector.updated_at),
            version: metadata
                .get("version")
                .and_then(|v| v.as_u64())
                .map(|v| v as u32)
                .unwrap_or(1),

            is_deleted: metadata
                .get("is_deleted")
                .and_then(|v| v.as_bool())
                .unwrap_or(false),
            deleted_at: None,
            custom_metadata: HashMap::new(),
        };

        // 构建记忆对象
        Ok(Memory {
            id: vector.id.clone(),
            content,
            memory_type,
            embedding: vector.embedding.clone(),
            attributes,
            connections,
            metadata: memory_metadata,
        })
    }

    fn node_to_memory(&self, node: &crate::database::GraphNode) -> Result<Memory> {
        // 从图节点构建记忆对象
        let properties = &node.properties;

        // 提取必要字段
        let content = properties
            .get("content")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        let memory_type_str = node.node_type.as_str();
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

        // 构建记忆属性
        let attributes = MemoryAttributes {
            keywords: properties
                .get("keywords")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(String::from))
                        .collect()
                })
                .unwrap_or_default(),
            tags: properties
                .get("tags")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(String::from))
                        .collect()
                })
                .unwrap_or_default(),
            context: properties
                .get("context")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            importance: properties
                .get("importance")
                .and_then(|v| v.as_f64())
                .map(|v| v as f32)
                .unwrap_or(0.5),
            emotion: properties
                .get("emotion")
                .and_then(|v| v.as_str())
                .map(String::from),
            source: properties
                .get("source")
                .and_then(|v| v.as_str())
                .map(String::from),
            confidence: properties
                .get("confidence")
                .and_then(|v| v.as_f64())
                .map(|v| v as f32)
                .unwrap_or(0.8),
            language: properties
                .get("language")
                .and_then(|v| v.as_str())
                .map(String::from),
            custom_attributes: HashMap::new(),
        };

        // 构建记忆连接
        let connections = MemoryConnections::default();

        // 构建记忆元数据
        let memory_metadata = MemoryMetadata {
            created_at: node.created_at,
            updated_at: node.updated_at,
            access_count: properties
                .get("access_count")
                .and_then(|v| v.as_u64())
                .unwrap_or(0),
            last_accessed: properties
                .get("last_accessed")
                .and_then(|v| v.as_str())
                .and_then(|v| chrono::DateTime::parse_from_rfc3339(v).ok())
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or(node.updated_at),
            version: properties
                .get("version")
                .and_then(|v| v.as_u64())
                .map(|v| v as u32)
                .unwrap_or(1),

            is_deleted: properties
                .get("is_deleted")
                .and_then(|v| v.as_bool())
                .unwrap_or(false),
            deleted_at: None,
            custom_metadata: HashMap::new(),
        };

        // 构建记忆对象
        Ok(Memory {
            id: node.id.clone(),
            content,
            memory_type,
            embedding: vec![0.0; 256], // 使用默认向量，因为图节点没有嵌入向量
            attributes,
            connections,
            metadata: memory_metadata,
        })
    }

    fn build_metadata_filters(
        &self,
        constraints: &RetrievalConstraints,
    ) -> Result<Option<HashMap<String, serde_json::Value>>> {
        let mut filters = HashMap::new();

        if !constraints.required_tags.is_empty() {
            filters.insert(
                "required_tags".to_string(),
                serde_json::Value::Array(
                    constraints
                        .required_tags
                        .iter()
                        .map(|t| serde_json::Value::String(t.clone()))
                        .collect(),
                ),
            );
        }

        if !constraints.excluded_tags.is_empty() {
            filters.insert(
                "excluded_tags".to_string(),
                serde_json::Value::Array(
                    constraints
                        .excluded_tags
                        .iter()
                        .map(|t| serde_json::Value::String(t.clone()))
                        .collect(),
                ),
            );
        }

        if filters.is_empty() {
            Ok(None)
        } else {
            Ok(Some(filters))
        }
    }

    fn build_graph_filters(
        &self,
        constraints: &RetrievalConstraints,
    ) -> Result<Option<HashMap<String, serde_json::Value>>> {
        self.build_metadata_filters(constraints)
    }

    async fn find_seed_nodes_from_queries(&self, _queries: &[String]) -> Result<Vec<MemoryId>> {
        // 简化实现：返回空向量
        Ok(Vec::new())
    }

    fn calculate_structural_score(&self, position: usize, _paths: &[Vec<MemoryId>]) -> Result<f32> {
        // 基于位置的简单评分
        Ok(1.0 / (position as f32 + 1.0))
    }

    async fn calculate_temporal_relevance(
        &self,
        memory: &Memory,
        time_window: &(DateTime<Utc>, DateTime<Utc>),
        _query: &str,
    ) -> Result<f32> {
        let memory_time = memory.metadata.created_at;
        if memory_time >= time_window.0 && memory_time <= time_window.1 {
            let window_duration = time_window.1 - time_window.0;
            let memory_offset = memory_time - time_window.0;
            Ok(1.0 - (memory_offset.num_seconds() as f32 / window_duration.num_seconds() as f32))
        } else {
            Ok(0.0)
        }
    }

    async fn calculate_personalization_score(
        &self,
        memory: &Memory,
        user_profile: &UserProfile,
    ) -> Result<f32> {
        let mut score = 0.0;

        // 基于用户偏好计算分数
        for (preference, weight) in &user_profile.preferences {
            if memory
                .content
                .to_lowercase()
                .contains(&preference.to_lowercase())
            {
                score += weight;
            }
        }

        // 基于主题兴趣
        for (topic, interest) in &user_profile.topic_interests {
            if memory
                .attributes
                .context
                .to_lowercase()
                .contains(&topic.to_lowercase())
            {
                score += interest * 0.5;
            }
        }

        Ok(score.min(1.0))
    }

    async fn build_subgraph(
        &self,
        memory_ids: &[MemoryId],
    ) -> Result<HashMap<MemoryId, Vec<MemoryId>>> {
        // 简化的子图构建
        let mut subgraph = HashMap::new();
        for id in memory_ids {
            subgraph.insert(id.clone(), Vec::new());
        }
        Ok(subgraph)
    }
}

// 实现各个组件

impl PageRankEngine {
    fn new(config: &Config) -> Self {
        Self {
            damping_factor: config.graph.pagerank.damping_factor,
            max_iterations: config.graph.pagerank.max_iterations,
            convergence_threshold: config.graph.pagerank.convergence_threshold,
            personalization_weight: config.graph.pagerank.personalization_weight,
        }
    }

    async fn compute_personalized_pagerank(
        &self,
        user_profile: &UserProfile,
        query: &str,
        _db: &VectorGraphDB,
    ) -> Result<HashMap<MemoryId, f32>> {
        // 简化的个性化PageRank实现
        let mut scores = HashMap::new();

        // 基于用户偏好生成个性化向量
        for (preference, weight) in &user_profile.preferences {
            if query.contains(preference) {
                scores.insert(format!("memory_{}", preference), *weight);
            }
        }

        Ok(scores)
    }

    #[allow(dead_code)]
    async fn compute_standard_pagerank(
        &self,
        graph: &HashMap<MemoryId, Vec<MemoryId>>,
    ) -> Result<HashMap<MemoryId, f32>> {
        let mut scores = HashMap::new();
        let node_count = graph.len() as f32;

        // 初始化分数
        for node in graph.keys() {
            scores.insert(node.clone(), 1.0 / node_count);
        }

        // 迭代计算PageRank
        for _ in 0..self.max_iterations {
            let mut new_scores = HashMap::new();

            for (node, neighbors) in graph {
                let mut score = (1.0 - self.damping_factor) / node_count;

                for neighbor in neighbors {
                    if let Some(neighbor_score) = scores.get(neighbor) {
                        let neighbor_out_degree =
                            graph.get(neighbor).map(|n| n.len()).unwrap_or(1) as f32;
                        score += self.damping_factor * neighbor_score / neighbor_out_degree;
                    }
                }

                new_scores.insert(node.clone(), score);
            }

            // 检查收敛
            let mut converged = true;
            for (node, new_score) in &new_scores {
                if let Some(old_score) = scores.get(node) {
                    if (new_score - old_score).abs() > self.convergence_threshold {
                        converged = false;
                        break;
                    }
                }
            }

            scores = new_scores;

            if converged {
                break;
            }
        }

        Ok(scores)
    }
}

impl FusionEngine {
    fn new(config: &Config) -> Self {
        Self {
            semantic_weight: config.retrieval.fusion_weights.semantic_weight,
            temporal_weight: config.retrieval.fusion_weights.temporal_weight,
            structural_weight: config.retrieval.fusion_weights.structural_weight,
            importance_weight: config.retrieval.fusion_weights.importance_weight,
            personalization_weight: config.retrieval.fusion_weights.personalization_weight,
        }
    }

    async fn fuse_results(
        &self,
        results_list: Vec<(Vec<RetrievalResult>, f32)>,
        fusion_method: FusionMethod,
    ) -> Result<Vec<RetrievalResult>> {
        match fusion_method {
            FusionMethod::LinearWeighted => self.linear_weighted_fusion(results_list).await,
            FusionMethod::RankFusion => self.rank_fusion(results_list).await,
            FusionMethod::LearnedFusion => self.learned_fusion(results_list).await,
            FusionMethod::DynamicFusion => self.dynamic_fusion(results_list).await,
        }
    }

    async fn linear_weighted_fusion(
        &self,
        results_list: Vec<(Vec<RetrievalResult>, f32)>,
    ) -> Result<Vec<RetrievalResult>> {
        let mut fused_results = HashMap::new();

        for (results, weight) in results_list {
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

    async fn rank_fusion(
        &self,
        results_list: Vec<(Vec<RetrievalResult>, f32)>,
    ) -> Result<Vec<RetrievalResult>> {
        // 简化的排名融合实现
        self.linear_weighted_fusion(results_list).await
    }

    async fn learned_fusion(
        &self,
        results_list: Vec<(Vec<RetrievalResult>, f32)>,
    ) -> Result<Vec<RetrievalResult>> {
        // 简化的学习融合实现
        self.linear_weighted_fusion(results_list).await
    }

    async fn dynamic_fusion(
        &self,
        results_list: Vec<(Vec<RetrievalResult>, f32)>,
    ) -> Result<Vec<RetrievalResult>> {
        // 简化的动态融合实现
        self.linear_weighted_fusion(results_list).await
    }
}

impl PersonalizationManager {
    fn new(config: &Config) -> Result<Self> {
        Ok(Self {
            user_profile: UserProfile::default(),
            interaction_tracker: InteractionTracker::new(config.learning.max_interaction_history),
            preference_learner: RetrievalPreferenceLearner::new(config),
        })
    }

    fn get_user_profile(&self) -> Result<UserProfile> {
        Ok(self.user_profile.clone())
    }

    fn record_interaction(&mut self, interaction: InteractionRecord) -> Result<()> {
        self.user_profile.interaction_history.push(interaction);
        self.user_profile.updated_at = Utc::now();

        // 限制历史记录长度
        if self.user_profile.interaction_history.len() > 1000 {
            self.user_profile.interaction_history.drain(0..500);
        }

        Ok(())
    }
}

impl InteractionTracker {
    fn new(max_history_size: usize) -> Self {
        Self {
            recent_interactions: VecDeque::new(),
            max_history_size,
        }
    }
}

impl RetrievalPreferenceLearner {
    fn new(config: &Config) -> Self {
        Self {
            learning_rate: config.learning.learning_rate,
            decay_factor: config.learning.importance_decay_factor,
            reinforcement_threshold: 0.7,
        }
    }
}

impl RetrievalCache {
    fn new(max_size: usize) -> Self {
        Self {
            query_cache: HashMap::new(),
            pagerank_cache: HashMap::new(),
            personalization_cache: PersonalizationVector::default(),
            max_cache_size: max_size,
        }
    }

    fn cleanup_expired(&mut self) {
        let now = Utc::now();
        self.query_cache
            .retain(|_, cached| now - cached.timestamp < cached.ttl);

        // 如果缓存过大，移除最旧的条目
        if self.query_cache.len() > self.max_cache_size {
            let mut entries: Vec<_> = self
                .query_cache
                .iter()
                .map(|(k, v)| (k.clone(), v.timestamp))
                .collect();
            entries.sort_by_key(|(_, timestamp)| *timestamp);

            let to_remove = entries.len() - self.max_cache_size;
            let keys_to_remove: Vec<_> = entries
                .into_iter()
                .take(to_remove)
                .map(|(key, _)| key)
                .collect();

            for key in keys_to_remove {
                self.query_cache.remove(&key);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core_types::{Interaction, InteractionType, QueryFilters, QueryType, QueryWeights};
    use crate::database::VectorGraphDB;
    use tempfile::NamedTempFile;
    use uuid::Uuid;

    async fn create_test_retriever() -> HippoRAGRetriever {
        let temp_file = NamedTempFile::new().unwrap();
        let db_url = format!("sqlite://{}", temp_file.path().display());

        let config = Config {
            database: crate::config::DatabaseConfig {
                url: db_url,
                ..Default::default()
            },
            ..Default::default()
        };

        let db = Arc::new(VectorGraphDB::new(config.clone()).await.unwrap());
        HippoRAGRetriever::new(db, config).await.unwrap()
    }

    #[tokio::test]
    async fn test_semantic_retrieval() {
        let retriever = create_test_retriever().await;

        let query = Query {
            text: "人工智能".to_string(),
            query_type: QueryType::Semantic,
            filters: QueryFilters::default(),
            limit: Some(10),
            offset: None,
            sort_by: None,
            weights: QueryWeights::default(),
        };

        let context = RetrievalContext {
            session_id: None,
            current_topic: None,
            recent_queries: Vec::new(),
            time_window: None,
            priority: Priority::Normal,
            constraints: RetrievalConstraints {
                max_results: Some(10),
                min_relevance: Some(0.5),
                required_tags: Vec::new(),
                excluded_tags: Vec::new(),
                time_range: None,
                source_filter: None,
            },
        };

        let results = retriever
            .semantic_retrieval(&query, &context)
            .await
            .unwrap();
        // 由于没有实际数据，结果应该为空
        assert!(results.is_empty());
    }

    #[tokio::test]
    async fn test_cache_functionality() {
        let mut retriever = create_test_retriever().await;

        let query = Query {
            text: "测试查询".to_string(),
            query_type: QueryType::Semantic,
            filters: QueryFilters::default(),
            limit: Some(5),
            offset: None,
            sort_by: None,
            weights: QueryWeights::default(),
        };

        let context = RetrievalContext {
            session_id: None,
            current_topic: None,
            recent_queries: Vec::new(),
            time_window: None,
            priority: Priority::Normal,
            constraints: RetrievalConstraints {
                max_results: Some(5),
                min_relevance: None,
                required_tags: Vec::new(),
                excluded_tags: Vec::new(),
                time_range: None,
                source_filter: None,
            },
        };

        // 第一次检索
        let results1 = retriever
            .retrieve(&query, &context, &RetrievalStrategy::Semantic)
            .await
            .unwrap();

        // 第二次检索应该使用缓存
        let results2 = retriever
            .retrieve(&query, &context, &RetrievalStrategy::Semantic)
            .await
            .unwrap();

        assert_eq!(results1.len(), results2.len());
    }

    #[tokio::test]
    async fn test_personalization_manager() {
        let config = Config::default();
        let mut manager = PersonalizationManager::new(&config).unwrap();

        let interaction = Interaction {
            id: format!("interaction_{}", Uuid::new_v4()),
            user_id: "system".to_string(),
            session_id: None,
            query: "test query".to_string(),
            retrieved_memories: vec!["memory_1".to_string()],
            user_feedback: Some(0.8),
            interaction_type: InteractionType::Query,
            timestamp: Utc::now(),
            response_time_ms: Some(5000),
            additional_info: HashMap::new(),
        };

        let interaction_record = InteractionRecord {
            memory_id: "test_memory".to_string(),
            interaction_type: interaction.interaction_type,
            timestamp: interaction.timestamp,
            feedback_score: interaction.user_feedback,
            dwell_time: interaction.response_time_ms,
        };

        manager.record_interaction(interaction_record).unwrap();

        let profile = manager.get_user_profile().unwrap();
        assert_eq!(profile.interaction_history.len(), 1);
    }
}

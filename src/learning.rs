//! 学习模块
//!
//! 实现记忆系统的自适应学习和演化功能，包括重要性调整、连接强度更新、
//! 用户偏好学习等。

use crate::config::Config;
use crate::core_types::{Connection, ConnectionId, MemoryId};
use crate::database::VectorGraphDB;
use crate::error::Result;
use chrono::{DateTime, Duration, Utc};
use log::{debug, info};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use tokio::sync::RwLock;

/// 学习引擎
///
/// 负责记忆系统的自适应学习，包括重要性调整、连接演化、用户偏好学习等。
#[derive(Debug)]
#[allow(dead_code)]
pub struct LearningEngine {
    /// 向量图数据库
    db: Arc<VectorGraphDB>,
    /// 配置
    config: Config,
    /// 重要性学习器
    importance_learner: ImportanceLearner,
    /// 连接学习器
    connection_learner: ConnectionLearner,
    /// 偏好学习器
    preference_learner: PreferenceLearner,
    /// 模式检测器
    pattern_detector: PatternDetector,
    /// 反馈处理器
    feedback_processor: FeedbackProcessor,
    /// 学习统计
    stats: Arc<RwLock<LearningStats>>,
}

/// 重要性学习器
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ImportanceLearner {
    /// 学习率
    learning_rate: f32,
    /// 衰减因子
    decay_factor: f32,
    /// 强化因子
    reinforcement_factor: f32,
    /// 访问权重
    access_weight: f32,
    /// 反馈权重
    feedback_weight: f32,
    /// 时间权重
    temporal_weight: f32,
    /// 连接权重
    connection_weight: f32,
}

/// 连接学习器
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ConnectionLearner {
    /// 创建阈值
    creation_threshold: f32,
    /// 强化率
    strengthening_rate: f32,
    /// 弱化率
    weakening_rate: f32,
    /// 剪枝阈值
    pruning_threshold: f32,
    /// 语义权重
    semantic_weight: f32,
    /// 时间权重
    temporal_weight: f32,
    /// 因果权重
    causal_weight: f32,
}

/// 偏好学习器
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct PreferenceLearner {
    /// 学习率 - 控制学习速度
    learning_rate: f32,
    /// 探索率 - 控制探索新偏好的程度
    exploration_rate: f32,
    /// 偏好衰减率 - 控制旧偏好的遗忘速度
    preference_decay: f32,
    /// 主题学习率 - 主题相关的学习速度
    topic_learning_rate: f32,
    /// 时间学习率 - 时间模式的学习速度
    temporal_learning_rate: f32,
}

/// 模式检测器
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct PatternDetector {
    /// 最小模式频率阈值
    min_pattern_frequency: u32,
    /// 时间窗口大小
    temporal_window: Duration,
    /// 语义相似度阈值
    semantic_similarity_threshold: f32,
    /// 模式缓存
    pattern_cache: Arc<RwLock<HashMap<String, DetectedPattern>>>,
}

/// 反馈处理器
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct FeedbackProcessor {
    /// 正面反馈阈值
    positive_threshold: f32,
    /// 负面反馈阈值
    negative_threshold: f32,
    /// 隐式反馈权重
    implicit_feedback_weight: f32,
    /// 显式反馈权重
    explicit_feedback_weight: f32,
    /// 反馈历史记录
    feedback_history: Arc<RwLock<VecDeque<FeedbackRecord>>>,
}

/// 学习统计信息
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LearningStats {
    pub total_learning_cycles: u64,
    pub importance_adjustments: u64,
    pub connections_created: u64,
    pub connections_strengthened: u64,
    pub connections_weakened: u64,
    pub connections_pruned: u64,
    pub patterns_detected: u64,
    pub feedback_processed: u64,
    pub average_learning_accuracy: f32,
    pub last_learning_cycle: Option<DateTime<Utc>>,
}

/// 检测到的模式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectedPattern {
    pub pattern_id: String,
    pub pattern_type: PatternType,
    pub frequency: u32,
    pub confidence: f32,
    pub elements: Vec<String>,
    pub temporal_signature: Option<TemporalSignature>,
    pub first_detected: DateTime<Utc>,
    pub last_seen: DateTime<Utc>,
}

/// 模式类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PatternType {
    /// 访问模式
    AccessPattern,
    /// 查询模式
    QueryPattern,
    /// 主题模式
    TopicPattern,
    /// 时间模式
    TemporalPattern,
    /// 连接模式
    ConnectionPattern,
    /// 反馈模式
    FeedbackPattern,
}

/// 时间签名
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalSignature {
    pub preferred_hours: Vec<u8>,
    pub preferred_days: Vec<u8>,
    pub session_patterns: Vec<SessionPattern>,
    pub frequency_distribution: HashMap<String, f32>,
}

/// 会话模式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionPattern {
    pub duration: Duration,
    pub query_count: u32,
    pub topic_switches: u32,
    pub feedback_ratio: f32,
}

/// 反馈记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedbackRecord {
    pub memory_id: MemoryId,

    pub feedback_type: FeedbackType,
    pub score: f32,
    pub context: FeedbackContext,
    pub timestamp: DateTime<Utc>,
}

/// 反馈类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FeedbackType {
    /// 显式反馈（用户主动评分）
    Explicit,
    /// 隐式反馈（基于行为推断）
    Implicit,
    /// 点击反馈
    Click,
    /// 停留时间反馈
    DwellTime,
    /// 分享反馈
    Share,
    /// 收藏反馈
    Bookmark,
    /// 忽略反馈
    Ignore,
}

/// 反馈上下文
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedbackContext {
    pub query: String,
    pub result_position: usize,
    pub session_id: String,
    pub device_type: Option<String>,
    pub time_of_day: u8,
    pub day_of_week: u8,
}

/// 学习任务
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LearningTask {
    /// 重要性调整
    ImportanceAdjustment {
        memory_ids: Vec<MemoryId>,
        trigger: ImportanceTrigger,
    },
    /// 连接演化
    ConnectionEvolution {
        connection_ids: Vec<ConnectionId>,
        evolution_type: ConnectionEvolutionType,
    },
    /// 偏好更新
    PreferenceUpdate { update_type: PreferenceUpdateType },
    /// 模式检测
    PatternDetection {
        data_window: (DateTime<Utc>, DateTime<Utc>),
        pattern_types: Vec<PatternType>,
    },
    /// 反馈处理
    FeedbackProcessing { feedback_batch: Vec<FeedbackRecord> },
}

/// 重要性触发器
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImportanceTrigger {
    /// 时间衰减
    TimeDecay,
    /// 访问频率
    AccessFrequency,
    /// 用户反馈
    UserFeedback,
    /// 连接强度
    ConnectionStrength,
    /// 查询相关性
    QueryRelevance,
}

/// 连接演化类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConnectionEvolutionType {
    /// 强化连接
    Strengthen,
    /// 弱化连接
    Weaken,
    /// 创建新连接
    Create,
    /// 删除连接
    Prune,
}

/// 偏好更新类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PreferenceUpdateType {
    /// 主题偏好
    TopicPreference,
    /// 时间偏好
    TemporalPreference,
    /// 查询偏好
    QueryPreference,
    /// 反馈偏好
    FeedbackPreference,
}

/// 学习结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningResult {
    pub task_type: String,
    pub success: bool,
    pub changes_made: u32,
    pub accuracy_improvement: Option<f32>,
    pub execution_time: Duration,
    pub details: HashMap<String, serde_json::Value>,
}

impl LearningEngine {
    /// 创建新的学习引擎
    pub async fn new(db: Arc<VectorGraphDB>, config: Config) -> Result<Self> {
        let importance_learner = ImportanceLearner::new(&config);
        let connection_learner = ConnectionLearner::new(&config);
        let preference_learner = PreferenceLearner::new(&config);
        let pattern_detector = PatternDetector::new(&config);
        let feedback_processor = FeedbackProcessor::new(&config);

        Ok(Self {
            db,
            config,
            importance_learner,
            connection_learner,
            preference_learner,
            pattern_detector,
            feedback_processor,
            stats: Arc::new(RwLock::new(LearningStats::default())),
        })
    }

    /// 执行学习任务
    pub async fn execute_learning_task(&self, task: LearningTask) -> Result<LearningResult> {
        let start_time = Utc::now();
        let task_type_name = match &task {
            LearningTask::ImportanceAdjustment { .. } => "ImportanceAdjustment",
            LearningTask::ConnectionEvolution { .. } => "ConnectionEvolution",
            LearningTask::PreferenceUpdate { .. } => "PreferenceUpdate",
            LearningTask::PatternDetection { .. } => "PatternDetection",
            LearningTask::FeedbackProcessing { .. } => "FeedbackProcessing",
        };
        info!("Executing learning task: {}", task_type_name);

        let result = match task {
            LearningTask::ImportanceAdjustment {
                memory_ids,
                trigger,
            } => self.adjust_importance(memory_ids, trigger).await?,
            LearningTask::ConnectionEvolution {
                connection_ids,
                evolution_type,
            } => {
                self.evolve_connections(connection_ids, evolution_type)
                    .await?
            }
            LearningTask::PreferenceUpdate { update_type } => {
                self.update_preferences(update_type).await?
            }
            LearningTask::PatternDetection {
                data_window,
                ref pattern_types,
            } => {
                self.detect_patterns(data_window, pattern_types.clone())
                    .await?
            }
            LearningTask::FeedbackProcessing { ref feedback_batch } => {
                self.process_feedback_batch(feedback_batch.clone()).await?
            }
        };

        let execution_time = Utc::now() - start_time;

        // 更新统计信息
        self.update_stats(|stats| {
            stats.total_learning_cycles += 1;
            stats.last_learning_cycle = Some(Utc::now());
        })
        .await;

        info!("Learning task completed in {:?}", execution_time);
        Ok(LearningResult {
            task_type: task_type_name.to_string(),
            success: result.success,
            changes_made: result.changes_made,
            accuracy_improvement: result.accuracy_improvement,
            execution_time,
            details: result.details,
        })
    }

    /// 调整记忆重要性
    async fn adjust_importance(
        &self,
        memory_ids: Vec<MemoryId>,
        trigger: ImportanceTrigger,
    ) -> Result<LearningResult> {
        debug!("Adjusting importance for {} memories", memory_ids.len());

        let mut changes_made = 0;
        let mut details = HashMap::new();

        for memory_id in memory_ids {
            let adjustment = self
                .importance_learner
                .calculate_adjustment(&memory_id, &trigger, &self.db)
                .await?;

            if adjustment.abs() > 0.01 {
                // 只有显著变化才应用
                // TODO: 实际更新数据库中的重要性
                changes_made += 1;
                details.insert(
                    memory_id.clone(),
                    serde_json::json!({
                        "adjustment": adjustment,
                        "trigger": format!("{:?}", trigger)
                    }),
                );
            }
        }

        // 更新统计信息
        self.update_stats(|stats| {
            stats.importance_adjustments += changes_made as u64;
        })
        .await;

        Ok(LearningResult {
            task_type: "ImportanceAdjustment".to_string(),
            success: true,
            changes_made,
            accuracy_improvement: None,
            execution_time: Duration::zero(),
            details,
        })
    }

    /// 演化连接
    async fn evolve_connections(
        &self,
        connection_ids: Vec<ConnectionId>,
        evolution_type: ConnectionEvolutionType,
    ) -> Result<LearningResult> {
        debug!(
            "Evolving {} connections with type: {:?}",
            connection_ids.len(),
            evolution_type
        );

        let mut changes_made = 0;
        let mut details = HashMap::new();

        match evolution_type {
            ConnectionEvolutionType::Strengthen => {
                for connection_id in connection_ids {
                    let strengthening = self
                        .connection_learner
                        .calculate_strengthening(&connection_id)
                        .await?;
                    if strengthening > 0.0 {
                        // TODO: 实际更新连接强度
                        changes_made += 1;
                        details.insert(
                            connection_id,
                            serde_json::json!({"strengthening": strengthening}),
                        );
                    }
                }

                self.update_stats(|stats| {
                    stats.connections_strengthened += changes_made as u64;
                })
                .await;
            }
            ConnectionEvolutionType::Weaken => {
                for connection_id in connection_ids {
                    let weakening = self
                        .connection_learner
                        .calculate_weakening(&connection_id)
                        .await?;
                    if weakening > 0.0 {
                        // TODO: 实际更新连接强度
                        changes_made += 1;
                        details.insert(connection_id, serde_json::json!({"weakening": weakening}));
                    }
                }

                self.update_stats(|stats| {
                    stats.connections_weakened += changes_made as u64;
                })
                .await;
            }
            ConnectionEvolutionType::Create => {
                let new_connections = self
                    .connection_learner
                    .discover_new_connections(&connection_ids)
                    .await?;
                changes_made = new_connections.len() as u32;

                for (i, connection) in new_connections.iter().enumerate() {
                    details.insert(
                        format!("new_connection_{}", i),
                        serde_json::json!({
                            "from": connection.from_memory,
                            "to": connection.to_memory,
                            "type": format!("{:?}", connection.connection_type),
                            "strength": connection.strength
                        }),
                    );
                }

                self.update_stats(|stats| {
                    stats.connections_created += changes_made as u64;
                })
                .await;
            }
            ConnectionEvolutionType::Prune => {
                let pruned_connections = self
                    .connection_learner
                    .prune_weak_connections(&connection_ids)
                    .await?;
                changes_made = pruned_connections.len() as u32;

                for (i, connection_id) in pruned_connections.iter().enumerate() {
                    details.insert(
                        format!("pruned_connection_{}", i),
                        serde_json::json!({
                            "connection_id": connection_id
                        }),
                    );
                }

                self.update_stats(|stats| {
                    stats.connections_pruned += changes_made as u64;
                })
                .await;
            }
        }

        Ok(LearningResult {
            task_type: "ConnectionEvolution".to_string(),
            success: true,
            changes_made,
            accuracy_improvement: None,
            execution_time: Duration::zero(),
            details,
        })
    }

    /// 更新用户偏好
    async fn update_preferences(
        &self,
        update_type: PreferenceUpdateType,
    ) -> Result<LearningResult> {
        debug!("Updating preferences with type: {:?}", update_type);

        let updates = self
            .preference_learner
            .calculate_preference_updates(&update_type)
            .await?;

        let mut details = HashMap::new();
        details.insert(
            "update_type".to_string(),
            serde_json::Value::String(format!("{:?}", update_type)),
        );
        details.insert("updates".to_string(), serde_json::to_value(&updates)?);

        Ok(LearningResult {
            task_type: "PreferenceUpdate".to_string(),
            success: true,
            changes_made: updates.len() as u32,
            accuracy_improvement: None,
            execution_time: Duration::zero(),
            details,
        })
    }

    /// 检测模式
    async fn detect_patterns(
        &self,
        data_window: (DateTime<Utc>, DateTime<Utc>),
        pattern_types: Vec<PatternType>,
    ) -> Result<LearningResult> {
        debug!(
            "Detecting patterns in window: {:?} to {:?}",
            data_window.0, data_window.1
        );

        let mut detected_patterns = Vec::new();
        let mut details = HashMap::new();

        for pattern_type in pattern_types {
            let patterns = self
                .pattern_detector
                .detect_patterns_of_type(&pattern_type, &data_window)
                .await?;
            detected_patterns.extend(patterns);
        }

        // 存储检测到的模式
        for pattern in &detected_patterns {
            self.pattern_detector.store_pattern(pattern).await?;
        }

        details.insert(
            "detected_patterns".to_string(),
            serde_json::to_value(&detected_patterns)?,
        );
        details.insert(
            "data_window".to_string(),
            serde_json::json!({
                "start": data_window.0,
                "end": data_window.1
            }),
        );

        // 更新统计信息
        self.update_stats(|stats| {
            stats.patterns_detected += detected_patterns.len() as u64;
        })
        .await;

        Ok(LearningResult {
            task_type: "PatternDetection".to_string(),
            success: true,
            changes_made: detected_patterns.len() as u32,
            accuracy_improvement: None,
            execution_time: Duration::zero(),
            details,
        })
    }

    /// 处理反馈批次
    async fn process_feedback_batch(
        &self,
        feedback_batch: Vec<FeedbackRecord>,
    ) -> Result<LearningResult> {
        debug!(
            "Processing feedback batch of {} records",
            feedback_batch.len()
        );

        let mut processed_count = 0;
        let mut details = HashMap::new();

        for feedback in feedback_batch {
            let processing_result = self.feedback_processor.process_feedback(&feedback).await?;
            if processing_result.applied {
                processed_count += 1;
            }

            details.insert(
                format!("feedback_{}", feedback.memory_id),
                serde_json::json!({
                    "type": format!("{:?}", feedback.feedback_type),
                    "score": feedback.score,
                    "applied": processing_result.applied,
                    "impact": processing_result.impact
                }),
            );
        }

        // 更新统计信息
        self.update_stats(|stats| {
            stats.feedback_processed += processed_count as u64;
        })
        .await;

        Ok(LearningResult {
            task_type: "FeedbackProcessing".to_string(),
            success: true,
            changes_made: processed_count,
            accuracy_improvement: None,
            execution_time: Duration::zero(),
            details,
        })
    }

    /// 执行定期学习循环
    pub async fn run_learning_cycle(&self) -> Result<Vec<LearningResult>> {
        info!("Starting learning cycle");

        let mut results = Vec::new();

        // 1. 重要性衰减
        let importance_task = LearningTask::ImportanceAdjustment {
            memory_ids: self.get_all_memory_ids().await?,
            trigger: ImportanceTrigger::TimeDecay,
        };
        results.push(self.execute_learning_task(importance_task).await?);

        // 2. 连接演化
        let connection_task = LearningTask::ConnectionEvolution {
            connection_ids: self.get_all_connection_ids().await?,
            evolution_type: ConnectionEvolutionType::Weaken,
        };
        results.push(self.execute_learning_task(connection_task).await?);

        // 3. 模式检测
        let now = Utc::now();
        let pattern_task = LearningTask::PatternDetection {
            data_window: (now - Duration::days(7), now),
            pattern_types: vec![
                PatternType::AccessPattern,
                PatternType::QueryPattern,
                PatternType::TemporalPattern,
            ],
        };
        results.push(self.execute_learning_task(pattern_task).await?);

        // 4. 处理积累的反馈
        let feedback_batch = self.feedback_processor.get_pending_feedback().await?;
        if !feedback_batch.is_empty() {
            let feedback_task = LearningTask::FeedbackProcessing { feedback_batch };
            results.push(self.execute_learning_task(feedback_task).await?);
        }

        info!("Learning cycle completed with {} tasks", results.len());
        Ok(results)
    }

    /// 获取学习统计信息
    pub async fn get_stats(&self) -> LearningStats {
        self.stats.read().await.clone()
    }

    /// 记录用户反馈
    pub async fn record_feedback(&self, feedback: FeedbackRecord) -> Result<()> {
        info!("Recording feedback for memory: {}", feedback.memory_id);

        // 立即处理显式反馈
        if matches!(feedback.feedback_type, FeedbackType::Explicit) {
            self.feedback_processor.process_feedback(&feedback).await?;
        } else {
            // 隐式反馈加入批处理队列
            self.feedback_processor.queue_feedback(feedback).await?;
        }

        Ok(())
    }

    // 私有辅助方法

    async fn update_stats<F>(&self, updater: F)
    where
        F: FnOnce(&mut LearningStats),
    {
        let mut stats = self.stats.write().await;
        updater(&mut stats);
    }

    async fn get_all_memory_ids(&self) -> Result<Vec<MemoryId>> {
        // TODO: 从数据库获取所有记忆ID
        Ok(Vec::new())
    }

    async fn get_all_connection_ids(&self) -> Result<Vec<ConnectionId>> {
        // TODO: 从数据库获取所有连接ID
        Ok(Vec::new())
    }
}

// 实现各个学习组件

impl ImportanceLearner {
    fn new(config: &Config) -> Self {
        Self {
            learning_rate: config.learning.learning_rate,
            decay_factor: config.learning.importance_decay_factor,
            reinforcement_factor: 1.1,
            access_weight: 0.3,
            feedback_weight: 0.4,
            temporal_weight: 0.2,
            connection_weight: 0.1,
        }
    }

    async fn calculate_adjustment(
        &self,
        memory_id: &MemoryId,
        trigger: &ImportanceTrigger,
        _db: &VectorGraphDB,
    ) -> Result<f32> {
        match trigger {
            ImportanceTrigger::TimeDecay => {
                // 基于时间的衰减
                Ok(-self.decay_factor * self.learning_rate)
            }
            ImportanceTrigger::AccessFrequency => {
                // 基于访问频率的调整
                let access_count = self.get_recent_access_count(memory_id).await?;
                Ok(access_count as f32 * self.access_weight * self.learning_rate)
            }
            ImportanceTrigger::UserFeedback => {
                // 基于用户反馈的调整
                let feedback_score = self.get_recent_feedback_score(memory_id).await?;
                Ok(feedback_score * self.feedback_weight * self.learning_rate)
            }
            ImportanceTrigger::ConnectionStrength => {
                // 基于连接强度的调整
                let connection_strength = self.get_average_connection_strength(memory_id).await?;
                Ok(connection_strength * self.connection_weight * self.learning_rate)
            }
            ImportanceTrigger::QueryRelevance => {
                // 基于查询相关性的调整
                let relevance_score = self.get_recent_relevance_score(memory_id).await?;
                Ok(relevance_score * self.learning_rate)
            }
        }
    }

    async fn get_recent_access_count(&self, _memory_id: &MemoryId) -> Result<u32> {
        // TODO: 实现访问计数获取
        Ok(0)
    }

    async fn get_recent_feedback_score(&self, _memory_id: &MemoryId) -> Result<f32> {
        // TODO: 实现反馈分数获取
        Ok(0.0)
    }

    async fn get_average_connection_strength(&self, _memory_id: &MemoryId) -> Result<f32> {
        // TODO: 实现连接强度获取
        Ok(0.0)
    }

    async fn get_recent_relevance_score(&self, _memory_id: &MemoryId) -> Result<f32> {
        // TODO: 实现相关性分数获取
        Ok(0.0)
    }
}

impl ConnectionLearner {
    fn new(_config: &Config) -> Self {
        Self {
            creation_threshold: 0.7,
            strengthening_rate: 0.1,
            weakening_rate: 0.05,
            pruning_threshold: 0.1,
            semantic_weight: 0.4,
            temporal_weight: 0.3,
            causal_weight: 0.3,
        }
    }

    async fn calculate_strengthening(&self, _connection_id: &ConnectionId) -> Result<f32> {
        // TODO: 计算连接强化值
        Ok(self.strengthening_rate)
    }

    async fn calculate_weakening(&self, _connection_id: &ConnectionId) -> Result<f32> {
        // TODO: 计算连接弱化值
        Ok(self.weakening_rate)
    }

    async fn discover_new_connections(
        &self,
        _connection_ids: &[ConnectionId],
    ) -> Result<Vec<Connection>> {
        // TODO: 发现新连接
        Ok(Vec::new())
    }

    async fn prune_weak_connections(
        &self,
        connection_ids: &[ConnectionId],
    ) -> Result<Vec<ConnectionId>> {
        // TODO: 修剪弱连接
        let mut pruned = Vec::new();
        for connection_id in connection_ids {
            // 简化逻辑：随机修剪一些连接
            if connection_id.len() % 3 == 0 {
                pruned.push(connection_id.clone());
            }
        }
        Ok(pruned)
    }
}

impl PreferenceLearner {
    fn new(config: &Config) -> Self {
        Self {
            learning_rate: config.learning.learning_rate,
            exploration_rate: 0.1,
            preference_decay: 0.95,
            topic_learning_rate: 0.05,
            temporal_learning_rate: 0.03,
        }
    }

    async fn calculate_preference_updates(
        &self,
        update_type: &PreferenceUpdateType,
    ) -> Result<HashMap<String, f32>> {
        let mut updates = HashMap::new();

        match update_type {
            PreferenceUpdateType::TopicPreference => {
                // TODO: 基于用户行为更新主题偏好
                updates.insert("AI".to_string(), 0.1);
                updates.insert("机器学习".to_string(), 0.05);
            }
            PreferenceUpdateType::TemporalPreference => {
                // TODO: 更新时间偏好
                updates.insert("morning_preference".to_string(), 0.02);
            }
            PreferenceUpdateType::QueryPreference => {
                // TODO: 更新查询偏好
                updates.insert("semantic_queries".to_string(), 0.03);
            }
            PreferenceUpdateType::FeedbackPreference => {
                // TODO: 更新反馈偏好
                updates.insert("positive_feedback_weight".to_string(), 0.01);
            }
        }

        Ok(updates)
    }
}

impl PatternDetector {
    fn new(_config: &Config) -> Self {
        Self {
            min_pattern_frequency: 3,
            temporal_window: Duration::days(7),
            semantic_similarity_threshold: 0.8,
            pattern_cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    async fn detect_patterns_of_type(
        &self,
        pattern_type: &PatternType,
        data_window: &(DateTime<Utc>, DateTime<Utc>),
    ) -> Result<Vec<DetectedPattern>> {
        match pattern_type {
            PatternType::AccessPattern => self.detect_access_patterns(data_window).await,
            PatternType::QueryPattern => self.detect_query_patterns(data_window).await,
            PatternType::TopicPattern => self.detect_topic_patterns(data_window).await,
            PatternType::TemporalPattern => self.detect_temporal_patterns(data_window).await,
            PatternType::ConnectionPattern => self.detect_connection_patterns(data_window).await,
            PatternType::FeedbackPattern => self.detect_feedback_patterns(data_window).await,
        }
    }

    async fn detect_access_patterns(
        &self,
        _data_window: &(DateTime<Utc>, DateTime<Utc>),
    ) -> Result<Vec<DetectedPattern>> {
        // TODO: 实现访问模式检测
        Ok(Vec::new())
    }

    async fn detect_query_patterns(
        &self,
        _data_window: &(DateTime<Utc>, DateTime<Utc>),
    ) -> Result<Vec<DetectedPattern>> {
        // TODO: 实现查询模式检测
        Ok(Vec::new())
    }

    async fn detect_topic_patterns(
        &self,
        _data_window: &(DateTime<Utc>, DateTime<Utc>),
    ) -> Result<Vec<DetectedPattern>> {
        // TODO: 实现主题模式检测
        Ok(Vec::new())
    }

    async fn detect_temporal_patterns(
        &self,
        _data_window: &(DateTime<Utc>, DateTime<Utc>),
    ) -> Result<Vec<DetectedPattern>> {
        // TODO: 实现时间模式检测
        Ok(Vec::new())
    }

    async fn detect_connection_patterns(
        &self,
        _data_window: &(DateTime<Utc>, DateTime<Utc>),
    ) -> Result<Vec<DetectedPattern>> {
        // TODO: 实现连接模式检测
        Ok(Vec::new())
    }

    async fn detect_feedback_patterns(
        &self,
        _data_window: &(DateTime<Utc>, DateTime<Utc>),
    ) -> Result<Vec<DetectedPattern>> {
        // TODO: 实现反馈模式检测
        Ok(Vec::new())
    }

    async fn store_pattern(&self, pattern: &DetectedPattern) -> Result<()> {
        let mut cache = self.pattern_cache.write().await;
        cache.insert(pattern.pattern_id.clone(), pattern.clone());
        Ok(())
    }
}

/// 反馈处理结果
#[derive(Debug, Clone)]
struct FeedbackProcessingResult {
    applied: bool,
    impact: f32,
}

impl FeedbackProcessor {
    fn new(_config: &Config) -> Self {
        Self {
            positive_threshold: 0.6,
            negative_threshold: 0.4,
            implicit_feedback_weight: 0.3,
            explicit_feedback_weight: 1.0,
            feedback_history: Arc::new(RwLock::new(VecDeque::new())),
        }
    }

    async fn process_feedback(
        &self,
        feedback: &FeedbackRecord,
    ) -> Result<FeedbackProcessingResult> {
        let weight = match feedback.feedback_type {
            FeedbackType::Explicit => self.explicit_feedback_weight,
            _ => self.implicit_feedback_weight,
        };

        let impact = feedback.score * weight;
        let applied = impact.abs() > 0.1; // 只有显著影响才应用

        if applied {
            // TODO: 实际应用反馈到记忆重要性或连接强度
        }

        // 记录到历史
        let mut history = self.feedback_history.write().await;
        history.push_back(feedback.clone());

        // 限制历史长度
        if history.len() > 10000 {
            history.drain(0..5000);
        }

        Ok(FeedbackProcessingResult { applied, impact })
    }

    async fn queue_feedback(&self, feedback: FeedbackRecord) -> Result<()> {
        let mut history = self.feedback_history.write().await;
        history.push_back(feedback);
        Ok(())
    }

    async fn get_pending_feedback(&self) -> Result<Vec<FeedbackRecord>> {
        let history = self.feedback_history.read().await;
        // 返回最近的隐式反馈进行批处理
        Ok(history
            .iter()
            .filter(|f| !matches!(f.feedback_type, FeedbackType::Explicit))
            .take(100)
            .cloned()
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::VectorGraphDB;
    use tempfile::NamedTempFile;

    async fn create_test_learning_engine() -> LearningEngine {
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
        LearningEngine::new(db, config).await.unwrap()
    }

    #[tokio::test]
    async fn test_importance_adjustment() {
        let engine = create_test_learning_engine().await;

        let task = LearningTask::ImportanceAdjustment {
            memory_ids: vec!["memory_1".to_string(), "memory_2".to_string()],
            trigger: ImportanceTrigger::TimeDecay,
        };

        let result = engine.execute_learning_task(task).await.unwrap();
        assert_eq!(result.task_type, "ImportanceAdjustment");
        assert!(result.success);
    }

    #[tokio::test]
    async fn test_pattern_detection() {
        let engine = create_test_learning_engine().await;

        let now = Utc::now();
        let task = LearningTask::PatternDetection {
            data_window: (now - Duration::days(1), now),
            pattern_types: vec![PatternType::AccessPattern, PatternType::QueryPattern],
        };

        let result = engine.execute_learning_task(task).await.unwrap();
        assert_eq!(result.task_type, "PatternDetection");
        assert!(result.success);
    }

    #[tokio::test]
    async fn test_feedback_processing() {
        let engine = create_test_learning_engine().await;

        let feedback = FeedbackRecord {
            memory_id: "memory_1".to_string(),
            feedback_type: FeedbackType::Explicit,
            score: 0.8,
            context: FeedbackContext {
                query: "测试查询".to_string(),
                result_position: 0,
                session_id: "session_1".to_string(),
                device_type: Some("desktop".to_string()),
                time_of_day: 14,
                day_of_week: 1,
            },
            timestamp: Utc::now(),
        };

        engine.record_feedback(feedback.clone()).await.unwrap();

        let task = LearningTask::FeedbackProcessing {
            feedback_batch: vec![feedback],
        };

        let result = engine.execute_learning_task(task).await.unwrap();
        assert_eq!(result.task_type, "FeedbackProcessing");
        assert!(result.success);
    }

    #[tokio::test]
    async fn test_learning_cycle() {
        let engine = create_test_learning_engine().await;

        let results = engine.run_learning_cycle().await.unwrap();
        assert!(!results.is_empty());

        // 检查统计信息是否更新
        let stats = engine.get_stats().await;
        assert!(stats.total_learning_cycles > 0);
    }
}

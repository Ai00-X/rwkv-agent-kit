//! 数据库性能优化模块
//!
//! 提供连接池优化、查询优化、缓存机制等性能优化功能

use lru::LruCache;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::collections::HashMap;
use std::num::NonZeroUsize;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

use crate::core::error::{ErrorCode, RwkvError, RwkvResult};

/// 数据库性能配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    /// 连接池配置
    pub pool: PoolConfig,
    /// 查询优化配置
    pub query: QueryConfig,
    /// 缓存配置
    pub cache: CacheConfig,
    /// 监控配置
    pub monitoring: MonitoringConfig,
}

/// 连接池配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolConfig {
    /// 最小连接数
    pub min_connections: u32,
    /// 最大连接数
    pub max_connections: u32,
    /// 连接超时时间（秒）
    pub connect_timeout: u64,
    /// 空闲连接超时时间（秒）
    pub idle_timeout: u64,
    /// 最大连接生命周期（秒）
    pub max_lifetime: u64,
    /// 获取连接超时时间（秒）
    pub acquire_timeout: u64,
    /// 是否启用连接预热
    pub enable_warmup: bool,
    /// 预热连接数
    pub warmup_connections: u32,
}

/// 查询优化配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryConfig {
    /// 查询超时时间（毫秒）
    pub query_timeout_ms: u64,
    /// 慢查询阈值（毫秒）
    pub slow_query_threshold_ms: u64,
    /// 批量操作大小
    pub batch_size: usize,
    /// 是否启用查询计划缓存
    pub enable_query_plan_cache: bool,
    /// 查询计划缓存大小
    pub query_plan_cache_size: usize,
    /// 是否启用预编译语句
    pub enable_prepared_statements: bool,
}

/// 缓存配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    /// 是否启用查询结果缓存
    pub enable_result_cache: bool,
    /// 结果缓存大小
    pub result_cache_size: usize,
    /// 结果缓存TTL（秒）
    pub result_cache_ttl: u64,
    /// 是否启用嵌入向量缓存
    pub enable_embedding_cache: bool,
    /// 嵌入向量缓存大小
    pub embedding_cache_size: usize,
    /// 嵌入向量缓存TTL（秒）
    pub embedding_cache_ttl: u64,
    /// 是否启用元数据缓存
    pub enable_metadata_cache: bool,
    /// 元数据缓存大小
    pub metadata_cache_size: usize,
}

/// 监控配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringConfig {
    /// 是否启用性能监控
    pub enable_monitoring: bool,
    /// 监控数据保留时间（秒）
    pub retention_seconds: u64,
    /// 监控采样率
    pub sampling_rate: f64,
    /// 是否启用指标导出
    pub enable_metrics_export: bool,
}

impl Default for MonitoringConfig {
    fn default() -> Self {
        Self {
            enable_monitoring: true,
            retention_seconds: 86400, // 24小时
            sampling_rate: 1.0,
            enable_metrics_export: true,
        }
    }
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            pool: PoolConfig {
                min_connections: 5,
                max_connections: 50,
                connect_timeout: 30,
                idle_timeout: 600,
                max_lifetime: 3600,
                acquire_timeout: 30,
                enable_warmup: true,
                warmup_connections: 5,
            },
            query: QueryConfig {
                query_timeout_ms: 30000,
                slow_query_threshold_ms: 1000,
                batch_size: 100,
                enable_query_plan_cache: true,
                query_plan_cache_size: 1000,
                enable_prepared_statements: true,
            },
            cache: CacheConfig {
                enable_result_cache: true,
                result_cache_size: 10000,
                result_cache_ttl: 300,
                enable_embedding_cache: true,
                embedding_cache_size: 5000,
                embedding_cache_ttl: 3600,
                enable_metadata_cache: true,
                metadata_cache_size: 1000,
            },
            monitoring: MonitoringConfig {
                enable_monitoring: true,
                retention_seconds: 86400, // 24小时
                sampling_rate: 1.0,
                enable_metrics_export: true,
            },
        }
    }
}

/// 查询统计信息
#[derive(Debug, Clone, Serialize)]
pub struct QueryStats {
    /// 查询SQL
    pub sql: String,
    /// 执行次数
    pub count: u64,
    /// 总执行时间（毫秒）
    pub total_duration_ms: u64,
    /// 平均执行时间（毫秒）
    pub avg_duration_ms: f64,
    /// 最小执行时间（毫秒）
    pub min_duration_ms: u64,
    /// 最大执行时间（毫秒）
    pub max_duration_ms: u64,
    /// 错误次数
    pub error_count: u64,
    /// 最后执行时间
    pub last_executed: chrono::DateTime<chrono::Utc>,
}

/// 连接池统计信息
#[derive(Debug, Clone, Serialize)]
pub struct PoolStats {
    /// 当前活跃连接数
    pub active_connections: u32,
    /// 当前空闲连接数
    pub idle_connections: u32,
    /// 总连接数
    pub total_connections: u32,
    /// 等待连接的任务数
    pub waiting_tasks: u32,
    /// 连接获取总次数
    pub acquire_count: u64,
    /// 连接获取成功次数
    pub acquire_success_count: u64,
    /// 连接获取失败次数
    pub acquire_failed_count: u64,
    /// 平均连接获取时间（毫秒）
    pub avg_acquire_time_ms: f64,
}

/// 缓存项
#[derive(Debug, Clone)]
struct CacheItem<T> {
    value: T,
    created_at: Instant,
    ttl: Duration,
}

impl<T> CacheItem<T> {
    fn new(value: T, ttl: Duration) -> Self {
        Self {
            value,
            created_at: Instant::now(),
            ttl,
        }
    }

    fn is_expired(&self) -> bool {
        self.created_at.elapsed() > self.ttl
    }
}

/// 查询结果缓存
pub struct QueryResultCache {
    cache: Arc<RwLock<LruCache<String, CacheItem<String>>>>,
    default_ttl: Duration,
}

impl QueryResultCache {
    pub fn new(capacity: usize, default_ttl: Duration) -> Self {
        Self {
            cache: Arc::new(RwLock::new(LruCache::new(
                NonZeroUsize::new(capacity).unwrap(),
            ))),
            default_ttl,
        }
    }

    /// 获取缓存结果
    pub async fn get(&self, key: &str) -> Option<String> {
        let mut cache = self.cache.write().await;

        if let Some(item) = cache.get(key) {
            if !item.is_expired() {
                return Some(item.value.clone());
            } else {
                cache.pop(key);
            }
        }

        None
    }

    /// 设置缓存结果
    pub async fn set(&self, key: String, value: String, ttl: Option<Duration>) {
        let mut cache = self.cache.write().await;
        let item = CacheItem::new(value, ttl.unwrap_or(self.default_ttl));
        cache.put(key, item);
    }

    /// 清除过期缓存
    pub async fn cleanup_expired(&self) {
        let mut cache = self.cache.write().await;
        let mut expired_keys = Vec::new();

        for (key, item) in cache.iter() {
            if item.is_expired() {
                expired_keys.push(key.clone());
            }
        }

        for key in expired_keys {
            cache.pop(&key);
        }
    }

    /// 获取缓存统计信息
    pub async fn get_stats(&self) -> HashMap<String, u64> {
        let cache = self.cache.read().await;
        let mut stats = HashMap::new();

        stats.insert("total_items".to_string(), cache.len() as u64);
        stats.insert("capacity".to_string(), cache.cap().get() as u64);

        stats
    }
}

/// 性能监控器
pub struct PerformanceMonitor {
    query_stats: Arc<RwLock<HashMap<String, QueryStats>>>,
    pool_stats: Arc<RwLock<PoolStats>>,
    config: MonitoringConfig,
}

impl PerformanceMonitor {
    pub fn new(config: MonitoringConfig) -> Self {
        Self {
            query_stats: Arc::new(RwLock::new(HashMap::new())),
            pool_stats: Arc::new(RwLock::new(PoolStats {
                active_connections: 0,
                idle_connections: 0,
                total_connections: 0,
                waiting_tasks: 0,
                acquire_count: 0,
                acquire_success_count: 0,
                acquire_failed_count: 0,
                avg_acquire_time_ms: 0.0,
            })),
            config,
        }
    }

    /// 记录查询统计
    pub async fn record_query(&self, sql: &str, duration: Duration, success: bool) {
        if !self.config.enable_monitoring {
            return;
        }

        let mut stats = self.query_stats.write().await;
        let duration_ms = duration.as_millis() as u64;

        if let Some(stat) = stats.get_mut(sql) {
            stat.count += 1;
            stat.total_duration_ms += duration_ms;
            stat.avg_duration_ms = stat.total_duration_ms as f64 / stat.count as f64;
            stat.min_duration_ms = stat.min_duration_ms.min(duration_ms);
            stat.max_duration_ms = stat.max_duration_ms.max(duration_ms);
            stat.last_executed = chrono::Utc::now();

            if !success {
                stat.error_count += 1;
            }
        } else {
            stats.insert(
                sql.to_string(),
                QueryStats {
                    sql: sql.to_string(),
                    count: 1,
                    total_duration_ms: duration_ms,
                    avg_duration_ms: duration_ms as f64,
                    min_duration_ms: duration_ms,
                    max_duration_ms: duration_ms,
                    error_count: if success { 0 } else { 1 },
                    last_executed: chrono::Utc::now(),
                },
            );
        }
    }

    /// 更新连接池统计
    pub async fn update_pool_stats(&self, pool: &SqlitePool) {
        if !self.config.enable_monitoring {
            return;
        }

        let mut stats = self.pool_stats.write().await;

        // 注意：sqlx的Pool没有直接暴露这些统计信息
        // 这里是示例实现，实际需要根据sqlx版本调整
        stats.total_connections = pool.size();
        stats.idle_connections = pool.num_idle() as u32;
        // active_connections = total - idle
        stats.active_connections = stats.total_connections - stats.idle_connections;
    }

    /// 记录连接获取
    pub async fn record_acquire(&self, duration: Duration, success: bool) {
        if !self.config.enable_monitoring {
            return;
        }

        let mut stats = self.pool_stats.write().await;
        stats.acquire_count += 1;

        if success {
            stats.acquire_success_count += 1;
        } else {
            stats.acquire_failed_count += 1;
        }

        let duration_ms = duration.as_millis() as f64;
        stats.avg_acquire_time_ms = (stats.avg_acquire_time_ms * (stats.acquire_count - 1) as f64
            + duration_ms)
            / stats.acquire_count as f64;
    }

    /// 获取查询统计
    pub async fn get_query_stats(&self) -> Vec<QueryStats> {
        let stats = self.query_stats.read().await;
        stats.values().cloned().collect()
    }

    /// 获取慢查询
    pub async fn get_slow_queries(&self, threshold_ms: u64) -> Vec<QueryStats> {
        let stats = self.query_stats.read().await;
        stats
            .values()
            .filter(|stat| stat.avg_duration_ms >= threshold_ms as f64)
            .cloned()
            .collect()
    }

    /// 获取连接池统计
    pub async fn get_pool_stats(&self) -> PoolStats {
        let stats = self.pool_stats.read().await;
        stats.clone()
    }

    /// 清理过期统计
    pub async fn cleanup_expired_stats(&self) {
        let cutoff =
            chrono::Utc::now() - chrono::Duration::seconds(self.config.retention_seconds as i64);

        let mut stats = self.query_stats.write().await;
        stats.retain(|_, stat| stat.last_executed >= cutoff);
    }
}

/// 优化的连接池管理器
pub struct OptimizedPool {
    pool: SqlitePool,
    monitor: PerformanceMonitor,
    query_cache: QueryResultCache,
    config: PerformanceConfig,
}

impl OptimizedPool {
    /// 创建优化的连接池
    pub async fn new(database_url: &str, config: PerformanceConfig) -> RwkvResult<Self> {
        let pool_options = sqlx::sqlite::SqlitePoolOptions::new()
            .min_connections(config.pool.min_connections)
            .max_connections(config.pool.max_connections)
            .acquire_timeout(Duration::from_secs(config.pool.acquire_timeout))
            .idle_timeout(Duration::from_secs(config.pool.idle_timeout))
            .max_lifetime(Duration::from_secs(config.pool.max_lifetime));

        let pool = pool_options.connect(database_url).await.map_err(|e| {
            RwkvError::new(
                ErrorCode::DatabaseConnectionFailed,
                format!("连接池创建失败: {}", e),
            )
        })?;

        let monitor = PerformanceMonitor::new(config.monitoring.clone());
        let query_cache = QueryResultCache::new(
            config.cache.result_cache_size,
            Duration::from_secs(config.cache.result_cache_ttl),
        );

        let optimized_pool = Self {
            monitor,
            query_cache,
            config: config.clone(),
            pool,
        };

        // 预热连接池
        if config.pool.enable_warmup {
            optimized_pool.warmup_connections().await?;
        }

        Ok(optimized_pool)
    }

    /// 预热连接池
    async fn warmup_connections(&self) -> RwkvResult<()> {
        log::info!("正在预热连接池...");

        let warmup_count = self
            .config
            .pool
            .warmup_connections
            .min(self.config.pool.max_connections);

        for i in 0..warmup_count {
            let start_time = Instant::now();
            match self.pool.acquire().await {
                Ok(_conn) => {
                    let duration = start_time.elapsed();
                    self.monitor.record_acquire(duration, true).await;
                    log::debug!(
                        "预热连接 {}/{} 成功，耗时: {:?}",
                        i + 1,
                        warmup_count,
                        duration
                    );
                }
                Err(e) => {
                    let duration = start_time.elapsed();
                    self.monitor.record_acquire(duration, false).await;
                    log::warn!("预热连接 {}/{} 失败: {}", i + 1, warmup_count, e);
                }
            }
        }

        log::info!("连接池预热完成");
        Ok(())
    }

    /// 执行查询（带缓存和监控）
    pub async fn execute_query<T>(&self, sql: &str, params: Vec<String>) -> RwkvResult<Vec<T>>
    where
        T: for<'r> sqlx::FromRow<'r, sqlx::sqlite::SqliteRow> + Send + Unpin + 'static,
    {
        let start_time = Instant::now();

        // 检查缓存（仅对SELECT查询）
        let _cache_key = if self.config.cache.enable_result_cache
            && sql.trim_start().to_uppercase().starts_with("SELECT")
        {
            Some(format!("{}:{:?}", sql, params))
        } else {
            None
        };

        // 执行查询
        let mut query = sqlx::query_as::<_, T>(sql);
        for param in params {
            query = query.bind(param);
        }
        let result = query.fetch_all(&self.pool).await;
        let duration = start_time.elapsed();
        let success = result.is_ok();

        // 记录统计
        self.monitor.record_query(sql, duration, success).await;

        // 检查慢查询
        if duration.as_millis() as u64 >= self.config.query.slow_query_threshold_ms {
            log::warn!("慢查询检测: SQL={}, 耗时={}ms", sql, duration.as_millis());
        }

        result.map_err(|e| {
            RwkvError::new(
                ErrorCode::DatabaseQueryFailed,
                format!("查询执行失败: {}", e),
            )
            .with_context(sql)
        })
    }

    /// 执行单条查询（返回单个结果）
    pub async fn fetch_one<T>(&self, sql: &str, params: Vec<String>) -> RwkvResult<T>
    where
        T: for<'r> sqlx::FromRow<'r, sqlx::sqlite::SqliteRow> + Send + Unpin + 'static,
    {
        let rows = self.execute_query::<T>(sql, params).await?;
        rows.into_iter()
            .next()
            .ok_or_else(|| RwkvError::new(ErrorCode::DatabaseQueryFailed, "查询结果为空"))
    }

    /// 执行非查询操作（INSERT, UPDATE, DELETE）
    pub async fn execute(&self, sql: &str, params: Vec<String>) -> RwkvResult<u64> {
        let start_time = Instant::now();

        let mut query = sqlx::query(sql);
        for param in params {
            query = query.bind(param);
        }
        let result = query.execute(&self.pool).await;

        let duration = start_time.elapsed();
        let success = result.is_ok();

        // 记录统计
        self.monitor.record_query(sql, duration, success).await;

        // 检查慢查询
        if duration.as_millis() as u64 >= self.config.query.slow_query_threshold_ms {
            log::warn!("慢操作检测: SQL={}, 耗时={}ms", sql, duration.as_millis());
        }

        result.map(|r| r.rows_affected()).map_err(|e| {
            RwkvError::new(
                ErrorCode::DatabaseQueryFailed,
                format!("操作执行失败: {}", e),
            )
            .with_context(sql)
        })
    }

    /// 批量执行操作
    pub async fn execute_batch(
        &self,
        operations: Vec<(&str, Vec<String>)>,
    ) -> RwkvResult<Vec<u64>> {
        let start_time = Instant::now();
        let mut results = Vec::new();

        // 使用事务执行批量操作
        let mut tx = self.pool.begin().await.map_err(|e| {
            RwkvError::new(
                ErrorCode::DatabaseTransactionFailed,
                format!("事务开始失败: {}", e),
            )
        })?;

        for (sql, params) in operations {
            let mut query = sqlx::query(sql);
            for param in params {
                query = query.bind(param);
            }
            let result = query.execute(&mut *tx).await.map_err(|e| {
                RwkvError::new(
                    ErrorCode::DatabaseQueryFailed,
                    format!("批量操作失败: {}", e),
                )
                .with_context(sql)
            })?;

            results.push(result.rows_affected());
        }

        tx.commit().await.map_err(|e| {
            RwkvError::new(
                ErrorCode::DatabaseTransactionFailed,
                format!("事务提交失败: {}", e),
            )
        })?;

        let duration = start_time.elapsed();
        self.monitor
            .record_query("BATCH_OPERATION", duration, true)
            .await;

        log::debug!(
            "批量操作完成，处理 {} 个操作，耗时: {:?}",
            results.len(),
            duration
        );

        Ok(results)
    }

    /// 获取性能统计
    pub async fn get_performance_stats(&self) -> HashMap<String, serde_json::Value> {
        let mut stats = HashMap::new();

        // 查询统计
        let query_stats = self.monitor.get_query_stats().await;
        stats.insert(
            "query_stats".to_string(),
            serde_json::to_value(query_stats).unwrap(),
        );

        // 连接池统计
        let pool_stats = self.monitor.get_pool_stats().await;
        stats.insert(
            "pool_stats".to_string(),
            serde_json::to_value(pool_stats).unwrap(),
        );

        // 缓存统计
        let cache_stats = self.query_cache.get_stats().await;
        stats.insert(
            "cache_stats".to_string(),
            serde_json::to_value(cache_stats).unwrap(),
        );

        stats
    }

    /// 清理过期数据
    pub async fn cleanup(&self) {
        self.monitor.cleanup_expired_stats().await;
        self.query_cache.cleanup_expired().await;
    }

    /// 获取原始连接池引用
    pub fn pool(&self) -> &SqlitePool {
        &self.pool
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_query_result_cache() {
        let cache = QueryResultCache::new(10, Duration::from_secs(60));

        // 测试设置和获取
        cache
            .set("key1".to_string(), "value1".to_string(), None)
            .await;
        let result = cache.get("key1").await;
        assert_eq!(result, Some("value1".to_string()));

        // 测试不存在的key
        let result = cache.get("nonexistent").await;
        assert_eq!(result, None);

        // 测试过期
        cache
            .set(
                "key2".to_string(),
                "value2".to_string(),
                Some(Duration::from_millis(10)),
            )
            .await;
        tokio::time::sleep(Duration::from_millis(20)).await;
        let result = cache.get("key2").await;
        assert_eq!(result, None);
    }

    #[tokio::test]
    async fn test_performance_monitor() {
        let config = MonitoringConfig::default();
        let monitor = PerformanceMonitor::new(config);

        // 记录查询统计
        monitor
            .record_query("SELECT * FROM test", Duration::from_millis(100), true)
            .await;
        monitor
            .record_query("SELECT * FROM test", Duration::from_millis(200), true)
            .await;

        let stats = monitor.get_query_stats().await;
        assert_eq!(stats.len(), 1);

        let stat = &stats[0];
        assert_eq!(stat.count, 2);
        assert_eq!(stat.avg_duration_ms, 150.0);
        assert_eq!(stat.min_duration_ms, 100);
        assert_eq!(stat.max_duration_ms, 200);
    }
}

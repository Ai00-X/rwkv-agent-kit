//! 错误处理中间件
//!
//! 提供统一的错误处理、日志记录、恢复策略和监控功能

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

use super::error::{ErrorCategory, ErrorCode, ErrorSeverity, RwkvError, RwkvResult};

/// 错误处理配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorHandlerConfig {
    /// 是否启用错误恢复
    pub enable_recovery: bool,
    /// 最大重试次数
    pub max_retries: u32,
    /// 重试延迟（毫秒）
    pub retry_delay_ms: u64,
    /// 是否启用指数退避
    pub exponential_backoff: bool,
    /// 退避倍数
    pub backoff_multiplier: f64,
    /// 最大重试延迟（毫秒）
    pub max_retry_delay_ms: u64,
    /// 是否启用断路器
    pub enable_circuit_breaker: bool,
    /// 断路器失败阈值
    pub circuit_breaker_threshold: u32,
    /// 断路器恢复时间（秒）
    pub circuit_breaker_recovery_time: u64,
    /// 是否启用错误聚合
    pub enable_error_aggregation: bool,
    /// 错误聚合窗口（秒）
    pub aggregation_window_seconds: u64,
}

impl Default for ErrorHandlerConfig {
    fn default() -> Self {
        Self {
            enable_recovery: true,
            max_retries: 3,
            retry_delay_ms: 1000,
            exponential_backoff: true,
            backoff_multiplier: 2.0,
            max_retry_delay_ms: 30000,
            enable_circuit_breaker: true,
            circuit_breaker_threshold: 5,
            circuit_breaker_recovery_time: 60,
            enable_error_aggregation: true,
            aggregation_window_seconds: 300, // 5分钟
        }
    }
}

/// 断路器状态
#[derive(Debug, Clone, PartialEq)]
pub enum CircuitState {
    /// 关闭状态 - 正常工作
    Closed,
    /// 打开状态 - 暂停工作
    Open,
    /// 半开状态 - 测试恢复
    HalfOpen,
}

/// 断路器
#[derive(Debug)]
pub struct CircuitBreaker {
    /// 当前状态
    state: CircuitState,
    /// 失败计数
    failure_count: u32,
    /// 失败阈值
    threshold: u32,
    /// 上次失败时间
    last_failure_time: Option<Instant>,
    /// 恢复超时
    recovery_timeout: Duration,
}

impl CircuitBreaker {
    pub fn new(threshold: u32, recovery_timeout: Duration) -> Self {
        Self {
            state: CircuitState::Closed,
            failure_count: 0,
            threshold,
            last_failure_time: None,
            recovery_timeout,
        }
    }

    /// 检查是否可以执行操作
    pub fn can_execute(&mut self) -> bool {
        match self.state {
            CircuitState::Closed => true,
            CircuitState::Open => {
                if let Some(last_failure) = self.last_failure_time {
                    if last_failure.elapsed() >= self.recovery_timeout {
                        self.state = CircuitState::HalfOpen;
                        true
                    } else {
                        false
                    }
                } else {
                    true
                }
            }
            CircuitState::HalfOpen => true,
        }
    }

    /// 记录成功
    pub fn record_success(&mut self) {
        self.failure_count = 0;
        self.state = CircuitState::Closed;
        self.last_failure_time = None;
    }

    /// 记录失败
    pub fn record_failure(&mut self) {
        self.failure_count += 1;
        self.last_failure_time = Some(Instant::now());

        if self.failure_count >= self.threshold {
            self.state = CircuitState::Open;
        }
    }

    pub fn state(&self) -> &CircuitState {
        &self.state
    }
}

/// 错误统计信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorStats {
    /// 错误代码
    pub error_code: ErrorCode,
    /// 发生次数
    pub count: u64,
    /// 第一次发生时间
    pub first_occurrence: DateTime<Utc>,
    /// 最后一次发生时间
    pub last_occurrence: DateTime<Utc>,
    /// 平均间隔时间（秒）
    pub average_interval: f64,
}

/// 错误聚合器
#[derive(Debug)]
pub struct ErrorAggregator {
    /// 错误统计
    error_stats: HashMap<ErrorCode, ErrorStats>,
    /// 聚合窗口
    window_duration: Duration,
    /// 上次清理时间
    last_cleanup: Instant,
}

impl ErrorAggregator {
    pub fn new(window_duration: Duration) -> Self {
        Self {
            error_stats: HashMap::new(),
            window_duration,
            last_cleanup: Instant::now(),
        }
    }

    /// 记录错误
    pub fn record_error(&mut self, error: &RwkvError) {
        let now = Utc::now();

        if let Some(stats) = self.error_stats.get_mut(&error.code) {
            let interval = (now - stats.last_occurrence).num_seconds() as f64;
            stats.average_interval =
                (stats.average_interval * stats.count as f64 + interval) / (stats.count + 1) as f64;
            stats.count += 1;
            stats.last_occurrence = now;
        } else {
            self.error_stats.insert(
                error.code,
                ErrorStats {
                    error_code: error.code,
                    count: 1,
                    first_occurrence: now,
                    last_occurrence: now,
                    average_interval: 0.0,
                },
            );
        }

        // 定期清理过期统计
        if self.last_cleanup.elapsed() >= self.window_duration {
            self.cleanup_expired_stats();
        }
    }

    /// 清理过期统计
    fn cleanup_expired_stats(&mut self) {
        let cutoff = Utc::now() - chrono::Duration::from_std(self.window_duration).unwrap();
        self.error_stats
            .retain(|_, stats| stats.last_occurrence >= cutoff);
        self.last_cleanup = Instant::now();
    }

    /// 获取错误统计
    pub fn get_stats(&self) -> Vec<ErrorStats> {
        self.error_stats.values().cloned().collect()
    }

    /// 获取高频错误
    pub fn get_frequent_errors(&self, threshold: u64) -> Vec<ErrorStats> {
        self.error_stats
            .values()
            .filter(|stats| stats.count >= threshold)
            .cloned()
            .collect()
    }
}

/// 错误处理器
#[derive(Debug)]
pub struct ErrorHandler {
    /// 配置
    config: ErrorHandlerConfig,
    /// 断路器映射（按错误类别）
    circuit_breakers: Arc<RwLock<HashMap<ErrorCategory, CircuitBreaker>>>,
    /// 错误聚合器
    error_aggregator: Arc<RwLock<ErrorAggregator>>,
}

impl Default for ErrorHandler {
    fn default() -> Self {
        Self::new(ErrorHandlerConfig::default())
    }
}

impl ErrorHandler {
    /// 创建新的错误处理器
    pub fn new(config: ErrorHandlerConfig) -> Self {
        let aggregator =
            ErrorAggregator::new(Duration::from_secs(config.aggregation_window_seconds));

        Self {
            config,
            circuit_breakers: Arc::new(RwLock::new(HashMap::new())),
            error_aggregator: Arc::new(RwLock::new(aggregator)),
        }
    }

    /// 处理错误
    pub async fn handle_error(&self, error: &RwkvError) -> RwkvResult<()> {
        // 记录错误到日志
        self.log_error(error).await;

        // 记录错误到聚合器
        if self.config.enable_error_aggregation {
            let mut aggregator = self.error_aggregator.write().await;
            aggregator.record_error(error);
        }

        // 更新断路器状态
        if self.config.enable_circuit_breaker {
            self.update_circuit_breaker(error).await;
        }

        Ok(())
    }

    /// 带重试的执行函数
    pub async fn execute_with_retry<F, Fut, T>(&self, operation: F, context: &str) -> RwkvResult<T>
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = RwkvResult<T>>,
    {
        if !self.config.enable_recovery {
            return operation().await;
        }

        let mut attempts = 0;
        let mut delay = Duration::from_millis(self.config.retry_delay_ms);

        loop {
            // 检查断路器
            if self.config.enable_circuit_breaker && !self.can_execute_operation().await {
                return Err(RwkvError::new(
                    ErrorCode::InternalTimeout,
                    "断路器开启，操作被阻止",
                )
                .with_context(context));
            }

            match operation().await {
                Ok(result) => {
                    // 操作成功，重置断路器
                    if self.config.enable_circuit_breaker && attempts > 0 {
                        self.record_circuit_breaker_success().await;
                    }
                    return Ok(result);
                }
                Err(error) => {
                    attempts += 1;

                    // 记录错误
                    self.handle_error(&error).await.ok();

                    // 检查是否可以重试
                    if !self.should_retry(&error, attempts) {
                        return Err(error);
                    }

                    // 等待后重试
                    log::warn!(
                        "操作失败，{}ms后重试 (第{}/{}次): {}",
                        delay.as_millis(),
                        attempts,
                        self.config.max_retries,
                        error
                    );

                    tokio::time::sleep(delay).await;

                    // 计算下次延迟
                    if self.config.exponential_backoff {
                        delay = Duration::from_millis(
                            ((delay.as_millis() as f64 * self.config.backoff_multiplier) as u64)
                                .min(self.config.max_retry_delay_ms),
                        );
                    }
                }
            }
        }
    }

    /// 检查是否应该重试
    fn should_retry(&self, error: &RwkvError, attempts: u32) -> bool {
        if attempts >= self.config.max_retries {
            return false;
        }

        // 某些错误不应该重试
        match error.code {
            ErrorCode::UserInputTooLong
            | ErrorCode::UserInputInvalidFormat
            | ErrorCode::UserInputContainsForbidden
            | ErrorCode::UserInputEmpty
            | ErrorCode::ConfigParseError
            | ErrorCode::ConfigValidationError => false,

            // 系统资源错误可能在稍后恢复
            ErrorCode::SystemOutOfMemory
            | ErrorCode::SystemDiskFull
            | ErrorCode::SystemResourceExhausted => true,

            // 网络和数据库错误通常可以重试
            ErrorCode::NetworkTimeout
            | ErrorCode::NetworkConnectionFailed
            | ErrorCode::DatabaseTimeout
            | ErrorCode::DatabaseConnectionFailed => true,

            // 其他错误根据严重程度判断
            _ => error.is_recoverable(),
        }
    }

    /// 检查是否可以执行操作
    async fn can_execute_operation(&self) -> bool {
        // 这里可以实现更复杂的逻辑，检查各种断路器状态
        true
    }

    /// 记录断路器成功
    async fn record_circuit_breaker_success(&self) {
        // 实现断路器成功记录逻辑
    }

    /// 更新断路器状态
    async fn update_circuit_breaker(&self, error: &RwkvError) {
        let category = error.category();
        let mut breakers = self.circuit_breakers.write().await;

        let breaker = breakers.entry(category).or_insert_with(|| {
            CircuitBreaker::new(
                self.config.circuit_breaker_threshold,
                Duration::from_secs(self.config.circuit_breaker_recovery_time),
            )
        });

        breaker.record_failure();

        if breaker.state() == &CircuitState::Open {
            log::warn!("断路器打开，类别: {:?}", category);
        }
    }

    /// 记录错误到日志
    async fn log_error(&self, error: &RwkvError) {
        match error.severity() {
            ErrorSeverity::Low => {
                log::debug!("错误 [{}]: {}", error.code as u32, error.message);
            }
            ErrorSeverity::Medium => {
                log::info!("错误 [{}]: {}", error.code as u32, error.message);
            }
            ErrorSeverity::High => {
                log::warn!("错误 [{}]: {}", error.code as u32, error.message);
            }
            ErrorSeverity::Critical => {
                log::error!("严重错误 [{}]: {}", error.code as u32, error.message);

                // 对于严重错误，记录完整的错误信息
                if let Some(context) = &error.context {
                    log::error!("错误上下文: {}", context);
                }
                if let Some(source) = &error.source {
                    log::error!("错误源: {}", source);
                }
                if let Some(trace_id) = &error.trace_id {
                    log::error!("追踪ID: {}", trace_id);
                }
            }
        }
    }

    /// 获取错误统计
    pub async fn get_error_stats(&self) -> Vec<ErrorStats> {
        let aggregator = self.error_aggregator.read().await;
        aggregator.get_stats()
    }

    /// 获取高频错误
    pub async fn get_frequent_errors(&self, threshold: u64) -> Vec<ErrorStats> {
        let aggregator = self.error_aggregator.read().await;
        aggregator.get_frequent_errors(threshold)
    }

    /// 获取断路器状态
    pub async fn get_circuit_breaker_status(&self) -> HashMap<ErrorCategory, CircuitState> {
        let breakers = self.circuit_breakers.read().await;
        breakers
            .iter()
            .map(|(category, breaker)| (*category, breaker.state().clone()))
            .collect()
    }

    /// 重置断路器
    pub async fn reset_circuit_breaker(&self, category: ErrorCategory) -> RwkvResult<()> {
        let mut breakers = self.circuit_breakers.write().await;
        if let Some(breaker) = breakers.get_mut(&category) {
            breaker.record_success();
            log::info!("断路器已重置，类别: {:?}", category);
        }
        Ok(())
    }

    /// 重置所有断路器
    pub async fn reset_all_circuit_breakers(&self) -> RwkvResult<()> {
        let mut breakers = self.circuit_breakers.write().await;
        for (category, breaker) in breakers.iter_mut() {
            breaker.record_success();
            log::info!("断路器已重置，类别: {:?}", category);
        }
        Ok(())
    }
}

/// 错误处理宏
#[macro_export]
macro_rules! handle_error_with_retry {
    ($handler:expr, $operation:expr, $context:expr) => {
        $handler
            .execute_with_retry(|| async { $operation }, $context)
            .await
    };
}

/// 全局错误处理器
static mut GLOBAL_ERROR_HANDLER: Option<Arc<ErrorHandler>> = None;
static INIT: std::sync::Once = std::sync::Once::new();

/// 初始化全局错误处理器
pub fn initialize_global_error_handler(config: ErrorHandlerConfig) {
    INIT.call_once(|| {
        let handler = Arc::new(ErrorHandler::new(config));
        unsafe {
            GLOBAL_ERROR_HANDLER = Some(handler);
        }
    });
}

/// 获取全局错误处理器
pub fn get_global_error_handler() -> Option<Arc<ErrorHandler>> {
    #[allow(static_mut_refs)] // 全局错误处理器需要静态可变访问
    unsafe {
        GLOBAL_ERROR_HANDLER.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::{sleep, Duration};

    #[tokio::test]
    async fn test_circuit_breaker() {
        let mut breaker = CircuitBreaker::new(3, Duration::from_secs(1));

        // 初始状态应该是关闭的
        assert_eq!(breaker.state(), &CircuitState::Closed);
        assert!(breaker.can_execute());

        // 记录失败
        breaker.record_failure();
        breaker.record_failure();
        assert_eq!(breaker.state(), &CircuitState::Closed);

        breaker.record_failure();
        assert_eq!(breaker.state(), &CircuitState::Open);
        assert!(!breaker.can_execute());

        // 等待恢复时间
        sleep(Duration::from_secs(2)).await;
        assert!(breaker.can_execute());
        assert_eq!(breaker.state(), &CircuitState::HalfOpen);

        // 记录成功
        breaker.record_success();
        assert_eq!(breaker.state(), &CircuitState::Closed);
    }

    #[tokio::test]
    async fn test_error_aggregator() {
        let mut aggregator = ErrorAggregator::new(Duration::from_secs(60));

        let error1 = RwkvError::new(ErrorCode::DatabaseTimeout, "超时1");
        let error2 = RwkvError::new(ErrorCode::DatabaseTimeout, "超时2");
        let error3 = RwkvError::new(ErrorCode::ModelLoadFailed, "加载失败");

        aggregator.record_error(&error1);
        aggregator.record_error(&error2);
        aggregator.record_error(&error3);

        let stats = aggregator.get_stats();
        assert_eq!(stats.len(), 2);

        let db_stats = stats
            .iter()
            .find(|s| s.error_code == ErrorCode::DatabaseTimeout)
            .unwrap();
        assert_eq!(db_stats.count, 2);

        let model_stats = stats
            .iter()
            .find(|s| s.error_code == ErrorCode::ModelLoadFailed)
            .unwrap();
        assert_eq!(model_stats.count, 1);
    }

    #[tokio::test]
    async fn test_error_handler_retry() {
        let config = ErrorHandlerConfig {
            max_retries: 2,
            retry_delay_ms: 10,
            ..Default::default()
        };

        let handler = ErrorHandler::new(config);

        let result = handler
            .execute_with_retry(
                || {
                    static ATTEMPT_COUNT: std::sync::atomic::AtomicUsize =
                        std::sync::atomic::AtomicUsize::new(0);
                    let current_attempt =
                        ATTEMPT_COUNT.fetch_add(1, std::sync::atomic::Ordering::SeqCst) + 1;
                    async move {
                        if current_attempt < 3 {
                            Err(RwkvError::new(ErrorCode::DatabaseTimeout, "临时失败"))
                        } else {
                            Ok("成功")
                        }
                    }
                },
                "测试操作",
            )
            .await;

        assert!(result.is_ok());
    }
}

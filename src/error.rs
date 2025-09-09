//! 错误处理模块
//!
//! 定义了RWKV-Agent-Kit库中使用的所有错误类型和结果类型。

use thiserror::Error;

/// RWKV-Agent-Kit库的统一结果类型
pub type Result<T> = std::result::Result<T, MemoryError>;

/// RWKV-Agent-Kit库的错误类型
#[derive(Error, Debug)]
pub enum MemoryError {
    /// 数据库相关错误
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    /// 序列化/反序列化错误
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// IO错误
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// 配置错误
    #[error("Configuration error: {0}")]
    Config(#[from] config::ConfigError),

    /// 记忆不存在错误
    #[error("Memory with id '{id}' not found")]
    MemoryNotFound { id: String },

    /// 无效的向量维度错误
    #[error("Invalid vector dimension: expected {expected}, got {actual}")]
    InvalidVectorDimension { expected: usize, actual: usize },

    /// 无效的查询参数错误
    #[error("Invalid query parameter: {message}")]
    InvalidQuery { message: String },

    /// 连接创建失败错误
    #[error("Failed to create connection between memories '{from}' and '{to}': {reason}")]
    ConnectionFailed {
        from: String,
        to: String,
        reason: String,
    },

    /// 记忆演化错误
    #[error("Memory evolution failed: {message}")]
    EvolutionFailed { message: String },

    /// 学习模块错误
    #[error("Learning module error: {message}")]
    LearningError { message: String },

    /// 检索引擎错误
    #[error("Retrieval engine error: {message}")]
    RetrievalError { message: String },

    /// 向量操作错误
    #[error("Vector operation error: {message}")]
    VectorError { message: String },

    /// 图操作错误
    #[error("Graph operation error: {message}")]
    GraphError { message: String },

    /// 缓存错误
    #[error("Cache error: {message}")]
    CacheError { message: String },

    /// 并发错误
    #[error("Concurrency error: {message}")]
    ConcurrencyError { message: String },

    /// 资源不足错误
    #[error("Insufficient resources: {message}")]
    InsufficientResources { message: String },

    /// 超时错误
    #[error("Operation timed out: {operation}")]
    Timeout { operation: String },

    /// 验证错误
    #[error("Validation error: {message}")]
    ValidationError { message: String },

    /// 权限错误
    #[error("Permission denied: {message}")]
    PermissionDenied { message: String },

    /// 内部错误
    #[error("Internal error: {message}")]
    Internal { message: String },

    /// 其他错误
    #[error("Other error: {0}")]
    Other(#[from] anyhow::Error),
}

impl MemoryError {
    /// 创建记忆不存在错误
    pub fn memory_not_found(id: impl Into<String>) -> Self {
        Self::MemoryNotFound { id: id.into() }
    }

    /// 创建无效向量维度错误
    pub fn invalid_vector_dimension(expected: usize, actual: usize) -> Self {
        Self::InvalidVectorDimension { expected, actual }
    }

    /// 创建无效查询错误
    pub fn invalid_query(message: impl Into<String>) -> Self {
        Self::InvalidQuery {
            message: message.into(),
        }
    }

    /// 创建连接失败错误
    pub fn connection_failed(
        from: impl Into<String>,
        to: impl Into<String>,
        reason: impl Into<String>,
    ) -> Self {
        Self::ConnectionFailed {
            from: from.into(),
            to: to.into(),
            reason: reason.into(),
        }
    }

    /// 创建演化失败错误
    pub fn evolution_failed(message: impl Into<String>) -> Self {
        Self::EvolutionFailed {
            message: message.into(),
        }
    }

    /// 创建学习错误
    pub fn learning_error(message: impl Into<String>) -> Self {
        Self::LearningError {
            message: message.into(),
        }
    }

    /// 创建检索错误
    pub fn retrieval_error(message: impl Into<String>) -> Self {
        Self::RetrievalError {
            message: message.into(),
        }
    }

    /// 创建向量错误
    pub fn vector_error(message: impl Into<String>) -> Self {
        Self::VectorError {
            message: message.into(),
        }
    }

    /// 创建图错误
    pub fn graph_error(message: impl Into<String>) -> Self {
        Self::GraphError {
            message: message.into(),
        }
    }

    /// 创建缓存错误
    pub fn cache_error(message: impl Into<String>) -> Self {
        Self::CacheError {
            message: message.into(),
        }
    }

    /// 创建并发错误
    pub fn concurrency_error(message: impl Into<String>) -> Self {
        Self::ConcurrencyError {
            message: message.into(),
        }
    }

    /// 创建资源不足错误
    pub fn insufficient_resources(message: impl Into<String>) -> Self {
        Self::InsufficientResources {
            message: message.into(),
        }
    }

    /// 创建超时错误
    pub fn timeout(operation: impl Into<String>) -> Self {
        Self::Timeout {
            operation: operation.into(),
        }
    }

    /// 创建验证错误
    pub fn validation_error(message: impl Into<String>) -> Self {
        Self::ValidationError {
            message: message.into(),
        }
    }

    /// 创建权限错误
    pub fn permission_denied(message: impl Into<String>) -> Self {
        Self::PermissionDenied {
            message: message.into(),
        }
    }

    /// 创建内部错误
    pub fn internal(message: impl Into<String>) -> Self {
        Self::Internal {
            message: message.into(),
        }
    }

    /// 检查是否为数据库错误
    pub fn is_database_error(&self) -> bool {
        matches!(self, Self::Database(_))
    }

    /// 检查是否为序列化错误
    pub fn is_serialization_error(&self) -> bool {
        matches!(self, Self::Serialization(_))
    }

    /// 检查是否为记忆不存在错误
    pub fn is_memory_not_found(&self) -> bool {
        matches!(self, Self::MemoryNotFound { .. })
    }

    /// 检查是否为超时错误
    pub fn is_timeout(&self) -> bool {
        matches!(self, Self::Timeout { .. })
    }

    /// 获取错误的严重程度
    pub fn severity(&self) -> ErrorSeverity {
        match self {
            Self::Database(_) | Self::Io(_) => ErrorSeverity::Critical,
            Self::MemoryNotFound { .. }
            | Self::InvalidQuery { .. }
            | Self::ValidationError { .. } => ErrorSeverity::Warning,
            Self::Timeout { .. } | Self::InsufficientResources { .. } => ErrorSeverity::Error,
            Self::Internal { .. } => ErrorSeverity::Critical,
            _ => ErrorSeverity::Error,
        }
    }
}

/// 错误严重程度
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorSeverity {
    /// 信息级别
    Info,
    /// 警告级别
    Warning,
    /// 错误级别
    Error,
    /// 严重错误级别
    Critical,
}

impl std::fmt::Display for ErrorSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Info => write!(f, "INFO"),
            Self::Warning => write!(f, "WARNING"),
            Self::Error => write!(f, "ERROR"),
            Self::Critical => write!(f, "CRITICAL"),
        }
    }
}

/// 错误上下文信息
#[derive(Debug, Clone)]
pub struct ErrorContext {
    pub operation: String,
    pub component: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub additional_info: std::collections::HashMap<String, String>,
}

impl ErrorContext {
    /// 创建新的错误上下文
    pub fn new(operation: impl Into<String>, component: impl Into<String>) -> Self {
        Self {
            operation: operation.into(),
            component: component.into(),
            timestamp: chrono::Utc::now(),
            additional_info: std::collections::HashMap::new(),
        }
    }

    /// 添加额外信息
    pub fn with_info(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.additional_info.insert(key.into(), value.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let error = MemoryError::memory_not_found("test_id");
        assert!(error.is_memory_not_found());
        assert_eq!(error.severity(), ErrorSeverity::Warning);
    }

    #[test]
    fn test_error_context() {
        let context = ErrorContext::new("test_operation", "test_component")
            .with_info("key1", "value1")
            .with_info("key2", "value2");

        assert_eq!(context.operation, "test_operation");
        assert_eq!(context.component, "test_component");
        assert_eq!(context.additional_info.len(), 2);
    }

    #[test]
    fn test_error_severity() {
        assert_eq!(ErrorSeverity::Critical.to_string(), "CRITICAL");
        assert_eq!(ErrorSeverity::Error.to_string(), "ERROR");
        assert_eq!(ErrorSeverity::Warning.to_string(), "WARNING");
        assert_eq!(ErrorSeverity::Info.to_string(), "INFO");
    }
}

//! 错误处理模块
//!
//! 提供统一的错误类型定义、错误代码映射和错误处理工具

use serde::{Deserialize, Serialize};
use std::fmt;

/// 错误严重级别
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ErrorSeverity {
    /// 低级别 - 可恢复的错误
    Low,
    /// 中级别 - 需要注意的错误
    Medium,
    /// 高级别 - 严重错误
    High,
    /// 致命级别 - 系统无法继续运行
    Critical,
}

/// 错误类别
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ErrorCategory {
    /// 配置错误
    Configuration,
    /// 数据库错误
    Database,
    /// 模型错误
    Model,
    /// 智能体错误
    Agent,
    /// 网络错误
    Network,
    /// 系统错误
    System,
    /// 用户输入错误
    UserInput,
    /// 内部错误
    Internal,
}

/// 标准化错误代码
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ErrorCode {
    // 配置错误 (1000-1099)
    ConfigFileNotFound = 1001,
    ConfigParseError = 1002,
    ConfigValidationError = 1003,
    ModelPathNotFound = 1004,
    TokenizerNotFound = 1005,
    StateFileNotFound = 1006,

    // 数据库错误 (2000-2099)
    DatabaseConnectionFailed = 2001,
    DatabaseQueryFailed = 2002,
    DatabaseTransactionFailed = 2003,
    DatabaseMigrationFailed = 2004,
    DatabaseTimeout = 2005,
    DatabaseCorrupted = 2006,
    DatabasePermissionDenied = 2007,

    // 模型错误 (3000-3099)
    ModelLoadFailed = 3001,
    ModelInferenceFailed = 3002,
    ModelOutOfMemory = 3003,
    ModelTokenizerError = 3004,
    ModelStateError = 3005,
    ModelQuantizationError = 3006,

    // 智能体错误 (4000-4099)
    AgentNotFound = 4001,
    AgentRegistrationFailed = 4002,
    AgentConfigurationError = 4003,
    AgentPromptError = 4004,
    AgentMemoryError = 4005,
    AgentToolError = 4006,

    // 网络错误 (5000-5099)
    NetworkTimeout = 5001,
    NetworkConnectionFailed = 5002,
    NetworkRequestFailed = 5003,
    NetworkAuthenticationFailed = 5004,

    // 系统错误 (6000-6099)
    SystemOutOfMemory = 6001,
    SystemDiskFull = 6002,
    SystemPermissionDenied = 6003,
    SystemResourceExhausted = 6004,
    SystemThreadPanic = 6005,

    // 用户输入错误 (7000-7099)
    UserInputTooLong = 7001,
    UserInputInvalidFormat = 7002,
    UserInputContainsForbidden = 7003,
    UserInputEmpty = 7004,

    // 内部错误 (9000-9099)
    InternalUnknownError = 9001,
    InternalLogicError = 9002,
    InternalDataCorruption = 9003,
    InternalTimeout = 9004,
}

impl ErrorCode {
    /// 获取错误代码对应的错误类别
    pub fn category(&self) -> ErrorCategory {
        match self {
            ErrorCode::ConfigFileNotFound
            | ErrorCode::ConfigParseError
            | ErrorCode::ConfigValidationError
            | ErrorCode::ModelPathNotFound
            | ErrorCode::TokenizerNotFound
            | ErrorCode::StateFileNotFound => ErrorCategory::Configuration,

            ErrorCode::DatabaseConnectionFailed
            | ErrorCode::DatabaseQueryFailed
            | ErrorCode::DatabaseTransactionFailed
            | ErrorCode::DatabaseMigrationFailed
            | ErrorCode::DatabaseTimeout
            | ErrorCode::DatabaseCorrupted
            | ErrorCode::DatabasePermissionDenied => ErrorCategory::Database,

            ErrorCode::ModelLoadFailed
            | ErrorCode::ModelInferenceFailed
            | ErrorCode::ModelOutOfMemory
            | ErrorCode::ModelTokenizerError
            | ErrorCode::ModelStateError
            | ErrorCode::ModelQuantizationError => ErrorCategory::Model,

            ErrorCode::AgentNotFound
            | ErrorCode::AgentRegistrationFailed
            | ErrorCode::AgentConfigurationError
            | ErrorCode::AgentPromptError
            | ErrorCode::AgentMemoryError
            | ErrorCode::AgentToolError => ErrorCategory::Agent,

            ErrorCode::NetworkTimeout
            | ErrorCode::NetworkConnectionFailed
            | ErrorCode::NetworkRequestFailed
            | ErrorCode::NetworkAuthenticationFailed => ErrorCategory::Network,

            ErrorCode::SystemOutOfMemory
            | ErrorCode::SystemDiskFull
            | ErrorCode::SystemPermissionDenied
            | ErrorCode::SystemResourceExhausted
            | ErrorCode::SystemThreadPanic => ErrorCategory::System,

            ErrorCode::UserInputTooLong
            | ErrorCode::UserInputInvalidFormat
            | ErrorCode::UserInputContainsForbidden
            | ErrorCode::UserInputEmpty => ErrorCategory::UserInput,

            ErrorCode::InternalUnknownError
            | ErrorCode::InternalLogicError
            | ErrorCode::InternalDataCorruption
            | ErrorCode::InternalTimeout => ErrorCategory::Internal,
        }
    }

    /// 获取错误代码对应的严重级别
    pub fn severity(&self) -> ErrorSeverity {
        match self {
            // 配置错误通常是中高级别
            ErrorCode::ConfigFileNotFound
            | ErrorCode::ModelPathNotFound
            | ErrorCode::TokenizerNotFound
            | ErrorCode::StateFileNotFound => ErrorSeverity::High,
            ErrorCode::ConfigParseError | ErrorCode::ConfigValidationError => ErrorSeverity::Medium,

            // 数据库错误
            ErrorCode::DatabaseConnectionFailed | ErrorCode::DatabaseCorrupted => {
                ErrorSeverity::Critical
            }
            ErrorCode::DatabaseQueryFailed
            | ErrorCode::DatabaseTransactionFailed
            | ErrorCode::DatabaseMigrationFailed => ErrorSeverity::High,
            ErrorCode::DatabaseTimeout | ErrorCode::DatabasePermissionDenied => {
                ErrorSeverity::Medium
            }

            // 模型错误
            ErrorCode::ModelLoadFailed => ErrorSeverity::Critical,
            ErrorCode::ModelOutOfMemory => ErrorSeverity::High,
            ErrorCode::ModelInferenceFailed
            | ErrorCode::ModelTokenizerError
            | ErrorCode::ModelStateError
            | ErrorCode::ModelQuantizationError => ErrorSeverity::Medium,

            // 智能体错误
            ErrorCode::AgentNotFound | ErrorCode::AgentConfigurationError => ErrorSeverity::Medium,
            ErrorCode::AgentRegistrationFailed
            | ErrorCode::AgentPromptError
            | ErrorCode::AgentMemoryError
            | ErrorCode::AgentToolError => ErrorSeverity::Low,

            // 网络错误
            ErrorCode::NetworkTimeout
            | ErrorCode::NetworkConnectionFailed
            | ErrorCode::NetworkRequestFailed => ErrorSeverity::Low,
            ErrorCode::NetworkAuthenticationFailed => ErrorSeverity::Medium,

            // 系统错误
            ErrorCode::SystemOutOfMemory
            | ErrorCode::SystemDiskFull
            | ErrorCode::SystemResourceExhausted => ErrorSeverity::Critical,
            ErrorCode::SystemThreadPanic => ErrorSeverity::High,
            ErrorCode::SystemPermissionDenied => ErrorSeverity::Medium,

            // 用户输入错误
            ErrorCode::UserInputTooLong
            | ErrorCode::UserInputInvalidFormat
            | ErrorCode::UserInputContainsForbidden
            | ErrorCode::UserInputEmpty => ErrorSeverity::Low,

            // 内部错误
            ErrorCode::InternalDataCorruption => ErrorSeverity::Critical,
            ErrorCode::InternalLogicError => ErrorSeverity::High,
            ErrorCode::InternalUnknownError | ErrorCode::InternalTimeout => ErrorSeverity::Medium,
        }
    }

    /// 获取错误代码的描述信息
    pub fn description(&self) -> &'static str {
        match self {
            ErrorCode::ConfigFileNotFound => "配置文件未找到",
            ErrorCode::ConfigParseError => "配置文件解析失败",
            ErrorCode::ConfigValidationError => "配置验证失败",
            ErrorCode::ModelPathNotFound => "模型文件路径不存在",
            ErrorCode::TokenizerNotFound => "分词器文件未找到",
            ErrorCode::StateFileNotFound => "状态文件未找到",

            ErrorCode::DatabaseConnectionFailed => "数据库连接失败",
            ErrorCode::DatabaseQueryFailed => "数据库查询失败",
            ErrorCode::DatabaseTransactionFailed => "数据库事务失败",
            ErrorCode::DatabaseMigrationFailed => "数据库迁移失败",
            ErrorCode::DatabaseTimeout => "数据库操作超时",
            ErrorCode::DatabaseCorrupted => "数据库损坏",
            ErrorCode::DatabasePermissionDenied => "数据库权限不足",

            ErrorCode::ModelLoadFailed => "模型加载失败",
            ErrorCode::ModelInferenceFailed => "模型推理失败",
            ErrorCode::ModelOutOfMemory => "模型内存不足",
            ErrorCode::ModelTokenizerError => "分词器错误",
            ErrorCode::ModelStateError => "模型状态错误",
            ErrorCode::ModelQuantizationError => "模型量化错误",

            ErrorCode::AgentNotFound => "智能体未找到",
            ErrorCode::AgentRegistrationFailed => "智能体注册失败",
            ErrorCode::AgentConfigurationError => "智能体配置错误",
            ErrorCode::AgentPromptError => "智能体提示词错误",
            ErrorCode::AgentMemoryError => "智能体记忆错误",
            ErrorCode::AgentToolError => "智能体工具错误",

            ErrorCode::NetworkTimeout => "网络超时",
            ErrorCode::NetworkConnectionFailed => "网络连接失败",
            ErrorCode::NetworkRequestFailed => "网络请求失败",
            ErrorCode::NetworkAuthenticationFailed => "网络认证失败",

            ErrorCode::SystemOutOfMemory => "系统内存不足",
            ErrorCode::SystemDiskFull => "磁盘空间不足",
            ErrorCode::SystemPermissionDenied => "系统权限不足",
            ErrorCode::SystemResourceExhausted => "系统资源耗尽",
            ErrorCode::SystemThreadPanic => "线程异常终止",

            ErrorCode::UserInputTooLong => "用户输入过长",
            ErrorCode::UserInputInvalidFormat => "用户输入格式错误",
            ErrorCode::UserInputContainsForbidden => "用户输入包含禁止内容",
            ErrorCode::UserInputEmpty => "用户输入为空",

            ErrorCode::InternalUnknownError => "内部未知错误",
            ErrorCode::InternalLogicError => "内部逻辑错误",
            ErrorCode::InternalDataCorruption => "内部数据损坏",
            ErrorCode::InternalTimeout => "内部操作超时",
        }
    }
}

/// 统一错误类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RwkvError {
    /// 错误代码
    pub code: ErrorCode,
    /// 错误消息
    pub message: String,
    /// 错误上下文
    pub context: Option<String>,
    /// 错误发生时间戳
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// 错误追踪ID
    pub trace_id: Option<String>,
    /// 源错误（如果存在）
    pub source: Option<String>,
}

impl RwkvError {
    /// 创建新的错误
    pub fn new(code: ErrorCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            context: None,
            timestamp: chrono::Utc::now(),
            trace_id: None,
            source: None,
        }
    }

    /// 添加上下文信息
    pub fn with_context(mut self, context: impl Into<String>) -> Self {
        self.context = Some(context.into());
        self
    }

    /// 添加追踪ID
    pub fn with_trace_id(mut self, trace_id: impl Into<String>) -> Self {
        self.trace_id = Some(trace_id.into());
        self
    }

    /// 添加源错误
    pub fn with_source(mut self, source: impl Into<String>) -> Self {
        self.source = Some(source.into());
        self
    }

    /// 获取错误类别
    pub fn category(&self) -> ErrorCategory {
        self.code.category()
    }

    /// 获取错误严重级别
    pub fn severity(&self) -> ErrorSeverity {
        self.code.severity()
    }

    /// 检查是否为可恢复错误
    pub fn is_recoverable(&self) -> bool {
        matches!(self.severity(), ErrorSeverity::Low | ErrorSeverity::Medium)
    }

    /// 检查是否为致命错误
    pub fn is_fatal(&self) -> bool {
        matches!(self.severity(), ErrorSeverity::Critical)
    }

    /// 转换为JSON字符串
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|_| {
            format!(
                "{{\"code\":{},\"message\":\"{}\"}}",
                self.code as u32, self.message
            )
        })
    }
}

impl fmt::Display for RwkvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}] {}: {}",
            self.code as u32,
            self.code.description(),
            self.message
        )?;

        if let Some(context) = &self.context {
            write!(f, " (上下文: {})", context)?;
        }

        if let Some(trace_id) = &self.trace_id {
            write!(f, " [追踪ID: {}]", trace_id)?;
        }

        Ok(())
    }
}

// 手动实现 std::error::Error trait
impl std::error::Error for RwkvError {}

/// 快速创建错误的宏
#[macro_export]
macro_rules! rwkv_error {
    ($code:expr, $msg:expr) => {
        RwkvError::new($code, $msg)
    };
    ($code:expr, $msg:expr, $context:expr) => {
        RwkvError::new($code, $msg).with_context($context)
    };
}

/// 错误结果类型别名
pub type RwkvResult<T> = Result<T, RwkvError>;

/// 错误转换trait
pub trait IntoRwkvError {
    fn into_rwkv_error(self, code: ErrorCode) -> RwkvError;
}

impl<E: std::error::Error> IntoRwkvError for E {
    fn into_rwkv_error(self, code: ErrorCode) -> RwkvError {
        RwkvError::new(code, self.to_string()).with_source(format!("{:?}", self))
    }
}

/// 从标准错误转换为RWKV错误
impl From<std::io::Error> for RwkvError {
    fn from(err: std::io::Error) -> Self {
        let code = match err.kind() {
            std::io::ErrorKind::NotFound => ErrorCode::ConfigFileNotFound,
            std::io::ErrorKind::PermissionDenied => ErrorCode::SystemPermissionDenied,
            std::io::ErrorKind::OutOfMemory => ErrorCode::SystemOutOfMemory,
            std::io::ErrorKind::TimedOut => ErrorCode::InternalTimeout,
            _ => ErrorCode::InternalUnknownError,
        };

        RwkvError::new(code, err.to_string()).with_source(format!("{:?}", err))
    }
}

impl From<serde_json::Error> for RwkvError {
    fn from(err: serde_json::Error) -> Self {
        RwkvError::new(ErrorCode::ConfigParseError, err.to_string())
            .with_source(format!("{:?}", err))
    }
}

impl From<sqlx::Error> for RwkvError {
    fn from(err: sqlx::Error) -> Self {
        let code = match &err {
            sqlx::Error::Database(_) => ErrorCode::DatabaseQueryFailed,
            sqlx::Error::Io(_) => ErrorCode::DatabaseConnectionFailed,
            sqlx::Error::Tls(_) => ErrorCode::DatabaseConnectionFailed,
            sqlx::Error::Protocol(_) => ErrorCode::DatabaseConnectionFailed,
            sqlx::Error::RowNotFound => ErrorCode::DatabaseQueryFailed,
            sqlx::Error::TypeNotFound { .. } => ErrorCode::DatabaseQueryFailed,
            sqlx::Error::ColumnIndexOutOfBounds { .. } => ErrorCode::DatabaseQueryFailed,
            sqlx::Error::ColumnNotFound(_) => ErrorCode::DatabaseQueryFailed,
            sqlx::Error::ColumnDecode { .. } => ErrorCode::DatabaseQueryFailed,
            sqlx::Error::Decode(_) => ErrorCode::DatabaseQueryFailed,
            sqlx::Error::PoolTimedOut => ErrorCode::DatabaseTimeout,
            sqlx::Error::PoolClosed => ErrorCode::DatabaseConnectionFailed,
            sqlx::Error::WorkerCrashed => ErrorCode::DatabaseConnectionFailed,
            sqlx::Error::Migrate(_) => ErrorCode::DatabaseMigrationFailed,
            _ => ErrorCode::DatabaseQueryFailed,
        };

        RwkvError::new(code, err.to_string()).with_source(format!("{:?}", err))
    }
}

impl From<anyhow::Error> for RwkvError {
    fn from(err: anyhow::Error) -> Self {
        // 尝试从anyhow错误中提取更具体的错误类型
        if let Some(io_err) = err.downcast_ref::<std::io::Error>() {
            return RwkvError::from(std::io::Error::new(io_err.kind(), io_err.to_string()));
        }

        if let Some(json_err) = err.downcast_ref::<serde_json::Error>() {
            return RwkvError::new(
                ErrorCode::ConfigParseError,
                format!("JSON 序列化错误: {}", json_err),
            );
        }

        if let Some(sqlx_err) = err.downcast_ref::<sqlx::Error>() {
            return RwkvError::new(
                ErrorCode::DatabaseQueryFailed,
                format!("数据库错误: {}", sqlx_err),
            );
        }

        // 默认为内部错误
        RwkvError::new(ErrorCode::InternalUnknownError, err.to_string())
            .with_source(format!("{:?}", err))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let error = RwkvError::new(ErrorCode::ModelLoadFailed, "测试错误");
        assert_eq!(error.code, ErrorCode::ModelLoadFailed);
        assert_eq!(error.message, "测试错误");
        assert_eq!(error.category(), ErrorCategory::Model);
        assert_eq!(error.severity(), ErrorSeverity::Critical);
        assert!(!error.is_recoverable());
        assert!(error.is_fatal());
    }

    #[test]
    fn test_error_with_context() {
        let error = RwkvError::new(ErrorCode::AgentNotFound, "智能体不存在")
            .with_context("agent_name: test_agent")
            .with_trace_id("trace_123");

        assert_eq!(error.context, Some("agent_name: test_agent".to_string()));
        assert_eq!(error.trace_id, Some("trace_123".to_string()));
    }

    #[test]
    fn test_error_display() {
        let error =
            RwkvError::new(ErrorCode::DatabaseTimeout, "查询超时").with_context("表: users");

        let display = format!("{}", error);
        assert!(display.contains("[2005]"));
        assert!(display.contains("数据库操作超时"));
        assert!(display.contains("查询超时"));
        assert!(display.contains("上下文: 表: users"));
    }

    #[test]
    fn test_error_json_serialization() {
        let error = RwkvError::new(ErrorCode::UserInputTooLong, "输入超过限制");
        let json = error.to_json();
        assert!(json.contains("7001"));
        assert!(json.contains("输入超过限制"));
    }

    #[test]
    fn test_error_conversions() {
        let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "文件未找到");
        let rwkv_error: RwkvError = io_error.into();
        assert_eq!(rwkv_error.code, ErrorCode::ConfigFileNotFound);

        let json_error = serde_json::from_str::<serde_json::Value>("invalid json").unwrap_err();
        let rwkv_error: RwkvError = json_error.into();
        assert_eq!(rwkv_error.code, ErrorCode::ConfigParseError);
    }
}

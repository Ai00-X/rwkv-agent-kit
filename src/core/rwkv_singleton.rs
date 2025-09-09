//! RWKV 服务全局单例模块
//!
//! 提供全局的 RWKV 服务实例，确保整个应用程序中只有一个 RWKV 推理引擎实例。
//! 这样可以避免重复初始化，节省资源，并确保配置的一致性。

use crate::core::config::KitConfig;
use crate::core::service::RwkvAgentKit as CoreService;
use crate::rwkv::config::ModelConfig;
use anyhow::Result;
use std::sync::{Arc, Mutex};
use tokio::sync::RwLock;

/// 全局 RWKV 服务实例（使用 OnceCell 避免 unsafe 静态可变引用）
static GLOBAL_RWKV_SERVICE: once_cell::sync::OnceCell<Arc<RwLock<CoreService>>> =
    once_cell::sync::OnceCell::new();
static RWKV_SERVICE_MUTEX: Mutex<()> = Mutex::new(());

/// 预加载全局 RWKV 服务
///
/// 这个函数应该在应用程序启动时调用，用于初始化全局的 RWKV 服务实例。
/// 使用默认的模型配置进行初始化。
pub async fn preload_global_rwkv_service() -> Result<()> {
    preload_global_rwkv_service_with_config(ModelConfig::default()).await
}

/// 使用指定配置预加载全局 RWKV 服务
///
/// # 参数
/// * `model_config` - RWKV 模型配置
pub async fn preload_global_rwkv_service_with_config(model_config: ModelConfig) -> Result<()> {
    {
        let _guard = RWKV_SERVICE_MUTEX.lock().unwrap();
        
        // 检查是否已经初始化
        if GLOBAL_RWKV_SERVICE.get().is_some() {
            return Ok(());
        }
    } // 锁在这里释放

    println!("正在初始化全局 RWKV 服务...");

    // 创建 KitConfig
    let kit_config = KitConfig {
        model: model_config,
        agents: vec![],
        database: crate::db::DatabaseConfig::default(),
    };

    // 初始化核心服务
    let core_service = CoreService::new(kit_config).await?;

    // 设置全局实例
    GLOBAL_RWKV_SERVICE
        .set(Arc::new(RwLock::new(core_service)))
        .map_err(|_| anyhow::anyhow!("全局 RWKV 服务已初始化"))?;

    println!("全局 RWKV 服务初始化完成");
    Ok(())
}

/// 获取全局 RWKV 服务实例
///
/// # 返回值
/// 返回全局 RWKV 服务的 Arc<RwLock<CoreService>> 引用
///
/// # 错误
/// 如果全局服务尚未初始化，返回错误
pub fn get_global_rwkv_service() -> Result<Arc<RwLock<CoreService>>> {
    GLOBAL_RWKV_SERVICE.get().cloned().ok_or_else(|| {
        anyhow::anyhow!("全局 RWKV 服务尚未初始化，请先调用 preload_global_rwkv_service()")
    })
}

/// 检查全局 RWKV 服务是否已初始化
pub fn is_global_rwkv_service_initialized() -> bool {
    GLOBAL_RWKV_SERVICE.get().is_some()
}

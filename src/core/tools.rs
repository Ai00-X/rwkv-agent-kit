//! 工具系统模块

use anyhow::Result;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// 工具注册表
#[derive(Default)]
pub struct ToolRegistry {
    pub tools: HashMap<String, Box<dyn Tool>>,
}

impl ToolRegistry {
    /// 注册新工具
    pub fn register<T: Tool + 'static>(&mut self, tool: T) {
        let name = tool.name().to_string();
        self.tools.insert(name, Box::new(tool));
    }

    /// 获取工具列表 (名称)
    pub fn list_tools(&self) -> Vec<String> {
        self.tools.keys().cloned().collect()
    }

    /// 获取工具描述列表
    pub fn list_tool_descriptions(&self) -> Vec<(String, String)> {
        self.tools
            .iter()
            .map(|(name, tool)| (name.clone(), tool.description().to_string()))
            .collect()
    }

    /// 执行工具
    pub async fn execute_tool(
        &self,
        tool_name: &str,
        params: serde_json::Value,
    ) -> Result<serde_json::Value> {
        match self.tools.get(tool_name) {
            Some(tool) => tool.execute(params).await,
            None => Err(anyhow::anyhow!("Tool '{}' not found", tool_name)),
        }
    }

    /// 获取工具数量
    pub fn count(&self) -> usize {
        self.tools.len()
    }
}

impl std::fmt::Debug for ToolRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ToolRegistry")
            .field("tools", &format!("HashMap with {} tools", self.tools.len()))
            .finish()
    }
}

/// 工具接口
#[async_trait::async_trait]
pub trait Tool: Send + Sync {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    async fn execute(&self, params: serde_json::Value) -> Result<serde_json::Value>;
}

/// 共享工具注册表类型
pub type SharedToolRegistry = Arc<RwLock<ToolRegistry>>;

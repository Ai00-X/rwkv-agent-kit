//! Router Agent
//!
//! Main routing agent that analyzes user input and dispatches to appropriate agents and tools

use crate::{
    agent::config::MemoryConfig,
    agent::InferenceParams,
    agent::{Agent, AgentConfig, PromptBuilder, PromptBuilderInstance},
    rwkv::ModelConfig,
};
use anyhow::Result;
use std::sync::Arc;
// use serde::{Deserialize, Serialize}; // 暂时未使用
use chrono::Datelike; // for weekday()

/// Router Prompt Builder
#[derive(Debug, Clone)]
pub struct RouterPromptBuilder {
    /// Available agents for routing
    pub available_agents: Vec<String>,
    /// Available tools for routing
    pub available_tools: Vec<String>,
}

impl Default for RouterPromptBuilder {
    fn default() -> Self {
        Self {
            available_agents: vec![
                "chat".to_string(),
                "conversation_summarizer".to_string(),
                "persona_extractor".to_string(),
            ],
            available_tools: vec![],
        }
    }
}

impl RouterPromptBuilder {
    /// Create with custom agents and tools
    pub fn with_agents_and_tools(agents: Vec<String>, tools: Vec<String>) -> Self {
        Self {
            available_agents: agents,
            available_tools: tools,
        }
    }
}

#[async_trait::async_trait]
impl PromptBuilder for RouterPromptBuilder {
    async fn build_prompt(
        &self,
        _agent: &Agent,
        user_input: &str,
        _tools: &[String],
    ) -> Result<String> {
        let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
        let weekday_cn = match chrono::Local::now().weekday() {
            chrono::Weekday::Mon => "周一",
            chrono::Weekday::Tue => "周二",
            chrono::Weekday::Wed => "周三",
            chrono::Weekday::Thu => "周四",
            chrono::Weekday::Fri => "周五",
            chrono::Weekday::Sat => "周六",
            chrono::Weekday::Sun => "周日",
        };

        let agents_list = self.available_agents.join(", ");
        let tools_list = if self.available_tools.is_empty() {
            "无".to_string()
        } else {
            self.available_tools.join(", ")
        };

        let base_prompt = format!(
            r#"System: 你是一个路由智能体，负责分析用户输入并决定调用哪些智能体和工具来完成任务。你需要输出标准JSON格式的路由决策。

当前时间: {}，{}
可用智能体: {}
可用工具: {}

分析用户输入并制定执行计划。根据输入类型选择合适的智能体和工具：

智能体功能说明：
- chat: 通用对话智能体，处理日常聊天、回答问题、提供建议
- conversation_summarizer: 对话总结智能体，分析对话重要性和提取关键信息
- persona_extractor: 画像提取智能体，从对话中提取用户特征和偏好

工具功能说明：
- 根据具体可用工具进行选择

输出JSON格式：
{{
  "analysis": "对用户输入的分析",
  "execution_plan": {{
    "agents": [
      {{
        "name": "智能体名称",
        "priority": 1,
        "reason": "选择原因"
      }}
    ],
    "tools": [
      {{
        "name": "工具名称", 
        "priority": 1,
        "params": {{}},
        "reason": "选择原因"
      }}
    ],
    "parallel_execution": true,
    "aggregation_method": "智能汇总"
  }}
}}

User: {user_input}
"#,
            now, weekday_cn, agents_list, tools_list,
        );

        Ok(base_prompt)
    }
}

/// Create router agent configuration
pub fn create_router_config() -> AgentConfig {
    AgentConfig {
        name: "router".to_string(),
        prompt_template: "{user_input}".to_string(),
        inference_params: InferenceParams {
            max_tokens: 1024,
            temperature: 0.3,
            top_p: 0.9,
            presence_penalty: 0.0,
            frequency_penalty: 0.0,
            stop_sequences: vec!["\n\n".to_string()],
        },
        tools: vec![],
        state: Some("router".to_string()),
        prompt_builder: Some(Arc::new(PromptBuilderInstance::Router(
            RouterPromptBuilder::default(),
        ))),
        save_conversations: false,
        memory: MemoryConfig::disabled(),
    }
}

/// Create router agent
pub async fn create_router() -> Result<Agent> {
    let config = create_router_config();
    let model_config = ModelConfig::default();
    Agent::new(config, &model_config)
}

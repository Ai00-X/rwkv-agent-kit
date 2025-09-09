//! 预定义智能体模块
//!
//! 提供一系列开箱即用的智能体，无需复杂配置即可启动使用

pub mod conversation_summarizer;
// pub mod persona_extractor; // 已移除画像智能体
pub mod chat;
pub mod router;
pub mod workflow_aggregator;

// 重新导出主要的创建函数和配置
pub use conversation_summarizer::{
    create_conversation_summarizer, create_conversation_summarizer_config,
    ConversationSummarizerPromptBuilder,
};
// PersonaExtractor 相关导出已移除
pub use chat::{create_chat, create_chat_config, ChatPromptBuilder};
pub use router::{create_router, create_router_config, RouterPromptBuilder};
pub use workflow_aggregator::{AgentResult, ToolResult, WorkflowAggregator, WorkflowResult};

use crate::agent::{Agent, AgentConfig};
use anyhow::Result;

/// 智能体类型枚举
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AgentType {
    /// 对话总结智能体
    ConversationSummarizer,
    // PersonaExtractor, // 已移除画像智能体
    /// 主对话智能体
    Chat,
    /// 路由智能体
    Router,
}

/// 智能体工厂
///
/// 提供统一的智能体创建接口
pub struct AgentFactory;

impl AgentFactory {
    /// 根据类型创建智能体配置
    pub fn create_config(agent_type: AgentType) -> AgentConfig {
        match agent_type {
            AgentType::ConversationSummarizer => create_conversation_summarizer_config(),
            // AgentType::PersonaExtractor => create_persona_extractor_config(), // 已移除
            AgentType::Chat => create_chat_config(),
            AgentType::Router => create_router_config(),
        }
    }

    /// 根据类型创建智能体实例
    pub async fn create_agent(agent_type: AgentType) -> Result<Agent> {
        match agent_type {
            AgentType::ConversationSummarizer => create_conversation_summarizer().await,
            // AgentType::PersonaExtractor => create_persona_extractor().await, // 已移除
            AgentType::Chat => create_chat().await,
            AgentType::Router => create_router().await,
        }
    }

    /// 获取所有可用的智能体类型
    pub fn available_types() -> Vec<AgentType> {
        vec![
            AgentType::ConversationSummarizer,
            // AgentType::PersonaExtractor, // 已移除
            AgentType::Chat,
            AgentType::Router,
        ]
    }

    /// 根据名称获取智能体类型
    pub fn get_type_by_name(name: &str) -> Option<AgentType> {
        match name {
            "conversation_summarizer" => Some(AgentType::ConversationSummarizer),
            // "persona_extractor" => Some(AgentType::PersonaExtractor), // 已移除
            "chat" => Some(AgentType::Chat),
            "router" => Some(AgentType::Router),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_factory_config_creation() {
        let summarizer_config = AgentFactory::create_config(AgentType::ConversationSummarizer);
        assert_eq!(summarizer_config.name, "conversation_summarizer");

        // PersonaExtractor 测试已移除

        let chat_config = AgentFactory::create_config(AgentType::Chat);
        assert_eq!(chat_config.name, "chat");

        let router_config = AgentFactory::create_config(AgentType::Router);
        assert_eq!(router_config.name, "router");
    }

    // #[tokio::test]
    // async fn test_agent_creation() {
    //     // 注意：这个测试需要实际的模型文件才能运行
    //     // 在没有模型文件的情况下，这个测试会失败
    //     // 可以考虑使用模拟或跳过这个测试

    //     // let summarizer = AgentFactory::create_agent(AgentType::ConversationSummarizer).await;
    //     // assert!(summarizer.is_ok());
    // }
}

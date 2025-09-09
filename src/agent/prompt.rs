//! 提示词构建模块

use super::Agent;
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait PromptBuilder: Send + Sync + std::fmt::Debug {
    async fn build_prompt(
        &self,
        agent: &Agent,
        user_input: &str,
        tools: &[String],
    ) -> Result<String>;
}

/// PromptBuilder 实例枚举，用于避免 trait object 的对象安全问题
#[derive(Debug, Clone)]
pub enum PromptBuilderInstance {
    Default(DefaultPromptBuilder),
    Chat(super::super::agents::ChatPromptBuilder),
    Router(super::super::agents::RouterPromptBuilder),
    ConversationSummarizer(super::super::agents::ConversationSummarizerPromptBuilder),
}

#[async_trait]
impl PromptBuilder for PromptBuilderInstance {
    async fn build_prompt(
        &self,
        agent: &Agent,
        user_input: &str,
        tools: &[String],
    ) -> Result<String> {
        match self {
            PromptBuilderInstance::Default(builder) => {
                builder.build_prompt(agent, user_input, tools).await
            }
            PromptBuilderInstance::Chat(builder) => {
                builder.build_prompt(agent, user_input, tools).await
            }
            PromptBuilderInstance::Router(builder) => {
                builder.build_prompt(agent, user_input, tools).await
            }
            PromptBuilderInstance::ConversationSummarizer(builder) => {
                builder.build_prompt(agent, user_input, tools).await
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct DefaultPromptBuilder;

#[async_trait]
impl PromptBuilder for DefaultPromptBuilder {
    async fn build_prompt(
        &self,
        agent: &Agent,
        user_input: &str,
        tools: &[String],
    ) -> Result<String> {
        let mut prompt = agent.config.prompt_template.clone();
        // 工具列表
        let tools_list = if tools.is_empty() {
            "无".to_string()
        } else {
            tools.join(", ")
        };
        prompt = prompt.replace("{tools}", &tools_list);

        // 记忆注入（阶段4.3）
        let memory_context = String::new();
        if agent.config.memory.enabled {
            // 实际检索逻辑将在后续实现并通过全局上下文获取DbManager
            // 此处先保留占位，避免阻塞集成
            // memory_context = retrieve_and_format_memories(...).await?;
        }

        // 将历史与记忆上下文注入
        if !prompt.contains("{history}") {
            prompt.push_str("\n{history}");
        }
        let history_block = String::new(); // 暂无历史
        let mut composed_history = String::new();
        if !memory_context.is_empty() {
            composed_history.push_str("[Memory]\n");
            composed_history.push_str(&memory_context);
            composed_history.push('\n');
        }
        composed_history.push_str(&history_block);
        prompt = prompt.replace("{history}", &composed_history);

        // 注入用户输入
        prompt = prompt.replace("{user_input}", user_input);
        if !agent.config.prompt_template.contains("{user_input}") {
            prompt.push_str(&format!("\n\nUser: {}\n\nAssistant: ", user_input));
        }
        Ok(prompt)
    }
}

impl DefaultPromptBuilder {
    // 可在此扩展辅助方法
}

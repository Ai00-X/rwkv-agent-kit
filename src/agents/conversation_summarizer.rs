//! Conversation Summarizer Agent
//!
//! Specialized agent for summarizing conversation records into concise summaries within 100 characters

use crate::{
    agent::config::MemoryConfig,
    agent::InferenceParams,
    agent::{Agent, AgentConfig, PromptBuilder, PromptBuilderInstance},
    rwkv::ModelConfig,
};
use anyhow::Result;
use std::sync::Arc;

/// Conversation Summarizer Prompt Builder
#[derive(Debug, Clone)]
pub struct ConversationSummarizerPromptBuilder;

#[async_trait::async_trait]
impl PromptBuilder for ConversationSummarizerPromptBuilder {
    async fn build_prompt(
        &self,
        _agent: &Agent,
        user_input: &str,
        _tools: &[String],
    ) -> Result<String> {
        let prompt = format!(
            r#"You are a conversation analyzer. Analyze multi-turn conversations and output JSON summary with 4 fields. Output ONLY valid JSON.
Strictly base on history only. No hallucinations. If lacks substance, score low and output {{}}.
Evaluate importance 0-10;  
Otherwise, fill fields reflecting input.
JSON: {{"importance_score": int 0-10, "user_intent_summary": "intent from input, max 50 tokens", "summary": "accurate summary, max 100 tokens", "memory_triggers": ["3-5 keywords, max 5 tokens each"]}}
Constraints:
- importance_score: 0-10 int for significance;
- user_intent_summary: Exact user intent from input, empty if low/unclear;
- summary: Accurate conversation summary, empty if low;
- memory_triggers: 3-5 extracted terms (entities, concepts), avoid time words/verbs, empty array if low.
IMPORTANT: Use history language for fields.

User: There is some chat history: {}

Assistant: <think>
</think>
```json
"#,
            user_input.replace("\n", "")
        );
        Ok(prompt)
    }
}

/// Create conversation summarizer agent configuration
pub fn create_conversation_summarizer_config() -> AgentConfig {
    AgentConfig {
        name: "conversation_summarizer".to_string(),
        prompt_template: "{user_input}".to_string(), // Use custom prompt builder
        inference_params: InferenceParams {
            max_tokens: 4500, // Increased to accommodate entity extraction
            temperature: 0.0, // Lower temperature for stable output
            top_p: 0.0,
            presence_penalty: 0.0,
            frequency_penalty: 0.0,
            stop_sequences: vec![r"]}}".to_string()], // Remove stop sequences, let model end naturally
        },
        tools: vec![],                   // No tools needed
        state: Some("None".to_string()), // Use tool-call state
        prompt_builder: Some(Arc::new(PromptBuilderInstance::ConversationSummarizer(
            ConversationSummarizerPromptBuilder,
        ))),
        save_conversations: false, // 对话总结智能体不保存对话
        memory: MemoryConfig::default(),
    }
}

/// Create conversation summarizer agent
pub async fn create_conversation_summarizer() -> Result<Agent> {
    let config = create_conversation_summarizer_config();
    let model_config = ModelConfig::default();
    Agent::new(config, &model_config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversation_summarizer_config() {
        let config = create_conversation_summarizer_config();
        assert_eq!(config.name, "conversation_summarizer");
        assert_eq!(config.inference_params.max_tokens, 4500);
        assert!(config.prompt_builder.is_some());
    }

    #[tokio::test]
    async fn test_prompt_builder() {
        let builder = ConversationSummarizerPromptBuilder;
        let test_input = "User: How's the weather today?\nAssistant: The weather is sunny today with a pleasant temperature, perfect for outdoor activities.";

        // Create a mock agent for testing
        let config = create_conversation_summarizer_config();
        let model_config = ModelConfig::default();
        let agent = Agent::new(config, &model_config).unwrap();

        let prompt = builder.build_prompt(&agent, test_input, &[]).await.unwrap();
        assert!(prompt.contains("max 100 tokens"));
        // Since newlines are replaced in the prompt, check for the content without newlines
        let test_input_no_newlines = test_input.replace("\n", "");
        assert!(prompt.contains(&test_input_no_newlines));
    }
}

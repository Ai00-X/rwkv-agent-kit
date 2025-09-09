//! Persona Extractor Agent - DISABLED
//! 
//! This module has been disabled to remove persona extraction functionality
//! The code is preserved for potential future restoration

/*
use anyhow::Result;
use std::sync::Arc;
use crate::{
    agent::{Agent, AgentConfig, PromptBuilder},
    InferenceParams,
    ModelConfig,
    agent::config::MemoryConfig,
};
use crate::agent::prompt::PromptBuilder;

/// Persona Extractor Prompt Builder
#[derive(Debug, Clone)]
pub struct PersonaExtractorPromptBuilder;

#[async_trait::async_trait]
impl PromptBuilder for PersonaExtractorPromptBuilder {
    async fn build_prompt(
        &self,
        _agent: &Agent,
        user_input: &str,
        _tools: &[String],
    ) -> Result<String> {
        let prompt = format!(
            r#"Instruction: You are an expert assistant for extracting USER personality traits from conversations.

IMPORTANT: Only analyze and extract traits from USER messages. Completely ignore Assistant responses.

Rules:
- Only extract traits from lines starting with "User:" or user messages
- Ignore all "Assistant:" responses and AI-generated content
- Focus solely on what the USER says, thinks, prefers, or expresses
- Do not extract traits from Assistant's suggestions or responses

Generate a valid JSON in the following format:
{{
    "traits": [
        {{
            "trait_type": "preference",
            "trait_key": "music", 
            "trait_value": "classical music",
            "confidence": 0.9,
            "evidence": "user said they love classical music"
        }}
    ]
}}

Trait types: preference, attribute, taboo, goal, style, context

Conversation: {}

Analyze ONLY the USER's messages and extract their personality traits:

Response: ``` json"#,
            user_input.replace("\n", " ")
        );
        Ok(prompt)
    }
}

/// Create persona extractor agent configuration
pub fn create_persona_extractor_config() -> AgentConfig {
    AgentConfig {
        name: "persona_extractor".to_string(),
        prompt_template: "{user_input}".to_string(), // Use custom prompt builder
        inference_params: InferenceParams {
            max_tokens: 2000,
            temperature: 0.1, // Low temperature for consistent output
            top_p: 0.0,
            stop_sequences: vec!["```".to_string()], // Stop at end of JSON block
        },
        tools: vec![], // No tools needed
        state: Some("None".to_string()),
        prompt_builder: Some(Arc::new(PersonaExtractorPromptBuilder)),
        save_conversations: false, // Persona extractor doesn't save conversations
        memory: MemoryConfig::default(),
    }
}

/// Create persona extractor agent
pub async fn create_persona_extractor() -> Result<Agent> {
    let config = create_persona_extractor_config();
    let model_config = ModelConfig::default();
    Agent::new(config, &model_config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_persona_extractor_config() {
        let config = create_persona_extractor_config();
        assert_eq!(config.name, "persona_extractor");
        assert_eq!(config.inference_params.max_tokens, 2000);
        assert!(config.prompt_builder.is_some());
    }

    #[tokio::test]
    async fn test_prompt_builder() {
        let builder = PersonaExtractorPromptBuilder;
        let test_input = "User: I really love classical music and piano performances.\nAssistant: That's wonderful! Do you play piano yourself?";
        
        // Create a mock agent for testing
        let config = create_persona_extractor_config();
        let model_config = crate::ModelConfig::default();
        let agent = Agent::new(config, &model_config).unwrap();
        
        let prompt = builder.build_prompt(&agent, test_input, &[]).await.unwrap();
        assert!(prompt.contains("trait_type"));
        assert!(prompt.contains("confidence"));
        assert!(prompt.contains("Only analyze and extract traits from USER messages"));
        assert!(prompt.contains("Ignore all \"Assistant:\" responses"));
 
    }
}
*/
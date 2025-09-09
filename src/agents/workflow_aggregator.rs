//! Workflow Aggregator
//!
//! Responsible for aggregating results from multiple agents and tools into a coherent response

use anyhow::Result;
use serde::{Deserialize, Serialize};
// use std::collections::HashMap; // 暂时未使用

/// Workflow execution result from an agent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentResult {
    pub agent_name: String,
    pub response: String,
    pub error: Option<String>,
    pub execution_time_ms: u64,
}

/// Workflow execution result from a tool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult {
    pub tool_name: String,
    pub result: serde_json::Value,
    pub error: Option<String>,
    pub execution_time_ms: u64,
}

/// Complete workflow execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowResult {
    pub user_input: String,
    pub analysis: String,
    pub agent_results: Vec<AgentResult>,
    pub tool_results: Vec<ToolResult>,
    pub aggregated_response: String,
    pub total_execution_time_ms: u64,
    pub parallel_execution: bool,
}

/// Workflow Aggregator
///
/// Aggregates results from multiple agents and tools into a coherent response
pub struct WorkflowAggregator;

impl WorkflowAggregator {
    /// Aggregate results from multiple agents and tools
    pub fn aggregate_results(
        user_input: &str,
        analysis: &str,
        agent_results: Vec<AgentResult>,
        tool_results: Vec<ToolResult>,
        aggregation_method: &str,
        parallel_execution: bool,
        total_execution_time_ms: u64,
    ) -> Result<WorkflowResult> {
        let aggregated_response = Self::perform_aggregation(
            user_input,
            analysis,
            &agent_results,
            &tool_results,
            aggregation_method,
        )?;

        Ok(WorkflowResult {
            user_input: user_input.to_string(),
            analysis: analysis.to_string(),
            agent_results,
            tool_results,
            aggregated_response,
            total_execution_time_ms,
            parallel_execution,
        })
    }

    /// Perform the actual aggregation based on the specified method
    fn perform_aggregation(
        user_input: &str,
        analysis: &str,
        agent_results: &[AgentResult],
        tool_results: &[ToolResult],
        aggregation_method: &str,
    ) -> Result<String> {
        match aggregation_method {
            "智能汇总" | "智能综合" | "intelligent_summary" => {
                Self::intelligent_aggregation(user_input, analysis, agent_results, tool_results)
            }
            "简单拼接" | "simple_concat" => {
                Self::simple_concatenation(agent_results, tool_results)
            }
            "优先级排序" | "priority_based" => {
                Self::priority_based_aggregation(agent_results, tool_results)
            }
            "单一代理" | "single_agent" => Self::single_agent_response(agent_results),
            _ => {
                // 默认使用智能汇总
                Self::intelligent_aggregation(user_input, analysis, agent_results, tool_results)
            }
        }
    }

    /// Intelligent aggregation that synthesizes results contextually
    fn intelligent_aggregation(
        user_input: &str,
        analysis: &str,
        agent_results: &[AgentResult],
        tool_results: &[ToolResult],
    ) -> Result<String> {
        let mut response = String::new();

        // 1. 开始回应用户的输入
        if !analysis.is_empty() {
            response.push_str(&format!(
                "根据你的问题「{}」，我进行了以下分析：\n\n",
                user_input
            ));
            response.push_str(analysis);
            response.push_str("\n\n");
        }

        // 2. 整合智能体回应
        if !agent_results.is_empty() {
            let successful_agents: Vec<_> =
                agent_results.iter().filter(|r| r.error.is_none()).collect();

            match successful_agents.len().cmp(&1) {
                std::cmp::Ordering::Equal => {
                    // 单个智能体直接返回其结果
                    response.push_str(&successful_agents[0].response);
                }
                std::cmp::Ordering::Greater => {
                    // 多个智能体需要综合
                    response.push_str("综合多个智能体的分析：\n\n");
                    for (idx, result) in successful_agents.iter().enumerate() {
                        if result.agent_name == "chat" {
                            // 主对话智能体的回应作为主要内容
                            response.push_str(&result.response);
                            response.push_str("\n\n");
                        } else {
                            // 其他智能体作为补充信息
                            response.push_str(&format!(
                                "{}. {}: {}\n",
                                idx + 1,
                                Self::get_agent_display_name(&result.agent_name),
                                Self::truncate_text(&result.response, 200)
                            ));
                        }
                    }
                }
                std::cmp::Ordering::Less => {
                    // 没有成功的智能体，不做任何操作
                }
            }

            // 报告失败的智能体
            let failed_agents: Vec<_> =
                agent_results.iter().filter(|r| r.error.is_some()).collect();

            if !failed_agents.is_empty() {
                response.push_str("\n⚠️ 部分智能体执行失败:\n");
                for result in failed_agents {
                    response.push_str(&format!(
                        "- {}: {}\n",
                        Self::get_agent_display_name(&result.agent_name),
                        result.error.as_ref().unwrap_or(&"未知错误".to_string())
                    ));
                }
            }
        }

        // 3. 整合工具结果
        if !tool_results.is_empty() {
            response.push_str("\n🔧 工具执行结果:\n");
            let successful_tools: Vec<_> =
                tool_results.iter().filter(|r| r.error.is_none()).collect();

            for result in successful_tools {
                response.push_str(&format!(
                    "- {}: {}\n",
                    result.tool_name,
                    Self::format_tool_result(&result.result)
                ));
            }

            let failed_tools: Vec<_> = tool_results.iter().filter(|r| r.error.is_some()).collect();

            if !failed_tools.is_empty() {
                response.push_str("\n⚠️ 部分工具执行失败:\n");
                for result in failed_tools {
                    response.push_str(&format!(
                        "- {}: {}\n",
                        result.tool_name,
                        result.error.as_ref().unwrap_or(&"未知错误".to_string())
                    ));
                }
            }
        }

        // 4. 如果没有任何结果，提供默认回应
        if response.trim().is_empty() {
            response = format!("我已收到你的问题：「{}」\n\n很抱歉，暂时无法提供详细回应。请稍后再试或重新表述你的问题。", user_input);
        }

        Ok(response.trim().to_string())
    }

    /// Simple concatenation of all results
    fn simple_concatenation(
        agent_results: &[AgentResult],
        tool_results: &[ToolResult],
    ) -> Result<String> {
        let mut response = String::new();

        // 拼接智能体结果
        for result in agent_results {
            if result.error.is_none() {
                response.push_str(&format!(
                    "[{}] {}\n\n",
                    Self::get_agent_display_name(&result.agent_name),
                    result.response
                ));
            }
        }

        // 拼接工具结果
        for result in tool_results {
            if result.error.is_none() {
                response.push_str(&format!(
                    "[{}] {}\n\n",
                    result.tool_name,
                    Self::format_tool_result(&result.result)
                ));
            }
        }

        Ok(response.trim().to_string())
    }

    /// Priority-based aggregation (highest priority first)
    fn priority_based_aggregation(
        agent_results: &[AgentResult],
        tool_results: &[ToolResult],
    ) -> Result<String> {
        // 简化实现：按agent名称优先级排序
        let mut sorted_agents = agent_results.to_vec();
        sorted_agents.sort_by(|a, b| {
            let priority_a = Self::get_agent_priority(&a.agent_name);
            let priority_b = Self::get_agent_priority(&b.agent_name);
            priority_a.cmp(&priority_b)
        });

        let mut response = String::new();

        // 优先级最高的智能体结果作为主要内容
        if let Some(primary_result) = sorted_agents.first() {
            if primary_result.error.is_none() {
                response.push_str(&primary_result.response);
            }
        }

        // 其他结果作为补充
        if sorted_agents.len() > 1 {
            response.push_str("\n\n📋 补充信息:\n");
            for result in sorted_agents.iter().skip(1) {
                if result.error.is_none() {
                    response.push_str(&format!(
                        "• {}: {}\n",
                        Self::get_agent_display_name(&result.agent_name),
                        Self::truncate_text(&result.response, 150)
                    ));
                }
            }
        }

        // 添加工具结果
        if !tool_results.is_empty() {
            response.push_str("\n🔧 工具结果:\n");
            for result in tool_results {
                if result.error.is_none() {
                    response.push_str(&format!(
                        "• {}: {}\n",
                        result.tool_name,
                        Self::format_tool_result(&result.result)
                    ));
                }
            }
        }

        Ok(response.trim().to_string())
    }

    /// Single agent response (return the first successful agent response)
    fn single_agent_response(agent_results: &[AgentResult]) -> Result<String> {
        for result in agent_results {
            if result.error.is_none() {
                return Ok(result.response.clone());
            }
        }
        Ok("所有智能体执行失败".to_string())
    }

    /// Get display name for agent
    fn get_agent_display_name(agent_name: &str) -> &str {
        match agent_name {
            "chat" => "对话助手",
            "conversation_summarizer" => "对话总结",
            "persona_extractor" => "画像分析",
            "router" => "路由智能体",
            _ => agent_name,
        }
    }

    /// Get agent priority (lower number = higher priority)
    fn get_agent_priority(agent_name: &str) -> u8 {
        match agent_name {
            "chat" => 1,
            "conversation_summarizer" => 3,
            "persona_extractor" => 4,
            "router" => 2,
            _ => 99,
        }
    }

    /// Format tool result for display
    fn format_tool_result(result: &serde_json::Value) -> String {
        match result {
            serde_json::Value::String(s) => s.clone(),
            serde_json::Value::Object(_) => {
                // 尝试提取常见字段
                if let Some(message) = result.get("message").and_then(|v| v.as_str()) {
                    message.to_string()
                } else if let Some(result_val) = result.get("result").and_then(|v| v.as_str()) {
                    result_val.to_string()
                } else {
                    serde_json::to_string_pretty(result)
                            .unwrap_or_else(|_| "Invalid JSON".to_string()).to_string()
                }
            }
            _ => result.to_string(),
        }
    }

    /// Truncate text to specified length
    fn truncate_text(text: &str, max_length: usize) -> String {
        if text.len() <= max_length {
            text.to_string()
        } else {
            format!("{}...", &text[..max_length])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intelligent_aggregation() {
        let agent_results = vec![AgentResult {
            agent_name: "chat".to_string(),
            response: "这是对话助手的回应".to_string(),
            error: None,
            execution_time_ms: 100,
        }];

        let tool_results = vec![];

        let result = WorkflowAggregator::aggregate_results(
            "你好",
            "用户打招呼",
            agent_results,
            tool_results,
            "智能汇总",
            false,
            200,
        )
        .unwrap();

        assert!(result.aggregated_response.contains("这是对话助手的回应"));
    }

    #[test]
    fn test_single_agent_response() {
        let agent_results = vec![AgentResult {
            agent_name: "chat".to_string(),
            response: "单一回应".to_string(),
            error: None,
            execution_time_ms: 100,
        }];

        let result = WorkflowAggregator::perform_aggregation(
            "测试",
            "分析",
            &agent_results,
            &[],
            "单一代理",
        )
        .unwrap();

        assert_eq!(result, "单一回应");
    }
}

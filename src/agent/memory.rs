//! 智能体记忆模块

use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::RwLock;

/// 智能体记忆管理器
#[derive(Debug, Clone)]
pub struct Memory {
    /// 最近的对话历史 (user, assistant) 对
    history: Arc<RwLock<VecDeque<(String, String)>>>,
}

impl Memory {
    /// 创建新的记忆实例
    pub fn new() -> Self {
        Self {
            history: Arc::new(RwLock::new(VecDeque::new())),
        }
    }

    /// 添加一轮对话到历史记录
    pub async fn add_conversation(&self, user_input: String, assistant_response: String) {
        let mut history = self.history.write().await;

        // 使用正则表达式过滤掉``</think>``标签之前的内容
        let filtered_response = self.filter_thought_content(&assistant_response);

        // 添加新的对话
        history.push_back((user_input, filtered_response));

        // 保持最多5轮对话
        while history.len() > 5 {
            history.pop_front();
        }
    }

    /// 使用字符串查找过滤掉思考内容标签之前的内容
    pub fn filter_thought_content(&self, response: &str) -> String {
        // 支持多种思考内容标签格式：

        // </thinks>标签 - 获取最后一个</think>之后的内容
        if let Some(pos) = response.rfind("</think>") {
            return response[pos + 8..].to_string(); // 9 is length of "</thinks>"
        }

        // 如果没有匹配到任何标签，返回原内容
        response.to_string()
    }

    /// 获取格式化的历史对话记录
    pub async fn get_history(&self) -> String {
        let history = self.history.read().await;

        if history.is_empty() {
            return "".to_string();
        }

        let mut formatted_history = String::new();
        for (user_msg, assistant_msg) in history.iter() {
            formatted_history.push_str(&format!(
                "User: {}\n\nAssistant: {}\n\n",
                user_msg,
                assistant_msg.replace("\n", "")
            ));
        }

        // 移除最后的换行符
        if formatted_history.ends_with('\n') {
            formatted_history.pop();
        }

        formatted_history
    }

    /// 清空历史记录
    pub async fn clear_history(&self) {
        let mut history = self.history.write().await;
        history.clear();
    }

    /// 获取历史记录数量
    pub async fn history_count(&self) -> usize {
        let history = self.history.read().await;
        history.len()
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}

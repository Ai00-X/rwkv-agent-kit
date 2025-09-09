//! 核心服务模块

use ai00_core::{GenerateKind, GenerateRequest, ThreadRequest, Token};
use flume::Sender;
use std::collections::HashMap;
use std::sync::Arc;
use web_rwkv::tokenizer::Tokenizer;

use super::{
    error::{ErrorCode, RwkvError, RwkvResult},
    error_handler::ErrorHandler,
    tools::{SharedToolRegistry, ToolRegistry},
    KitConfig,
};
use crate::agent::prompt::PromptBuilder;
use crate::agent::Agent;
use crate::db::DatabaseManager;

/// 主服务结构
pub struct RwkvAgentKit {
    pub config: KitConfig,
    pub sender: Sender<ThreadRequest>,
    pub agents: HashMap<String, Agent>,
    pub tools: SharedToolRegistry,
    pub tokenizer: Arc<Tokenizer>,
    pub database_manager: Option<Arc<DatabaseManager>>,
    /// 错误处理器
    pub error_handler: Arc<ErrorHandler>,
}

impl std::fmt::Debug for RwkvAgentKit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RwkvAgentKit")
            .field("config", &self.config)
            .field(
                "agents",
                &format!("HashMap with {} agents", self.agents.len()),
            )
            .field("tools", &"SharedToolRegistry")
            .field("database_manager", &self.database_manager.is_some())
            .field("error_handler", &"ErrorHandler")
            .finish()
    }
}

impl RwkvAgentKit {
    /// 创建新的RWKV Agent Kit实例
    pub async fn new(config: KitConfig) -> RwkvResult<Self> {
        let error_handler = Arc::new(ErrorHandler::default());

        let (sender, receiver) = flume::unbounded::<ThreadRequest>();

        // 启动ai00-core服务
        tokio::spawn(async move {
            println!("🚀 启动ai00-core服务...");

            // 设置日志级别 - 关闭调试信息
            env_logger::Builder::from_default_env()
                .filter_level(log::LevelFilter::Error)
                .filter_module("ai00_core", log::LevelFilter::Error)
                .filter_module("web_rwkv", log::LevelFilter::Error)
                .try_init()
                .ok(); // 忽略重复初始化错误

            ai00_core::serve(receiver).await;
            println!("✅ ai00-core服务退出");
        });

        // 加载分词器
        let tokenizer = error_handler
            .execute_with_retry(
                || async {
                    let tokenizer_content = std::fs::read_to_string(&config.model.tokenizer_path)
                        .map_err(|e| {
                        RwkvError::new(ErrorCode::ModelLoadFailed, format!("分词器加载失败: {}", e))
                    })?;

                    Tokenizer::new(&tokenizer_content).map_err(|e| {
                        RwkvError::new(
                            ErrorCode::ModelLoadFailed,
                            format!("分词器初始化失败: {}", e),
                        )
                    })
                },
                "tokenizer_loading",
            )
            .await?;

        // 加载模型
        let reload_result = error_handler
            .execute_with_retry(
                || async {
                    let reload_request = config.model.clone().try_into().map_err(|e| {
                        RwkvError::new(
                            ErrorCode::ModelLoadFailed,
                            format!("Model config conversion failed: {}", e),
                        )
                    })?;

                    let (reload_sender, reload_receiver) = flume::unbounded();
                    let request = ThreadRequest::Reload {
                        request: Box::new(reload_request),
                        sender: Some(reload_sender),
                    };

                    sender.send_async(request).await.map_err(|e| {
                        RwkvError::new(
                            ErrorCode::ModelLoadFailed,
                            format!("Model reload request failed: {}", e),
                        )
                    })?;

                    // 等待模型加载完成
                    reload_receiver.recv_async().await.map_err(|e| {
                        RwkvError::new(
                            ErrorCode::ModelLoadFailed,
                            format!("Model reload response failed: {}", e),
                        )
                    })
                },
                "model_loading",
            )
            .await?;

        println!("✅ 模型加载完成，结果: {:?}", reload_result);

        // 初始化工具注册表
        let tools = Arc::new(tokio::sync::RwLock::new(ToolRegistry::default()));

        // 注册智能体
        let mut agents = HashMap::new();
        for agent_config in config.agents.clone() {
            let agent = Agent::new(agent_config.clone(), &config.model).map_err(|e| {
                RwkvError::new(
                    ErrorCode::AgentRegistrationFailed,
                    format!("智能体初始化失败: {}", e),
                )
            })?;
            agents.insert(agent_config.name, agent);
        }

        Ok(Self {
            config,
            sender,
            agents,
            tools,
            tokenizer: Arc::new(tokenizer),
            database_manager: None,
            error_handler,
        })
    }

    /// 注册新的智能体
    pub fn register_agent(&mut self, agent_config: crate::agent::AgentConfig) -> RwkvResult<()> {
        let agent = Agent::new(agent_config.clone(), &self.config.model).map_err(|e| {
            RwkvError::new(
                ErrorCode::AgentRegistrationFailed,
                format!("智能体注册失败: {}", e),
            )
        })?;
        self.agents.insert(agent_config.name, agent);
        Ok(())
    }

    /// 注册工具
    pub async fn register_tool<T: crate::core::Tool + 'static>(&self, tool: T) -> RwkvResult<()> {
        let mut tools = self.tools.write().await;
        tools.register(tool);
        println!(
            "🔧 工具 '{}' 已注册",
            tools.tools.keys().last().unwrap_or(&"unknown".to_string())
        );
        Ok(())
    }

    /// 获取工具列表
    pub async fn list_tools(&self) -> Vec<String> {
        let tools = self.tools.read().await;
        tools.list_tools()
    }

    /// 执行工具
    pub async fn execute_tool(
        &self,
        tool_name: &str,
        params: serde_json::Value,
    ) -> RwkvResult<serde_json::Value> {
        self.error_handler
            .execute_with_retry(
                || async {
                    let tools = self.tools.read().await;
                    tools
                        .execute_tool(tool_name, params.clone())
                        .await
                        .map_err(|e| {
                            RwkvError::new(
                                ErrorCode::AgentToolError,
                                format!("工具执行失败: {}", e),
                            )
                        })
                },
                &format!("tool_execution_{}", tool_name),
            )
            .await
    }

    /// 与指定智能体聊天
    pub async fn chat(&self, agent_name: &str, user_input: &str) -> RwkvResult<String> {
        self.chat_with_options(agent_name, user_input, None, None)
            .await
    }

    /// 与指定智能体聊天（可指定 BNF schema 和停止序列）
    pub async fn chat_with_options(
        &self,
        agent_name: &str,
        user_input: &str,
        bnf_schema: Option<String>,
        stop_sequences: Option<Vec<String>>,
    ) -> RwkvResult<String> {
        self.error_handler.execute_with_retry(
            || {
                let bnf_schema = bnf_schema.clone();
                let stop_sequences = stop_sequences.clone();
                async move {
                let agent = self.agents.get(agent_name)
                    .ok_or_else(|| RwkvError::new(
                        ErrorCode::AgentNotFound,
                        format!("Agent '{}' not found", agent_name)
                    ))?;

        // 使用智能体的提示词构建器构建提示词
        let prompt_builder = agent.config.get_prompt_builder();
        // 获取工具列表用于提示词构建
        let available_tools = self.list_tools().await;
        let mut prompt = prompt_builder.build_prompt(agent, user_input, &available_tools).await?;

        // 去掉关联记忆 memory 段（无论模板是否包含该占位符，都将其移除）
        {
            let cleaned = prompt
                .lines()
                .filter(|line| {
                    let t = line.trim_start();
                    // 过滤形如 "memory: [...]" 的整行
                    !(t.starts_with("memory:"))
                })
                .collect::<Vec<_>>()
                .join("\n");
            prompt = cleaned;
        }

        // 阶段4.3: 已禁用关联记忆注入（按需去除 memory 段）
        // ... existing code ...

         // 阶段 4.4: 历史对话注入（仅对没有自定义 PromptBuilder 的 agent）
        // 如果 agent 有自定义 PromptBuilder（如 ChatPromptBuilder），则跳过此处的历史注入
        // 因为自定义 PromptBuilder 会在 build_prompt 中处理历史记录
        if agent.config.memory.enabled && agent.config.prompt_builder.is_none() {
            // 使用Agent自己的Memory系统获取历史记录
            let history_text = agent.memory().get_history().await;

            if !history_text.is_empty() {
                // 注入到 prompt
                let history_marker = "history: [无]";
                if prompt.contains(history_marker) {
                    let history_injection = format!("history: [\n{}\n]", history_text);
                    prompt = prompt.replacen(history_marker, &history_injection, 1);
                } else {
                    prompt = format!("[Recent Conversation History]\n{}\n\n{}", history_text, prompt);
                }
            } else if let Some(db_manager) = &self.database_manager {
                // 回退：从数据库读取最近5轮（10条）
                match db_manager.get_active_session().await {
                    Ok(Some(session_id)) => {
                        match db_manager.list_memory_events(session_id).await {
                            Ok(memory_events) => {
                                if !memory_events.is_empty() {
                                    let max_events = 10usize; // 5轮=10条
                                    let recent_events: Vec<_> = memory_events.iter()
                                        .rev()
                                        .take(max_events)
                                        .collect();
                                    let history_text = recent_events.iter()
                                        .rev()
                                        .map(|event| format!("{}: {}", event.role, event.text))
                                        .collect::<Vec<_>>()
                                        .join("\n");
                                    let history_marker = "history: [无]";
                                    if prompt.contains(history_marker) {
                                        let history_injection = format!("history: [\n{}\n]", history_text);
                                        prompt = prompt.replacen(history_marker, &history_injection, 1);
                                    } else {
                                        prompt = format!("[Recent Conversation History]\n{}\n\n{}", history_text, prompt);
                                    }
                                }
                            }
                            Err(e) => {
                                log::warn!("Failed to retrieve memory events for session {}: {}", session_id, e);
                            }
                        }
                    }
                    Ok(None) => {
                        log::debug!("No active session found for history retrieval");
                    }
                    Err(e) => {
                        log::warn!("Failed to get active session: {}", e);
                    }
                }
            }
        }

        // 阶段4.5: 画像特征注入（如果启用）
        if agent.config.memory.enabled {
            if let Some(db_manager) = &self.database_manager {
                match db_manager.list_persona_traits(agent_name, None, Some(5)).await {
                    Ok(persona_traits) => {
                        if !persona_traits.is_empty() {
                            let features_text = persona_traits.iter()
                                .map(|trait_item| format!("{}:{}", trait_item.trait_key, trait_item.trait_value))
                                .collect::<Vec<_>>()
                                .join("; ");

                            // 检查是否有 features 占位符
                            let features_marker = "features: [无]";
                            if prompt.contains(features_marker) {
                                let features_injection = format!("features: [{}]", features_text);
                                prompt = prompt.replacen(features_marker, &features_injection, 1);
                            } else {
                                // 在记忆上下文后添加用户特征
                                prompt = format!("{}\n[User Features]: {}", prompt, features_text);
                            }
                        }
                    }
                    Err(e) => {
                        log::warn!("Failed to retrieve persona traits for agent {}: {}", agent_name, e);
                    }
                }
            }
        }

        // 阶段4.6：工具调用计划占位符（由 router 智能体处理工具执行）
        let tools_list = if available_tools.is_empty() { "无".to_string() } else { available_tools.join(", ") };
        prompt = prompt.replace("tool_call: [待规划]", &format!("tool_call: [可用工具: {}]", tools_list));

        // 阶段 5: 推理请求
        // 应用传入的 bnf_schema 和 stop_sequences（如果提供），否则使用 agent 默认配置
        let final_stop_sequences = stop_sequences.unwrap_or_else(|| agent.config.inference_params.stop_sequences.clone());

        let generate_request = GenerateRequest {
            prompt: prompt.clone(),
            model_text: String::new(),
            max_tokens: agent.config.inference_params.max_tokens,
            stop: final_stop_sequences,
            bias: Arc::new(HashMap::new()),
            bnf_schema,
            sampler: agent.sampler.clone(),
            kind: GenerateKind::None,
            state: agent.state.clone(),
        };

        // 调试输出：打印完整提示词
        println!("\n=== 完整提示词调试输出 ===");
        println!("Agent: {}", agent_name);
        println!("User Input: {}", user_input);
        println!("Stop Sequences: {:?}", generate_request.stop);
        println!("Max Tokens: {}", generate_request.max_tokens);
        println!("BNF Schema: {:?}", generate_request.bnf_schema);
        println!("完整提示词内容:");
        println!("{}", prompt);
        println!("=== 提示词调试输出结束 ===\n");

        // 发送推理请求
        let (result_sender, result_receiver) = flume::unbounded();
        let request = ThreadRequest::Generate {
            request: Box::new(generate_request),
            tokenizer: self.tokenizer.clone(),
            sender: result_sender,
        };

        self.sender.send_async(request).await
            .map_err(|e| RwkvError::new(
                ErrorCode::ModelInferenceFailed,
                format!("Failed to send request: {}", e)
            ))?;

        // 阶段 6: 收集响应结果
        let mut result = String::new();

        while let Ok(token) = result_receiver.recv_async().await {
            match &token {
                Token::Content(content) => {
                    result.push_str(content);
                },
                Token::Stop(_reason, _counter) => {
                    break;
                },
                _ => {
                    // 处理其他类型的token
                }
            }
        }

        // 阶段 7: 使用Agent自己的Memory系统保存对话
        if agent.config.memory.enabled {
            agent.memory().add_conversation(user_input.to_string(), result.clone()).await;
        }

        // 注意：数据库保存逻辑已移至 rwkv_agent_kit.rs 的 chat_with_memory 方法中
        // 避免重复保存，这里不再直接保存到数据库
        // if agent.config.save_conversations { ... } // 已注释掉以避免重复保存

                Ok(result)
            }
            },
            &format!("chat_{}", agent_name)
        ).await
    }

    /// 设置数据库管理器
    pub fn set_database_manager(&mut self, dbm: Arc<DatabaseManager>) {
        self.database_manager = Some(dbm);
    }

    /// 设置工具注册表
    pub fn set_tools(&mut self, tools: SharedToolRegistry) {
        self.tools = tools;
    }
}

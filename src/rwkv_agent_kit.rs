//! RWKV Agent Kit 主类
//!
//! 提供一个统一的入口点，自动初始化和管理所有核心组件：
//! - RWKV LLM 推理引擎
//! - 嵌入模型
//! - 统一数据库系统（向量、图、关系型数据库）

use anyhow::Result;
use chrono;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{
    agent::AgentConfig,
    core::{
        error_handler::ErrorHandler,
        rwkv_singleton::{
            get_global_rwkv_service, is_global_rwkv_service_initialized,
            preload_global_rwkv_service_with_config,
        },
        service::RwkvAgentKit as CoreService,
        tools::{SharedToolRegistry, ToolRegistry},
    },
    db::{
        embedding::{
            get_global_embedding_service, initialize_global_embedding_service, EmbeddingService,
        },
        DatabaseConfig, DatabaseManager,
    },
    rwkv::config::{BnfConfig, ModelConfig},
};

/// RWKV Agent Kit 主类
///
/// 这是整个工具包的核心入口点，负责：
/// 1. 自动启动 RWKV LLM 推理引擎
/// 2. 初始化嵌入模型
/// 3. 设置统一的数据库系统
/// 4. 管理智能体生命周期
#[derive(Debug)]
pub struct RwkvAgentKit {
    /// 核心服务实例（全局单例）
    pub core_service: Arc<RwLock<CoreService>>,
    /// 数据库管理器
    pub database_manager: DatabaseManager,
    /// 工具注册表
    pub tools: SharedToolRegistry,
    /// 配置信息
    pub config: RwkvAgentKitConfig,
    /// 智能体配置映射 (智能体名称 -> 配置)
    pub agent_configs: HashMap<String, AgentConfig>,
    /// 错误处理器
    pub error_handler: Arc<ErrorHandler>,
}

/// RWKV Agent Kit 配置
#[derive(Debug, Clone)]
#[derive(Default)]
pub struct RwkvAgentKitConfig {
    /// 模型配置
    pub model: ModelConfig,
    /// 数据库配置
    pub database: DatabaseConfig,
    /// 智能体配置列表
    pub agents: Vec<AgentConfig>,
}


impl RwkvAgentKit {
    /// 创建新的 RWKV Agent Kit 实例（内部使用）
    ///
    /// 这个方法会自动完成所有初始化工作：
    /// 1. 预加载嵌入模型
    /// 2. 初始化数据库系统
    /// 3. 启动 RWKV 推理引擎
    /// 4. 设置记忆管理器
    /// 5. 注册默认智能体
    async fn new(config: RwkvAgentKitConfig) -> Result<Self> {
        println!("🚀 正在启动 RWKV Agent Kit...");

        // 1. 初始化数据库管理器
        println!("🗄️ 正在初始化数据库系统...");
        let database_manager = DatabaseManager::new(config.database.clone())
            .await
            .map_err(|e| anyhow::anyhow!("Database initialization error: {}", e))?;
        println!("✅ 数据库系统初始化完成");

        // 2. 初始化完成
        println!("✅ 数据库初始化完成");

        // 2.5 初始化嵌入服务（仅在至少有一个 agent 开启记忆时）
        let memory_enabled = config.agents.iter().any(|a| a.memory.enabled);
        if memory_enabled {
            // 目前使用默认路径，可扩展为从配置传入
            if let Err(e) = initialize_global_embedding_service(None).await {
                eprintln!("⚠️ 嵌入服务初始化失败，将退回词重叠检索: {}", e);
            } else {
                println!("✅ 嵌入服务已初始化");
            }
        }

        // 3. 初始化全局 RWKV 服务（如果尚未初始化）
        println!("🤖 正在初始化全局 RWKV 服务...");
        if !is_global_rwkv_service_initialized() {
            preload_global_rwkv_service_with_config(config.model.clone()).await?;
        }
        let core_service_arc = get_global_rwkv_service()?;

        // 注册智能体到全局服务
        {
            let mut core_service_write = core_service_arc.write().await;
            for agent_config in &config.agents {
                core_service_write.register_agent(agent_config.clone())?;
            }
            // 注入数据库管理器，供核心服务进行记忆检索
            core_service_write.set_database_manager(Arc::new(database_manager.clone()));
        }

        // 获取核心服务的克隆用于存储
        let core_service = core_service_arc.clone();
        println!("✅ 全局 RWKV 服务初始化完成");

        // 7. 初始化工具注册表（共享）
        let tools: SharedToolRegistry = Arc::new(tokio::sync::RwLock::new(ToolRegistry::default()));

        // 将共享工具注册表注入核心服务
        {
            let mut core_service_write = core_service_arc.write().await;
            core_service_write.set_tools(tools.clone());
        }

        println!("🎉 RWKV Agent Kit 启动完成！");

        // 8. 初始化智能体配置映射
        let mut agent_configs = HashMap::new();
        for agent_config in &config.agents {
            agent_configs.insert(agent_config.name.clone(), agent_config.clone());
        }

        // 8.1 如果没有活跃会话，则为主对话 agent 打开一个默认会话
        match database_manager.get_active_session().await {
            Ok(Some(_)) => {
                // 已有活跃会话，保持不变
            }
            _ => {
                // 优先选择配置中 save_conversations=true 的 agent 作为主对话 agent（通常是 "chat"）
                let main_agent_name = config
                    .agents
                    .iter()
                    .find(|a| a.save_conversations)
                    .map(|a| a.name.clone())
                    // 如果没有显式主对话 agent，则回退到第一个已注册的 agent
                    .or_else(|| config.agents.first().map(|a| a.name.clone()));
                if let Some(agent_name) = main_agent_name {
                    let title = format!(
                        "{} 会话 @ {}",
                        agent_name,
                        chrono::Local::now().format("%Y-%m-%d %H:%M:%S")
                    );
                    if let Err(e) = database_manager
                        .open_session(&agent_name, Some(&title))
                        .await
                    {
                        eprintln!("⚠️ 启动默认会话失败: {}", e);
                    } else {
                        println!("🗣️ 已为主对话智能体 '{}' 启动默认会话", agent_name);
                    }
                }
            }
        }

        let error_handler = Arc::new(ErrorHandler::default());

        Ok(Self {
            core_service,
            database_manager,
            tools,
            config,
            agent_configs,
            error_handler,
        })
    }

    /// 注册新的智能体
    pub async fn register_agent(&mut self, agent_config: AgentConfig) -> Result<()> {
        println!("📝 正在注册智能体: {}", agent_config.name);
        {
            let mut core_service = self.core_service.write().await;
            core_service.register_agent(agent_config.clone())?;
        }
        // 更新智能体配置映射
        self.agent_configs
            .insert(agent_config.name.clone(), agent_config.clone());
        self.config.agents.push(agent_config);
        println!("✅ 智能体注册完成");
        Ok(())
    }

    /// 与指定智能体进行对话
    pub async fn chat(&mut self, agent_name: &str, user_input: &str) -> Result<String> {
        self.chat_with_memory(agent_name, user_input, true).await
    }

    /// 与 chat 智能体进行对话，并指定昵称
    pub async fn chat_with_nick(&mut self, user_input: &str, agent_nick: &str) -> Result<String> {
        // 检查是否有 chat agent
        if !self.agent_configs.contains_key("chat") {
            return Err(anyhow::anyhow!(
                "Chat agent not found. Please register a chat agent first."
            ));
        }

        // 临时创建一个带有自定义昵称的 ChatPromptBuilder
        use crate::agent::prompt::PromptBuilderInstance;
        use crate::agents::ChatPromptBuilder;
        let chat_builder = ChatPromptBuilder::with_nick(agent_nick);
        let custom_prompt_builder = Arc::new(PromptBuilderInstance::Chat(chat_builder));

        // 获取原始的 chat agent 配置
        let original_config = self.agent_configs.get("chat").unwrap().clone();

        // 创建临时配置，替换 prompt_builder
        let mut temp_config = original_config.clone();
        temp_config.prompt_builder = Some(custom_prompt_builder);

        // 临时注册这个配置到核心服务
        {
            let mut core_service = self.core_service.write().await;
            core_service.register_agent(temp_config)?;
        }

        // 进行对话
        let result = self.chat_with_memory("chat", user_input, true).await;

        // 恢复原始配置
        {
            let mut core_service = self.core_service.write().await;
            core_service.register_agent(original_config)?;
        }

        result
    }

    /// 与指定智能体进行对话（不存储记忆，用于内部处理）
    pub async fn chat_no_memory(&mut self, agent_name: &str, user_input: &str) -> Result<String> {
        self.chat_no_memory_with_options(agent_name, user_input, None, None)
            .await
    }

    /// 与指定智能体进行对话（不存储记忆，用于内部处理，可选 bnf_schema 和 stop）
    pub async fn chat_no_memory_with_options(
        &mut self,
        agent_name: &str,
        user_input: &str,
        bnf_schema: Option<String>,
        stop_sequences: Option<Vec<String>>,
    ) -> Result<String> {
        // 直接使用传入的参数，让各个agent自己在配置中定义所需的KBNF schema
        let (schema, stops) = (bnf_schema, stop_sequences);

        // 直接调用核心服务，避免与 chat_with_memory 形成递归
        let response = {
            let core_service = self.core_service.read().await;
            core_service
                .chat_with_options(agent_name, user_input, schema, stops)
                .await?
        };
        Ok(response)
    }

    /// 与指定智能体进行对话的内部实现
    async fn chat_with_memory(
        &mut self,
        agent_name: &str,
        user_input: &str,
        store_memory: bool,
    ) -> Result<String> {
        // 使用核心服务进行对话
        let response = {
            let core_service = self.core_service.read().await;

            // 调试：显示发送给LLM的完整提示词 - 已禁用
            // 注意：这里我们需要重新构建提示词来显示，因为core_service.chat内部构建了提示词
            // if let Some(agent) = core_service.agents.get(agent_name) {
            //     let prompt_builder = agent.config.get_prompt_builder();
            //     let available_tools = core_service.list_tools().await;
            //     if let Ok(mut debug_prompt) = prompt_builder.build_prompt(agent, user_input, &available_tools).await {
            //         // 应用与core_service.chat相同的提示词处理逻辑
            //         debug_prompt = debug_prompt
            //             .lines()
            //             .filter(|line| {
            //                 let t = line.trim_start();
            //                 !(t.starts_with("memory:"))
            //             })
            //             .collect::<Vec<_>>()
            //             .join("\n");
            //
            //         println!("\n🔍 ===== 发送给LLM的完整提示词 =====");
            //         println!("{}", debug_prompt);
            //         println!("🔍 ===== 提示词结束 =====\n");
            //     }
            // }

            core_service.chat(agent_name, user_input).await?
        };

        // 调试：显示AI的完整回复内容
        println!("\n🤖 ===== AI完整回复 =====");
        println!("{}", response);
        println!("🤖 ===== 回复结束 =====\n");

        // 检查智能体配置是否允许保存对话
        let should_save = if let Some(agent_config) = self.agent_configs.get(agent_name) {
            store_memory && agent_config.save_conversations
        } else {
            store_memory // 如果找不到配置，使用默认行为
        };

        // 如果需要存储记忆且智能体配置允许，则保存对话到数据库
        if should_save {
            // 检查回复是否为空，如果为空则不保存
            if response.trim().is_empty() {
                println!("⚠️ AI回复为空，跳过保存到数据库");
                return Ok(response);
            }

            // 获取或创建活跃会话
            let session_id = match self.database_manager.get_active_session().await {
                Ok(Some(id)) => id,
                _ => {
                    // 打开一个新的会话，标题可设为时间戳或agent_name
                    let title = format!(
                        "{} 会话 @ {}",
                        agent_name,
                        chrono::Local::now().format("%Y-%m-%d %H:%M:%S")
                    );
                    match self
                        .database_manager
                        .open_session(agent_name, Some(&title))
                        .await
                    {
                        Ok(id) => id,
                        Err(e) => {
                            eprintln!("⚠️ 打开会话失败: {}", e);
                            -1
                        }
                    }
                }
            };

            if session_id >= 0 {
                use crate::db::MemoryEvent;
                // 可用的嵌入服务（如果初始化成功）
                let embedding_service = get_global_embedding_service().ok();

                // 先写入用户消息
                let mut user_event = MemoryEvent {
                    session_id,
                    agent_name: agent_name.to_string(),
                    role: "user".to_string(),
                    text: user_input.to_string(),
                    topic: None,
                    sentiment: None,
                    importance: None,
                    decay: 1.0,
                    embedding: None,
                };
                if let Some(svc) = embedding_service.as_ref() {
                    if let Ok(emb) = svc.lock().await.encode_single(user_input).await {
                        if let Ok(bytes) = EmbeddingService::serialize_embedding(&emb) {
                            user_event.embedding = Some(bytes);
                        }
                    }
                }
                // 记录用户事件ID，供画像提取来源引用
                #[allow(unused_variables)] // 预留功能，后续启用
                let mut _last_user_event_id: Option<i64> = None;
                #[allow(unused_assignments)] // 预留功能，后续启用
                match self.database_manager.insert_memory_event(user_event).await {
                    Ok(id) => {
                        _last_user_event_id = Some(id);
                    }
                    Err(e) => {
                        eprintln!("⚠️ 保存用户消息失败: {}", e);
                    }
                }

                // 再写入助手回复
                let mut assistant_event = MemoryEvent {
                    session_id,
                    agent_name: agent_name.to_string(),
                    role: "assistant".to_string(),
                    text: response.clone(),
                    topic: None,
                    sentiment: None,
                    importance: None,
                    decay: 1.0,
                    embedding: None,
                };
                if let Some(svc) = embedding_service.as_ref() {
                    if let Ok(emb) = svc.lock().await.encode_single(&response).await {
                        if let Ok(bytes) = EmbeddingService::serialize_embedding(&emb) {
                            assistant_event.embedding = Some(bytes);
                        }
                    }
                }
                if let Err(e) = self
                    .database_manager
                    .insert_memory_event(assistant_event)
                    .await
                {
                    eprintln!("⚠️ 保存助手回复失败: {}", e);
                } else {
                    println!("💾 对话轮次已保存到 memory_events");
                }

                // === 阶段3：语义片段聚合 ===
                // 在成功保存事件后，检查是否需要创建语义片段
                // 暂时禁用语义片段功能
                // if let Err(e) = self.trigger_semantic_aggregation(session_id, user_input, &response).await {
                //     eprintln!("⚠️ 语义聚合失败: {}", e);
                // }

                // 画像提取功能已移除
            }
        }

        Ok(response
            .trim_start_matches(|c: char| c.is_whitespace())
            .trim_end_matches(|c: char| c.is_whitespace())
            .to_string())
    }

    /// 触发语义聚合：使用 ConversationSummarizer 分析当前对话轮次
    #[allow(dead_code)] // 预留功能，后续启用
    async fn trigger_semantic_aggregation(
        &mut self,
        session_id: i64,
        user_input: &str,
        assistant_response: &str,
    ) -> Result<()> {
        // 1. 构建对话历史（当前轮次）
        let conversation_history =
            format!("User: {}\nAssistant: {}", user_input, assistant_response);

        // 2. 调用 ConversationSummarizer 分析对话
        let summarizer_result = match self
            .chat_no_memory("conversation_summarizer", &conversation_history)
            .await
        {
            Ok(result) => result,
            Err(e) => {
                eprintln!("⚠️ ConversationSummarizer 调用失败: {}", e);
                return Ok(()); // 不阻断主流程
            }
        };

        // 3. 解析 JSON 响应
        let parsed_summary = match self.parse_summarizer_response(&summarizer_result) {
            Ok(summary) => summary,
            Err(e) => {
                eprintln!("⚠️ ConversationSummarizer 响应解析失败: {}", e);
                eprintln!("原始响应: {}", summarizer_result);
                return Ok(()); // 不阻断主流程
            }
        };

        // 4. 获取配置化阈值（从任一智能体的记忆配置中获取，默认为5）
        let semantic_threshold = self
            .agent_configs
            .values()
            .find(|config| config.memory.enabled)
            .map(|config| config.memory.semantic_chunk_threshold)
            .unwrap_or(5);

        // 检查重要性阈值（importance_score >= threshold 才创建语义片段）
        if parsed_summary.importance_score < semantic_threshold {
            println!(
                "📊 对话重要性较低 ({}), 跳过语义片段创建",
                parsed_summary.importance_score
            );
            return Ok(());
        }

        // 5. 创建语义片段
        use crate::db::SemanticChunk;
        let embedding_service = get_global_embedding_service().ok();

        let mut semantic_chunk = SemanticChunk {
            id: None,
            title: if parsed_summary.user_intent_summary.is_empty() {
                None
            } else {
                Some(parsed_summary.user_intent_summary.clone())
            },
            summary: parsed_summary.summary.clone(),
            keywords: if parsed_summary.memory_triggers.is_empty() {
                None
            } else {
                Some(parsed_summary.memory_triggers.join(", "))
            },
            embedding: None,
            last_ref_ts: None, // 将由数据库自动设置为当前时间
            weight: (parsed_summary.importance_score as f32) / 10.0, // 归一化到 0.0-1.0
        };

        // 6. 为语义片段生成嵌入向量
        if let Some(svc) = embedding_service.as_ref() {
            if let Ok(emb) = svc
                .lock()
                .await
                .encode_single(&semantic_chunk.summary)
                .await
            {
                if let Ok(bytes) = EmbeddingService::serialize_embedding(&emb) {
                    semantic_chunk.embedding = Some(bytes);
                }
            }
        }

        // 7. 保存语义片段到数据库
        match self
            .database_manager
            .insert_semantic_chunk(semantic_chunk)
            .await
        {
            Ok(chunk_id) => {
                println!(
                    "✨ 语义片段已创建 (ID: {}, 重要性: {})",
                    chunk_id, parsed_summary.importance_score
                );

                // 7.5. 创建语义片段映射表记录
                use crate::db::SemanticChunkMapping;
                let mapping = SemanticChunkMapping {
                    id: None,
                    chunk_id,
                    session_id,
                    memory_event_ids: "[]".to_string(), // 可扩展为实际记录相关事件ID
                    created_ts: None,                   // 由数据库自动设置
                };

                if let Err(e) = self
                    .database_manager
                    .insert_semantic_chunk_mapping(mapping)
                    .await
                {
                    eprintln!("⚠️ 语义片段映射创建失败: {}", e);
                }

                // 8. 提取实体（memory_triggers）但暂不更新图谱：按阶段目标仅实现工作记忆与语义记忆
                if !parsed_summary.memory_triggers.is_empty() {
                    // 根据配置决定是否更新知识图谱
                    let enable_graph_updates = self
                        .agent_configs
                        .values()
                        .find(|config| config.memory.enabled)
                        .map(|config| config.memory.enable_graph_updates)
                        .unwrap_or(false);

                    if enable_graph_updates {
                        if let Err(e) = self
                            .update_knowledge_graph(
                                &parsed_summary.memory_triggers,
                                (parsed_summary.importance_score as f32) / 10.0,
                            )
                            .await
                        {
                            eprintln!("⚠️ 知识图谱更新失败: {}", e);
                        }
                    } else {
                        // 知识图谱更新已禁用（情景/过程记忆暂不实现）
                        log::debug!(
                            "Skip knowledge graph update. Triggers = {:?}, importance = {}",
                            parsed_summary.memory_triggers,
                            parsed_summary.importance_score
                        );
                    }
                }
            }
            Err(e) => {
                eprintln!("⚠️ 语义片段保存失败: {}", e);
            }
        }

        Ok(())
    }

    /// 解析 ConversationSummarizer 的 JSON 响应
    #[allow(dead_code)] // 预留功能，后续启用
    fn parse_summarizer_response(&self, response: &str) -> Result<ConversationSummary> {
        // 先用与画像提取相同的鲁棒提取逻辑，截取第一个完整 JSON 对象
        let json_str_owned = extract_json_object(response).unwrap_or_else(|| response.to_string());
        let json_str = json_str_owned.as_str();

        // 解析 JSON
        let parsed: serde_json::Value =
            serde_json::from_str(json_str).map_err(|e| anyhow::anyhow!("JSON 解析失败: {}", e))?;

        // 提取字段
        let importance_score = parsed
            .get("importance_score")
            .and_then(|v| v.as_i64())
            .unwrap_or(0) as i32;

        let user_intent_summary = parsed
            .get("user_intent_summary")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        let summary = parsed
            .get("summary")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        let memory_triggers = parsed
            .get("memory_triggers")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str())
                    .map(|s| s.to_string())
                    .collect()
            })
            .unwrap_or_default();

        Ok(ConversationSummary {
            importance_score,
            user_intent_summary,
            summary,
            memory_triggers,
        })
    }

    /// 更新知识图谱：基于 memory_triggers 创建节点和共现边
    #[allow(dead_code)] // 预留功能，后续启用
    async fn update_knowledge_graph(
        &mut self,
        memory_triggers: &[String],
        base_edge_weight: f32,
    ) -> Result<()> {
        use crate::db::{GraphEdge, GraphNode};

        // 获取配置化的参数
        let memory_config = self
            .agent_configs
            .values()
            .find(|config| config.memory.enabled)
            .map(|config| config.memory.clone())
            .unwrap_or_default();

        let cooccur_weight_divisor = memory_config.cooccur_weight_divisor;
        let min_edge_weight = memory_config.min_edge_weight;
        let max_edge_weight = memory_config.max_edge_weight;
        let enable_weight_accumulation = memory_config.enable_weight_accumulation;

        // 1) 逐个触发词 upsert 节点，并记录 node_id
        let mut nodes: Vec<(String, i64)> = Vec::new();
        for trigger in memory_triggers {
            // 扩展的实体类型推断（基于规则）
            let entity_type = self.infer_entity_type(trigger);

            let node = GraphNode {
                id: None,
                entity_type,
                entity_name: trigger.clone(),
            };

            match self.database_manager.upsert_graph_node(node).await {
                Ok(node_id) => {
                    nodes.push((trigger.clone(), node_id));
                }
                Err(e) => {
                    eprintln!("⚠️ 图节点创建失败 {}: {}", trigger, e);
                }
            }
        }

        // 2) 在同一语义片段中将所有触发词两两连边（无向边使用有序 from->to 避免重复）
        for i in 0..nodes.len() {
            for j in (i + 1)..nodes.len() {
                let id1 = nodes[i].1;
                let id2 = nodes[j].1;
                let (from_node, to_node) = if id1 <= id2 { (id1, id2) } else { (id2, id1) };

                // 配置化的共现边权重计算
                let cooccur_weight = (base_edge_weight / cooccur_weight_divisor)
                    .clamp(min_edge_weight, max_edge_weight);

                // 根据实体类型推断更具体的关系类型
                let relation_type = self.infer_relation_type(&nodes[i].0, &nodes[j].0);

                let edge = GraphEdge {
                    id: None,
                    from_node,
                    to_node,
                    relation_type,
                    weight: cooccur_weight,
                };

                // 根据配置选择使用累积权重或直接插入
                let edge_result = if enable_weight_accumulation {
                    self.database_manager
                        .upsert_graph_edge_with_accumulation(edge)
                        .await
                } else {
                    self.database_manager.upsert_graph_edge(edge).await
                };

                match edge_result {
                    Ok(edge_id) => {
                        println!(
                            "🔗 图边已创建/更新: {} <-> {} (ID: {}, 权重: {:.3})",
                            nodes[i].0, nodes[j].0, edge_id, cooccur_weight
                        );
                    }
                    Err(e) => {
                        eprintln!("⚠️ 图边创建失败 {}-{}: {}", nodes[i].0, nodes[j].0, e);
                    }
                }
            }
        }

        Ok(())
    }

    /// 推断实体类型（可扩展为更复杂的NER或LLM分类）
    #[allow(dead_code)] // 预留功能，后续启用
    fn infer_entity_type(&self, entity_name: &str) -> String {
        // 基于规则的简单推断
        if entity_name.chars().any(|c| c.is_uppercase()) && entity_name.len() > 1 {
            // 首字母大写，可能是人名、地名或组织
            if entity_name.contains("公司")
                || entity_name.contains("Corp")
                || entity_name.contains("Ltd")
            {
                "organization".to_string()
            } else if entity_name.len() <= 5
                && entity_name
                    .chars()
                    .all(|c| c.is_alphabetic() || c.is_whitespace())
            {
                "person".to_string() // 短的字母组合，可能是人名
            } else {
                "entity".to_string() // 通用实体
            }
        } else if entity_name.ends_with("喜欢")
            || entity_name.ends_with("偏好")
            || entity_name.contains("prefer")
        {
            "preference".to_string()
        } else if entity_name.ends_with("任务")
            || entity_name.ends_with("task")
            || entity_name.contains("目标")
        {
            "task".to_string()
        } else {
            "concept".to_string() // 概念或其他
        }
    }

    /// 推断关系类型（基于实体类型和上下文）
    #[allow(dead_code)] // 预留功能，后续启用
    fn infer_relation_type(&self, entity1: &str, entity2: &str) -> String {
        let type1 = self.infer_entity_type(entity1);
        let type2 = self.infer_entity_type(entity2);

        match (type1.as_str(), type2.as_str()) {
            ("person", "organization") | ("organization", "person") => "works_at".to_string(),
            ("person", "preference") | ("preference", "person") => "prefers".to_string(),
            ("person", "task") | ("task", "person") => "assigned_to".to_string(),
            ("task", "concept") | ("concept", "task") => "relates_to".to_string(),
            ("person", "person") => "knows".to_string(),
            ("organization", "organization") => "partners_with".to_string(),
            ("preference", "preference") => "similar_to".to_string(),
            _ => "co_occurs".to_string(), // 默认共现关系
        }
    }

    /// 获取智能体列表
    pub async fn list_agents(&self) -> Vec<String> {
        let core_service = self.core_service.read().await;
        core_service.agents.keys().cloned().collect()
    }

    /// 获取系统统计信息
    pub async fn get_stats(&self) -> Result<HashMap<String, usize>> {
        let mut stats = HashMap::new();

        // 通过读锁访问 agents
        let agents_count = {
            let core_service = self.core_service.read().await;
            core_service.agents.len()
        };
        stats.insert("total_agents".to_string(), agents_count);

        Ok(stats)
    }

    /// 添加工具到工具注册表
    pub async fn register_tool<T: crate::core::tools::Tool + 'static>(&self, tool: T) {
        let mut tools = self.tools.write().await;
        let name = tool.name().to_string();
        tools.register(tool);
        println!("🔧 工具 '{}' 已注册 (wrapper)", name);
    }

    /// 获取数据库统计信息
    pub async fn get_database_stats(&self) -> Result<HashMap<String, String>> {
        let mut stats = HashMap::new();

        // SQLite 统计
        stats.insert("sqlite_status".to_string(), "connected".to_string());

        // 向量数据库统计
        stats.insert("vector_db_status".to_string(), "active".to_string());

        // 图数据库统计
        stats.insert("graph_db_status".to_string(), "active".to_string());

        Ok(stats)
    }

    /// 优雅关闭所有服务
    pub async fn shutdown(self) -> Result<()> {
        println!("🛑 正在关闭 RWKV Agent Kit...");

        // 关闭数据库连接
        self.database_manager
            .close()
            .await
            .map_err(|e| anyhow::anyhow!("Database close error: {}", e))?;

        println!("✅ RWKV Agent Kit 已安全关闭");
        Ok(())
    }
}

/// 便捷的构建器模式
pub struct RwkvAgentKitBuilder {
    config: RwkvAgentKitConfig,
}

impl RwkvAgentKitBuilder {
    /// 创建新的构建器
    pub fn new() -> Self {
        Self {
            config: RwkvAgentKitConfig::default(),
        }
    }

    /// 设置模型路径
    pub fn model_path<P: Into<String>>(mut self, path: P) -> Self {
        self.config.model.model_path = path.into();
        self
    }

    /// 设置分词器路径
    pub fn tokenizer_path<P: Into<String>>(mut self, path: P) -> Self {
        self.config.model.tokenizer_path = path.into();
        self
    }

    /// 设置精度
    pub fn precision<P: Into<String>>(mut self, precision: P) -> Self {
        self.config.model.precision = precision.into();
        self
    }

    /// 设置量化层数
    pub fn quant(mut self, quant: usize) -> Self {
        self.config.model.quant = Some(quant);
        self
    }

    /// 设置量化类型详细配置
    pub fn quant_type<Q: Into<String>>(mut self, quant_type: Q) -> Self {
        self.config.model.quant_type = Some(quant_type.into());
        self
    }

    /// 设置token块大小
    pub fn token_chunk_size(mut self, size: usize) -> Self {
        self.config.model.token_chunk_size = Some(size);
        self
    }

    /// 设置最大批次数
    pub fn max_batch(mut self, batch: usize) -> Self {
        self.config.model.max_batch = Some(batch);
        self
    }

    /// 设置嵌入设备
    pub fn embed_device<D: Into<String>>(mut self, device: D) -> Self {
        self.config.model.embed_device = Some(device.into());
        self
    }

    /// 设置BNF配置
    pub fn bnf(mut self, bnf: BnfConfig) -> Self {
        self.config.model.bnf = Some(bnf);
        self
    }

    /// 设置适配器
    pub fn adapter<A: Into<String>>(mut self, adapter: A) -> Self {
        self.config.model.adapter = Some(adapter.into());
        self
    }

    /// 设置数据库配置
    pub fn database_config(mut self, database_config: DatabaseConfig) -> Self {
        self.config.database = database_config;
        self
    }

    /// 添加智能体配置
    pub fn add_agent(mut self, agent_config: AgentConfig) -> Self {
        self.config.agents.push(agent_config);
        self
    }

    /// 添加默认智能体（主要agent和对话总结智能体）
    pub fn with_default_agents(mut self) -> Self {
        use crate::agents::{
            // create_persona_extractor_config, // 已移除画像提取智能体
            create_chat_config,
            create_conversation_summarizer_config,
        };

        // 首先添加画像提取智能体
        // self.config.agents.push(create_persona_extractor_config()); // 已移除画像提取智能体

        // 添加主对话 chat 智能体（独立模块定义，便于后续特殊处理与提示词优化）
        self.config.agents.push(create_chat_config());

        // 最后添加对话总结智能体
        self.config
            .agents
            .push(create_conversation_summarizer_config());

        self
    }

    /// 构建 RWKV Agent Kit 实例
    pub async fn build(self) -> Result<RwkvAgentKit> {
        RwkvAgentKit::new(self.config).await
    }
}

impl Default for RwkvAgentKitBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_builder() {
        let builder = RwkvAgentKitBuilder::new()
            .model_path("/path/to/model")
            .tokenizer_path("/path/to/tokenizer");

        assert_eq!(builder.config.model.model_path, "/path/to/model");
        assert_eq!(builder.config.model.tokenizer_path, "/path/to/tokenizer");
    }
}

/// 对话总结
#[allow(dead_code)] // 预留功能，后续启用
#[derive(Debug, Clone)]
struct ConversationSummary {
    importance_score: i32,
    user_intent_summary: String,
    summary: String,
    memory_triggers: Vec<String>,
}

/// 从文本中提取第一个完整的 JSON 对象（尽量用于解析 {"traits": ...}）
#[allow(dead_code)] // 预留功能，后续启用
fn extract_json_object(s: &str) -> Option<String> {
    // 首先尝试直接解析整个字符串
    if serde_json::from_str::<serde_json::Value>(s.trim()).is_ok() {
        return Some(s.trim().to_string());
    }

    // 如果直接解析失败，尝试提取JSON对象
    let bytes = s.as_bytes();
    let mut candidates = Vec::new();
    let mut i = 0;

    while i < bytes.len() {
        if bytes[i] == b'{' {
            let mut j = i;
            let mut depth: i32 = 0;
            let mut in_str = false;
            let mut esc = false;

            while j < bytes.len() {
                let b = bytes[j];
                if in_str {
                    if esc {
                        esc = false;
                    } else if b == b'\\' {
                        esc = true;
                    } else if b == b'"' {
                        in_str = false;
                    }
                } else if b == b'"' {
                    in_str = true;
                } else if b == b'{' {
                    depth += 1;
                } else if b == b'}' {
                    depth -= 1;
                    if depth == 0 {
                        let slice = &s[i..=j];
                        // 验证是否为有效JSON
                        if serde_json::from_str::<serde_json::Value>(slice).is_ok() {
                            candidates.push((slice.to_string(), slice.contains("\"traits\"")));
                        }
                        break;
                    }
                }
                j += 1;
            }
        }
        i += 1;
    }

    // 优先返回包含"traits"的JSON，否则返回第一个有效JSON
    candidates
        .iter()
        .find(|(_, has_traits)| *has_traits)
        .map(|(json, _)| json.clone())
        .or_else(|| candidates.first().map(|(json, _)| json.clone()))
}

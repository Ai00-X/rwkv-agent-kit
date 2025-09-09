use rwkv_agent_kit::RwkvAgentKitBuilder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 可选：初始化日志（仅输出警告及以上级别信息，去掉ai00core的INFO信息）
    let _ = env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Warn)
        .try_init();

    // 1) 使用默认模型与默认智能体集合构建 Kit
    // - 模型与分词器默认从 ./model/ 读取：model.st, tokenizer.json, chat.state, tool-call.state
    // - 数据库默认位于 ./data/agent_kit.db（仓库已包含 data/ 目录）
    let mut kit = RwkvAgentKitBuilder::new()
        .with_default_agents()
        .build()
        .await?;

    // 清理数据库中的重复记录（调试用）
    if let Err(e) = kit.database_manager.clear_all_memory_events().await {
        println!("Warning: Failed to clear memory events: {}", e);
    } else {
        println!("[DEBUG] 已清理数据库中的历史记录");
    }

    // 2) 多轮对话测试，验证历史记录功能
    let conversations = vec![
        "你好，我的名字叫小明，是一名程序员。",
        "我最喜欢的编程语言是Rust，你知道为什么吗？",
        "what's my name?",
        "我的职业是什么？请根据我们之前的对话回答。",
    ];

    for (round, user_input) in conversations.iter().enumerate() {
        println!("\n=== 第{}轮对话 ===", round + 1);
        println!("[USER] {}", user_input);

        let answer = kit.chat("chat", user_input).await?;

        // 打印AI完整回复
        println!("[DEBUG] AI完整回复: {}", answer);
        println!("[ASSISTANT] {}", answer);

        // 显示当前对话历史记录状态
        match kit.database_manager.get_active_session().await {
            Ok(Some(session_id)) => {
                match kit.database_manager.list_memory_events(session_id).await {
                    Ok(events) => {
                        println!("\n[DEBUG] 当前历史记录 ({} 条):", events.len());
                        for (i, event) in events.iter().enumerate() {
                            let display_text = if event.text.chars().count() > 50 {
                                let truncated: String = event.text.chars().take(50).collect();
                                format!("{}...", truncated)
                            } else {
                                event.text.clone()
                            };
                            println!("  {}. [{}] {}", i + 1, event.role, display_text);
                        }
                    }
                    Err(e) => {
                        println!("[DEBUG] 获取历史记录失败: {}", e);
                    }
                }
            }
            Ok(None) => {
                println!("[DEBUG] 没有活跃会话");
            }
            Err(e) => {
                println!("[DEBUG] 获取活跃会话失败: {}", e);
            }
        }

        println!("{}", format!("\n{}", "-".repeat(50)));
    }

    // 4) 画像特征查询功能已移除
    // 原本这里会查询并打印 persona_traits，但画像智能体已被移除
    println!("\n[INFO] 画像智能体功能已移除，不再提取和显示用户特征。");

    Ok(())
}

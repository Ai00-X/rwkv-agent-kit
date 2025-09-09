---
title: basic-usage
createTime: 2025/09/08 13:16:30
permalink: /article/s13gse6a/
---
# 基本使用

## 快速开始

### 1. 初始化智能体

```rust
use rwkv_agent_kit::{RwkvAgentKit, AgentConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建配置
    let config = AgentConfig {
        model_path: "path/to/your/model.safetensors".to_string(),
        tokenizer_path: "path/to/tokenizer.json".to_string(),
        max_tokens: 2048,
        temperature: 0.7,
        top_p: 0.9,
    };
    
    // 初始化智能体
    let mut agent = RwkvAgentKit::new(config).await?;
    
    Ok(())
}
```

### 2. 发送消息

```rust
// 发送用户消息
let response = agent.chat("你好，请介绍一下你自己").await?;
println!("智能体回复: {}", response);
```

### 3. 使用工具

```rust
use rwkv_agent_kit::tools::CalculatorTool;

// 添加计算器工具
agent.add_tool(Box::new(CalculatorTool::new()));

// 使用工具进行计算
let result = agent.chat("请计算 15 * 23 + 45").await?;
println!("计算结果: {}", result);
```

### 4. 会话管理

```rust
// 创建新会话
let session_id = agent.create_session().await?;

// 在指定会话中聊天
let response = agent.chat_in_session(session_id, "记住我的名字是张三").await?;

// 获取会话历史
let history = agent.get_session_history(session_id).await?;
```

## 配置选项

### 模型配置

- `model_path`: RWKV 模型文件路径
- `tokenizer_path`: 分词器文件路径
- `max_tokens`: 最大生成令牌数
- `temperature`: 生成温度 (0.0-1.0)
- `top_p`: Top-p 采样参数

### 数据库配置

```rust
use rwkv_agent_kit::database::DatabaseConfig;

let db_config = DatabaseConfig {
    url: "sqlite://memory.db".to_string(),
    max_connections: 10,
};
```

## 错误处理

```rust
match agent.chat("你好").await {
    Ok(response) => println!("回复: {}", response),
    Err(e) => eprintln!("错误: {}", e),
}
```
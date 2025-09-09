# 示例项目

本章节提供了丰富的示例项目，展示如何使用RWKV Agent Kit构建各种类型的智能体应用。每个示例都包含完整的源代码、详细的说明和最佳实践。

## 🚀 快速开始示例

### 基础聊天机器人

最简单的聊天机器人实现，适合初学者：

```rust
use rwkv_agent_kit::prelude::*;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化配置
    let config = AgentConfig::builder()
        .model_path("./models/rwkv-4-world-7b.pth")
        .device(Device::Cpu)
        .max_tokens(512)
        .temperature(0.7)
        .build()?;

    // 创建智能体
    let mut agent = Agent::new("ChatBot", config).await?;

    println!("聊天机器人已启动！输入 'quit' 退出。");

    loop {
        print!("用户: ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();
        
        if input == "quit" {
            break;
        }
        
        // 处理用户输入
        match agent.chat(input).await {
            Ok(response) => println!("机器人: {}", response),
            Err(e) => eprintln!("错误: {}", e),
        }
    }
    
    Ok(())
}
```

### 带记忆的智能助手

具有记忆功能的智能助手：

```rust
use rwkv_agent_kit::prelude::*;
use rwkv_agent_kit::memory::SqliteMemory;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 配置记忆系统
    let memory_config = MemoryConfig::builder()
        .storage_type(StorageType::Sqlite)
        .database_path("./data/assistant_memory.db")
        .max_context_length(2048)
        .build()?;

    // 创建记忆实例
    let memory = SqliteMemory::new(memory_config).await?;

    // 配置智能体
    let config = AgentConfig::builder()
        .model_path("./models/rwkv-4-world-7b.pth")
        .device(Device::Auto)
        .memory(Box::new(memory))
        .build()?;

    let mut agent = Agent::new("Assistant", config).await?;

    // 设置系统提示
    agent.set_system_prompt(
        "你是一个有用的AI助手。你能记住之前的对话内容，
         并根据用户的偏好提供个性化的帮助。"
    ).await?;

    println!("智能助手已启动！我会记住我们的对话。");

    loop {
        print!("\n用户: ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();
        
        if input == "quit" {
            break;
        }
        
        // 检索相关记忆
        let relevant_memories = agent.memory()
            .search_similar(input, 5)
            .await?;
        
        // 处理输入并生成回应
        let response = agent.chat_with_context(input, &relevant_memories).await?;
        println!("助手: {}", response);
        
        // 存储对话到记忆
        agent.memory()
            .store_conversation(input, &response)
            .await?;
    }
    
    Ok(())
}
```

## 🛠️ 工具集成示例

### 天气查询助手

集成天气API的智能助手：

```rust
use rwkv_agent_kit::prelude::*;
use rwkv_agent_kit::tools::{Tool, ToolResult};
use serde::{Deserialize, Serialize};
use reqwest;

#[derive(Debug, Serialize, Deserialize)]
struct WeatherQuery {
    city: String,
    country: Option<String>,
}

#[derive(Debug)]
struct WeatherTool {
    api_key: String,
    client: reqwest::Client,
}

impl WeatherTool {
    fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait::async_trait]
impl Tool for WeatherTool {
    fn name(&self) -> &str {
        "weather_query"
    }
    
    fn description(&self) -> &str {
        "查询指定城市的当前天气信息"
    }
    
    fn parameters_schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "city": {
                    "type": "string",
                    "description": "城市名称"
                },
                "country": {
                    "type": "string",
                    "description": "国家代码（可选）"
                }
            },
            "required": ["city"]
        })
    }
    
    async fn execute(&self, params: serde_json::Value) -> Result<ToolResult, ToolError> {
        let query: WeatherQuery = serde_json::from_value(params)?;
        
        let url = format!(
            "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric&lang=zh",
            query.city, self.api_key
        );
        
        let response = self.client.get(&url).send().await?;
        let weather_data: serde_json::Value = response.json().await?;
        
        if weather_data["cod"].as_i64() == Some(200) {
            let result = serde_json::json!({
                "city": weather_data["name"],
                "country": weather_data["sys"]["country"],
                "temperature": weather_data["main"]["temp"],
                "feels_like": weather_data["main"]["feels_like"],
                "humidity": weather_data["main"]["humidity"],
                "description": weather_data["weather"][0]["description"],
                "wind_speed": weather_data["wind"]["speed"]
            });
            Ok(ToolResult::success(result))
        } else {
            Err(ToolError::ExecutionError(
                format!("天气查询失败: {}", weather_data["message"])
            ))
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建天气工具
    let weather_tool = WeatherTool::new(
        std::env::var("OPENWEATHER_API_KEY")?
    );
    
    // 配置智能体
    let config = AgentConfig::builder()
        .model_path("./models/rwkv-4-world-7b.pth")
        .device(Device::Auto)
        .tools(vec![Box::new(weather_tool)])
        .build()?;
    
    let mut agent = Agent::new("WeatherAssistant", config).await?;
    
    agent.set_system_prompt(
        "你是一个天气助手。当用户询问天气时，使用weather_query工具获取准确的天气信息。
         请用友好、自然的语言回答用户的天气查询。"
    ).await?;
    
    println!("天气助手已启动！你可以询问任何城市的天气。");
    
    loop {
        print!("\n用户: ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();
        
        if input == "quit" {
            break;
        }
        
        match agent.chat(input).await {
            Ok(response) => println!("天气助手: {}", response),
            Err(e) => eprintln!("错误: {}", e),
        }
    }
    
    Ok(())
}
```

### 文件管理助手

能够管理文件系统的智能助手：

```rust
use rwkv_agent_kit::prelude::*;
use rwkv_agent_kit::tools::{Tool, ToolResult, ToolError};
use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct FileOperation {
    operation: String, // "list", "read", "write", "delete"
    path: String,
    content: Option<String>,
}

#[derive(Debug)]
struct FileManagerTool {
    allowed_paths: Vec<String>,
}

impl FileManagerTool {
    fn new(allowed_paths: Vec<String>) -> Self {
        Self { allowed_paths }
    }
    
    fn is_path_allowed(&self, path: &str) -> bool {
        self.allowed_paths.iter().any(|allowed| {
            Path::new(path).starts_with(allowed)
        })
    }
}

#[async_trait::async_trait]
impl Tool for FileManagerTool {
    fn name(&self) -> &str {
        "file_manager"
    }
    
    fn description(&self) -> &str {
        "管理文件系统：列出目录、读取文件、写入文件、删除文件"
    }
    
    fn parameters_schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "operation": {
                    "type": "string",
                    "enum": ["list", "read", "write", "delete"],
                    "description": "操作类型"
                },
                "path": {
                    "type": "string",
                    "description": "文件或目录路径"
                },
                "content": {
                    "type": "string",
                    "description": "写入文件的内容（仅用于write操作）"
                }
            },
            "required": ["operation", "path"]
        })
    }
    
    async fn execute(&self, params: serde_json::Value) -> Result<ToolResult, ToolError> {
        let op: FileOperation = serde_json::from_value(params)?;
        
        if !self.is_path_allowed(&op.path) {
            return Err(ToolError::PermissionDenied(
                format!("路径不在允许范围内: {}", op.path)
            ));
        }
        
        match op.operation.as_str() {
            "list" => {
                let entries = fs::read_dir(&op.path)
                    .map_err(|e| ToolError::ExecutionError(e.to_string()))?
                    .map(|entry| {
                        let entry = entry.unwrap();
                        let path = entry.path();
                        serde_json::json!({
                            "name": path.file_name().unwrap().to_string_lossy(),
                            "is_dir": path.is_dir(),
                            "size": entry.metadata().map(|m| m.len()).unwrap_or(0)
                        })
                    })
                    .collect::<Vec<_>>();
                
                Ok(ToolResult::success(serde_json::json!({
                    "entries": entries
                })))
            },
            "read" => {
                let content = fs::read_to_string(&op.path)
                    .map_err(|e| ToolError::ExecutionError(e.to_string()))?;
                
                Ok(ToolResult::success(serde_json::json!({
                    "content": content
                })))
            },
            "write" => {
                let content = op.content.ok_or_else(|| 
                    ToolError::InvalidParameters("写入操作需要content参数".to_string())
                )?;
                
                fs::write(&op.path, content)
                    .map_err(|e| ToolError::ExecutionError(e.to_string()))?;
                
                Ok(ToolResult::success(serde_json::json!({
                    "message": "文件写入成功"
                })))
            },
            "delete" => {
                if Path::new(&op.path).is_dir() {
                    fs::remove_dir_all(&op.path)
                } else {
                    fs::remove_file(&op.path)
                }.map_err(|e| ToolError::ExecutionError(e.to_string()))?;
                
                Ok(ToolResult::success(serde_json::json!({
                    "message": "删除成功"
                })))
            },
            _ => Err(ToolError::InvalidParameters(
                format!("不支持的操作: {}", op.operation)
            ))
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建文件管理工具（限制在特定目录）
    let file_tool = FileManagerTool::new(vec![
        "./workspace".to_string(),
        "./documents".to_string(),
    ]);
    
    let config = AgentConfig::builder()
        .model_path("./models/rwkv-4-world-7b.pth")
        .device(Device::Auto)
        .tools(vec![Box::new(file_tool)])
        .build()?;
    
    let mut agent = Agent::new("FileManager", config).await?;
    
    agent.set_system_prompt(
        "你是一个文件管理助手。你可以帮助用户管理文件和目录，
         包括列出文件、读取内容、写入文件和删除文件。
         请确保操作安全，并向用户确认重要操作。"
    ).await?;
    
    println!("文件管理助手已启动！我可以帮你管理workspace和documents目录。");
    
    loop {
        print!("\n用户: ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();
        
        if input == "quit" {
            break;
        }
        
        match agent.chat(input).await {
            Ok(response) => println!("文件助手: {}", response),
            Err(e) => eprintln!("错误: {}", e),
        }
    }
    
    Ok(())
}
```

## 🌐 Web应用示例

### REST API服务

使用Axum框架创建智能体API服务：

```rust
use rwkv_agent_kit::prelude::*;
use axum::{
    extract::{State, Json},
    http::StatusCode,
    response::Json as ResponseJson,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_cors::CorsLayer;

#[derive(Debug, Serialize, Deserialize)]
struct ChatRequest {
    message: String,
    session_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ChatResponse {
    response: String,
    session_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ErrorResponse {
    error: String,
}

type SharedAgent = Arc<Mutex<Agent>>;

async fn health_check() -> &'static str {
    "RWKV Agent API is running"
}

async fn chat_handler(
    State(agent): State<SharedAgent>,
    Json(request): Json<ChatRequest>,
) -> Result<ResponseJson<ChatResponse>, (StatusCode, ResponseJson<ErrorResponse>)> {
    let session_id = request.session_id.unwrap_or_else(|| {
        uuid::Uuid::new_v4().to_string()
    });
    
    let mut agent = agent.lock().await;
    
    match agent.chat(&request.message).await {
        Ok(response) => {
            Ok(ResponseJson(ChatResponse {
                response,
                session_id,
            }))
        },
        Err(e) => {
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                ResponseJson(ErrorResponse {
                    error: e.to_string(),
                })
            ))
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志
    tracing_subscriber::init();
    
    // 创建智能体
    let config = AgentConfig::builder()
        .model_path("./models/rwkv-4-world-7b.pth")
        .device(Device::Auto)
        .max_tokens(1024)
        .temperature(0.7)
        .build()?;
    
    let agent = Agent::new("APIAgent", config).await?;
    let shared_agent = Arc::new(Mutex::new(agent));
    
    // 创建路由
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/chat", post(chat_handler))
        .layer(CorsLayer::permissive())
        .with_state(shared_agent);
    
    // 启动服务器
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("RWKV Agent API服务器启动在 http://0.0.0.0:3000");
    
    axum::serve(listener, app).await?;
    
    Ok(())
}
```

### WebSocket实时聊天

实现WebSocket实时聊天功能：

```rust
use rwkv_agent_kit::prelude::*;
use axum::{
    extract::{ws::{Message, WebSocket, WebSocketUpgrade}, State},
    response::Response,
    routing::get,
    Router,
};
use futures::{sink::SinkExt, stream::StreamExt};
use std::sync::Arc;
use tokio::sync::Mutex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct WSMessage {
    #[serde(rename = "type")]
    msg_type: String,
    content: String,
    timestamp: u64,
}

type SharedAgent = Arc<Mutex<Agent>>;

async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(agent): State<SharedAgent>,
) -> Response {
    ws.on_upgrade(|socket| handle_socket(socket, agent))
}

async fn handle_socket(socket: WebSocket, agent: SharedAgent) {
    let (mut sender, mut receiver) = socket.split();
    
    // 发送欢迎消息
    let welcome_msg = WSMessage {
        msg_type: "agent".to_string(),
        content: "你好！我是RWKV智能助手，有什么可以帮助你的吗？".to_string(),
        timestamp: chrono::Utc::now().timestamp() as u64,
    };
    
    if let Ok(msg) = serde_json::to_string(&welcome_msg) {
        let _ = sender.send(Message::Text(msg)).await;
    }
    
    // 处理消息循环
    while let Some(msg) = receiver.next().await {
        if let Ok(msg) = msg {
            match msg {
                Message::Text(text) => {
                    if let Ok(user_msg) = serde_json::from_str::<WSMessage>(&text) {
                        if user_msg.msg_type == "user" {
                            // 处理用户消息
                            let mut agent = agent.lock().await;
                            match agent.chat(&user_msg.content).await {
                                Ok(response) => {
                                    let response_msg = WSMessage {
                                        msg_type: "agent".to_string(),
                                        content: response,
                                        timestamp: chrono::Utc::now().timestamp() as u64,
                                    };
                                    
                                    if let Ok(msg) = serde_json::to_string(&response_msg) {
                                        let _ = sender.send(Message::Text(msg)).await;
                                    }
                                },
                                Err(e) => {
                                    let error_msg = WSMessage {
                                        msg_type: "error".to_string(),
                                        content: format!("处理消息时出错: {}", e),
                                        timestamp: chrono::Utc::now().timestamp() as u64,
                                    };
                                    
                                    if let Ok(msg) = serde_json::to_string(&error_msg) {
                                        let _ = sender.send(Message::Text(msg)).await;
                                    }
                                }
                            }
                        }
                    }
                },
                Message::Close(_) => {
                    println!("WebSocket连接关闭");
                    break;
                },
                _ => {}
            }
        } else {
            break;
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建智能体
    let config = AgentConfig::builder()
        .model_path("./models/rwkv-4-world-7b.pth")
        .device(Device::Auto)
        .max_tokens(512)
        .temperature(0.8)
        .build()?;
    
    let agent = Agent::new("WSAgent", config).await?;
    let shared_agent = Arc::new(Mutex::new(agent));
    
    // 创建路由
    let app = Router::new()
        .route("/ws", get(websocket_handler))
        .with_state(shared_agent);
    
    // 启动服务器
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await?;
    println!("WebSocket服务器启动在 ws://0.0.0.0:3001/ws");
    
    axum::serve(listener, app).await?;
    
    Ok(())
}
```

## 🤖 多智能体协作示例

### 新闻分析团队

多个智能体协作分析新闻：

```rust
use rwkv_agent_kit::prelude::*;
use rwkv_agent_kit::multi_agent::*;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建消息总线
    let message_bus = MessageBus::new();
    
    // 创建新闻收集智能体
    let collector_config = AgentConfig::builder()
        .model_path("./models/rwkv-4-world-7b.pth")
        .device(Device::Auto)
        .build()?;
    
    let mut news_collector = Agent::new("NewsCollector", collector_config).await?
        .with_role("负责收集和筛选新闻")
        .with_tools(vec!["web_search", "rss_reader"])
        .connect_to_bus(&message_bus);
    
    // 创建情感分析智能体
    let sentiment_config = AgentConfig::builder()
        .model_path("./models/rwkv-4-world-7b.pth")
        .device(Device::Auto)
        .build()?;
    
    let mut sentiment_analyzer = Agent::new("SentimentAnalyzer", sentiment_config).await?
        .with_role("分析新闻情感倾向")
        .with_tools(vec!["sentiment_analysis"])
        .connect_to_bus(&message_bus);
    
    // 创建摘要生成智能体
    let summary_config = AgentConfig::builder()
        .model_path("./models/rwkv-4-world-7b.pth")
        .device(Device::Auto)
        .build()?;
    
    let mut summarizer = Agent::new("Summarizer", summary_config).await?
        .with_role("生成新闻摘要")
        .with_tools(vec!["text_summarization"])
        .connect_to_bus(&message_bus);
    
    // 创建报告生成智能体
    let report_config = AgentConfig::builder()
        .model_path("./models/rwkv-4-world-7b.pth")
        .device(Device::Auto)
        .build()?;
    
    let mut report_generator = Agent::new("ReportGenerator", report_config).await?
        .with_role("生成最终分析报告")
        .with_tools(vec!["report_generation"])
        .connect_to_bus(&message_bus);
    
    // 协作流程
    let topic = "人工智能最新发展";
    
    println!("开始新闻分析协作流程...");
    
    // 1. 收集新闻
    println!("步骤1: 收集相关新闻");
    let news_data = news_collector.process(&format!("收集关于'{}'的最新新闻", topic)).await?;
    
    // 2. 情感分析
    println!("步骤2: 进行情感分析");
    let sentiment_message = Message::new()
        .from("NewsCollector")
        .to("SentimentAnalyzer")
        .with_content(news_data.clone())
        .with_task("分析这些新闻的情感倾向");
    
    message_bus.send(sentiment_message).await?;
    let sentiment_result = sentiment_analyzer.receive_and_process().await?;
    
    // 3. 生成摘要
    println!("步骤3: 生成新闻摘要");
    let summary_message = Message::new()
        .from("NewsCollector")
        .to("Summarizer")
        .with_content(news_data.clone())
        .with_task("为这些新闻生成简洁摘要");
    
    message_bus.send(summary_message).await?;
    let summary_result = summarizer.receive_and_process().await?;
    
    // 4. 生成最终报告
    println!("步骤4: 生成分析报告");
    let report_data = format!(
        "新闻内容: {}\n情感分析: {}\n摘要: {}",
        news_data, sentiment_result, summary_result
    );
    
    let report_message = Message::new()
        .from("Coordinator")
        .to("ReportGenerator")
        .with_content(report_data)
        .with_task("基于收集的信息生成完整的分析报告");
    
    message_bus.send(report_message).await?;
    let final_report = report_generator.receive_and_process().await?;
    
    println!("\n=== 新闻分析报告 ===");
    println!("{}", final_report);
    
    Ok(())
}
```

## 📱 移动应用示例

### Flutter集成

通过FFI在Flutter中使用RWKV Agent Kit：

```rust
// lib.rs - Rust FFI接口
use rwkv_agent_kit::prelude::*;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    static ref AGENT: Mutex<Option<Agent>> = Mutex::new(None);
}

#[no_mangle]
pub extern "C" fn init_agent(model_path: *const c_char) -> i32 {
    let model_path = unsafe {
        CStr::from_ptr(model_path).to_string_lossy().into_owned()
    };
    
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    match rt.block_on(async {
        let config = AgentConfig::builder()
            .model_path(&model_path)
            .device(Device::Auto)
            .max_tokens(512)
            .build()?;
        
        Agent::new("MobileAgent", config).await
    }) {
        Ok(agent) => {
            *AGENT.lock().unwrap() = Some(agent);
            0 // 成功
        },
        Err(_) => -1 // 失败
    }
}

#[no_mangle]
pub extern "C" fn chat(input: *const c_char) -> *mut c_char {
    let input = unsafe {
        CStr::from_ptr(input).to_string_lossy().into_owned()
    };
    
    let mut agent_guard = AGENT.lock().unwrap();
    if let Some(ref mut agent) = *agent_guard {
        let rt = tokio::runtime::Runtime::new().unwrap();
        
        match rt.block_on(agent.chat(&input)) {
            Ok(response) => {
                let c_string = CString::new(response).unwrap();
                c_string.into_raw()
            },
            Err(_) => {
                let error = CString::new("处理消息时出错").unwrap();
                error.into_raw()
            }
        }
    } else {
        let error = CString::new("智能体未初始化").unwrap();
        error.into_raw()
    }
}

#[no_mangle]
pub extern "C" fn free_string(s: *mut c_char) {
    unsafe {
        if s.is_null() {
            return;
        }
        CString::from_raw(s);
    }
}
```

对应的Flutter代码：

```dart
// agent_service.dart
import 'dart:ffi';
import 'dart:io';
import 'package:ffi/ffi.dart';

class AgentService {
  static DynamicLibrary? _lib;
  static late Function _initAgent;
  static late Function _chat;
  static late Function _freeString;
  
  static void initialize() {
    if (Platform.isAndroid) {
      _lib = DynamicLibrary.open('librwkv_agent_mobile.so');
    } else if (Platform.isIOS) {
      _lib = DynamicLibrary.process();
    }
    
    _initAgent = _lib!.lookupFunction<
        Int32 Function(Pointer<Utf8>),
        int Function(Pointer<Utf8>)
    >('init_agent');
    
    _chat = _lib!.lookupFunction<
        Pointer<Utf8> Function(Pointer<Utf8>),
        Pointer<Utf8> Function(Pointer<Utf8>)
    >('chat');
    
    _freeString = _lib!.lookupFunction<
        Void Function(Pointer<Utf8>),
        void Function(Pointer<Utf8>)
    >('free_string');
  }
  
  static Future<bool> initAgent(String modelPath) async {
    final pathPtr = modelPath.toNativeUtf8();
    final result = _initAgent(pathPtr);
    malloc.free(pathPtr);
    return result == 0;
  }
  
  static Future<String> chat(String message) async {
    final messagePtr = message.toNativeUtf8();
    final responsePtr = _chat(messagePtr);
    
    final response = responsePtr.toDartString();
    
    _freeString(responsePtr);
    malloc.free(messagePtr);
    
    return response;
  }
}

// chat_screen.dart
import 'package:flutter/material.dart';
import 'agent_service.dart';

class ChatScreen extends StatefulWidget {
  @override
  _ChatScreenState createState() => _ChatScreenState();
}

class _ChatScreenState extends State<ChatScreen> {
  final TextEditingController _controller = TextEditingController();
  final List<ChatMessage> _messages = [];
  bool _isLoading = false;
  
  @override
  void initState() {
    super.initState();
    _initializeAgent();
  }
  
  Future<void> _initializeAgent() async {
    AgentService.initialize();
    final success = await AgentService.initAgent('/path/to/model.pth');
    if (success) {
      setState(() {
        _messages.add(ChatMessage(
          text: '你好！我是RWKV智能助手，有什么可以帮助你的吗？',
          isUser: false,
        ));
      });
    }
  }
  
  Future<void> _sendMessage(String text) async {
    if (text.trim().isEmpty) return;
    
    setState(() {
      _messages.add(ChatMessage(text: text, isUser: true));
      _isLoading = true;
    });
    
    _controller.clear();
    
    try {
      final response = await AgentService.chat(text);
      setState(() {
        _messages.add(ChatMessage(text: response, isUser: false));
        _isLoading = false;
      });
    } catch (e) {
      setState(() {
        _messages.add(ChatMessage(
          text: '抱歉，处理消息时出现错误。',
          isUser: false,
        ));
        _isLoading = false;
      });
    }
  }
  
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text('RWKV智能助手'),
      ),
      body: Column(
        children: [
          Expanded(
            child: ListView.builder(
              itemCount: _messages.length,
              itemBuilder: (context, index) {
                return ChatBubble(message: _messages[index]);
              },
            ),
          ),
          if (_isLoading)
            Padding(
              padding: EdgeInsets.all(8.0),
              child: CircularProgressIndicator(),
            ),
          Container(
            padding: EdgeInsets.all(8.0),
            child: Row(
              children: [
                Expanded(
                  child: TextField(
                    controller: _controller,
                    decoration: InputDecoration(
                      hintText: '输入消息...',
                      border: OutlineInputBorder(),
                    ),
                    onSubmitted: _sendMessage,
                  ),
                ),
                SizedBox(width: 8),
                IconButton(
                  icon: Icon(Icons.send),
                  onPressed: () => _sendMessage(_controller.text),
                ),
              ],
            ),
          ),
        ],
      ),
    );
  }
}

class ChatMessage {
  final String text;
  final bool isUser;
  
  ChatMessage({required this.text, required this.isUser});
}

class ChatBubble extends StatelessWidget {
  final ChatMessage message;
  
  ChatBubble({required this.message});
  
  @override
  Widget build(BuildContext context) {
    return Container(
      margin: EdgeInsets.symmetric(vertical: 4, horizontal: 8),
      child: Row(
        mainAxisAlignment: message.isUser 
            ? MainAxisAlignment.end 
            : MainAxisAlignment.start,
        children: [
          Container(
            constraints: BoxConstraints(
              maxWidth: MediaQuery.of(context).size.width * 0.7,
            ),
            padding: EdgeInsets.all(12),
            decoration: BoxDecoration(
              color: message.isUser 
                  ? Colors.blue[500] 
                  : Colors.grey[300],
              borderRadius: BorderRadius.circular(12),
            ),
            child: Text(
              message.text,
              style: TextStyle(
                color: message.isUser ? Colors.white : Colors.black,
              ),
            ),
          ),
        ],
      ),
    );
  }
}
```

---

这些示例展示了RWKV Agent Kit在不同场景下的应用。每个示例都可以作为你项目的起点，根据具体需求进行修改和扩展。

**更多资源**:
- [API文档](/api/) - 详细的接口说明
- [高级功能](/advanced/) - 深入了解高级特性
- [GitHub仓库](https://github.com/Ai00-X/rwkv-agent-kit) - 获取最新代码和示例
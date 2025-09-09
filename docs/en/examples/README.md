# Example Projects

This section provides rich example projects that demonstrate how to use RWKV Agent Kit to build various types of intelligent agent applications. Each example includes complete source code, detailed explanations, and best practices.

## 🚀 Quick Start Examples

### Basic Chatbot

The simplest chatbot implementation, perfect for beginners:

```rust
use rwkv_agent_kit::prelude::*;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize configuration
    let config = AgentConfig::builder()
        .model_path("./models/rwkv-4-world-7b.pth")
        .device(Device::Cpu)
        .max_tokens(512)
        .temperature(0.7)
        .build()?;

    // Create agent
    let mut agent = Agent::new("ChatBot", config).await?;

    println!("Chatbot started! Type 'quit' to exit.");

    loop {
        print!("User: ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();
        
        if input == "quit" {
            break;
        }
        
        // Process user input
        match agent.chat(input).await {
            Ok(response) => println!("Bot: {}", response),
            Err(e) => eprintln!("Error: {}", e),
        }
    }
    
    Ok(())
}
```

### Memory-Enabled Assistant

An intelligent assistant with memory capabilities:

```rust
use rwkv_agent_kit::prelude::*;
use rwkv_agent_kit::memory::SqliteMemory;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configure memory system
    let memory_config = MemoryConfig::builder()
        .storage_type(StorageType::Sqlite)
        .database_path("./data/assistant_memory.db")
        .max_context_length(2048)
        .build()?;

    // Create memory instance
    let memory = SqliteMemory::new(memory_config).await?;

    // Configure agent
    let config = AgentConfig::builder()
        .model_path("./models/rwkv-4-world-7b.pth")
        .device(Device::Auto)
        .memory(Box::new(memory))
        .build()?;

    let mut agent = Agent::new("Assistant", config).await?;

    // Set system prompt
    agent.set_system_prompt(
        "You are a helpful AI assistant. You can remember previous conversations
         and provide personalized assistance based on user preferences."
    ).await?;

    println!("Smart assistant started! I'll remember our conversations.");

    loop {
        print!("\nUser: ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();
        
        if input == "quit" {
            break;
        }
        
        // Retrieve relevant memories
        let relevant_memories = agent.memory()
            .search_similar(input, 5)
            .await?;
        
        // Process input and generate response
        let response = agent.chat_with_context(input, &relevant_memories).await?;
        println!("Assistant: {}", response);
        
        // Store conversation to memory
        agent.memory()
            .store_conversation(input, &response)
            .await?;
    }
    
    Ok(())
}
```

## 🛠️ Tool Integration Examples

### Weather Query Assistant

An intelligent assistant integrated with weather API:

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
        "Query current weather information for a specified city"
    }
    
    fn parameters_schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "city": {
                    "type": "string",
                    "description": "City name"
                },
                "country": {
                    "type": "string",
                    "description": "Country code (optional)"
                }
            },
            "required": ["city"]
        })
    }
    
    async fn execute(&self, params: serde_json::Value) -> Result<ToolResult, ToolError> {
        let query: WeatherQuery = serde_json::from_value(params)?;
        
        let url = format!(
            "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric",
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
                format!("Weather query failed: {}", weather_data["message"])
            ))
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create weather tool
    let weather_tool = WeatherTool::new(
        std::env::var("OPENWEATHER_API_KEY")?
    );
    
    // Configure agent
    let config = AgentConfig::builder()
        .model_path("./models/rwkv-4-world-7b.pth")
        .device(Device::Auto)
        .tools(vec![Box::new(weather_tool)])
        .build()?;
    
    let mut agent = Agent::new("WeatherAssistant", config).await?;
    
    agent.set_system_prompt(
        "You are a weather assistant. When users ask about weather, use the weather_query tool
         to get accurate weather information. Please respond in a friendly, natural manner."
    ).await?;
    
    println!("Weather assistant started! You can ask about weather in any city.");
    
    loop {
        print!("\nUser: ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();
        
        if input == "quit" {
            break;
        }
        
        match agent.chat(input).await {
            Ok(response) => println!("Weather Assistant: {}", response),
            Err(e) => eprintln!("Error: {}", e),
        }
    }
    
    Ok(())
}
```

### File Management Assistant

An intelligent assistant capable of managing file systems:

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
        "Manage file system: list directories, read files, write files, delete files"
    }
    
    fn parameters_schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "operation": {
                    "type": "string",
                    "enum": ["list", "read", "write", "delete"],
                    "description": "Operation type"
                },
                "path": {
                    "type": "string",
                    "description": "File or directory path"
                },
                "content": {
                    "type": "string",
                    "description": "Content to write to file (only for write operation)"
                }
            },
            "required": ["operation", "path"]
        })
    }
    
    async fn execute(&self, params: serde_json::Value) -> Result<ToolResult, ToolError> {
        let op: FileOperation = serde_json::from_value(params)?;
        
        if !self.is_path_allowed(&op.path) {
            return Err(ToolError::PermissionDenied(
                format!("Path not in allowed range: {}", op.path)
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
                    ToolError::InvalidParameters("Write operation requires content parameter".to_string())
                )?;
                
                fs::write(&op.path, content)
                    .map_err(|e| ToolError::ExecutionError(e.to_string()))?;
                
                Ok(ToolResult::success(serde_json::json!({
                    "message": "File written successfully"
                })))
            },
            "delete" => {
                if Path::new(&op.path).is_dir() {
                    fs::remove_dir_all(&op.path)
                } else {
                    fs::remove_file(&op.path)
                }.map_err(|e| ToolError::ExecutionError(e.to_string()))?;
                
                Ok(ToolResult::success(serde_json::json!({
                    "message": "Deleted successfully"
                })))
            },
            _ => Err(ToolError::InvalidParameters(
                format!("Unsupported operation: {}", op.operation)
            ))
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create file management tool (restricted to specific directories)
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
        "You are a file management assistant. You can help users manage files and directories,
         including listing files, reading content, writing files, and deleting files.
         Please ensure operations are safe and confirm important operations with users."
    ).await?;
    
    println!("File management assistant started! I can help manage workspace and documents directories.");
    
    loop {
        print!("\nUser: ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();
        
        if input == "quit" {
            break;
        }
        
        match agent.chat(input).await {
            Ok(response) => println!("File Assistant: {}", response),
            Err(e) => eprintln!("Error: {}", e),
        }
    }
    
    Ok(())
}
```

## 🌐 Web Application Examples

### REST API Service

Creating an intelligent agent API service using Axum framework:

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
    // Initialize logging
    tracing_subscriber::init();
    
    // Create agent
    let config = AgentConfig::builder()
        .model_path("./models/rwkv-4-world-7b.pth")
        .device(Device::Auto)
        .max_tokens(1024)
        .temperature(0.7)
        .build()?;
    
    let agent = Agent::new("APIAgent", config).await?;
    let shared_agent = Arc::new(Mutex::new(agent));
    
    // Create routes
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/chat", post(chat_handler))
        .layer(CorsLayer::permissive())
        .with_state(shared_agent);
    
    // Start server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("RWKV Agent API server started at http://0.0.0.0:3000");
    
    axum::serve(listener, app).await?;
    
    Ok(())
}
```

### WebSocket Real-time Chat

Implementing WebSocket real-time chat functionality:

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
    
    // Send welcome message
    let welcome_msg = WSMessage {
        msg_type: "agent".to_string(),
        content: "Hello! I'm RWKV intelligent assistant. How can I help you?".to_string(),
        timestamp: chrono::Utc::now().timestamp() as u64,
    };
    
    if let Ok(msg) = serde_json::to_string(&welcome_msg) {
        let _ = sender.send(Message::Text(msg)).await;
    }
    
    // Message processing loop
    while let Some(msg) = receiver.next().await {
        if let Ok(msg) = msg {
            match msg {
                Message::Text(text) => {
                    if let Ok(user_msg) = serde_json::from_str::<WSMessage>(&text) {
                        if user_msg.msg_type == "user" {
                            // Process user message
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
                                        content: format!("Error processing message: {}", e),
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
                    println!("WebSocket connection closed");
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
    // Create agent
    let config = AgentConfig::builder()
        .model_path("./models/rwkv-4-world-7b.pth")
        .device(Device::Auto)
        .max_tokens(512)
        .temperature(0.8)
        .build()?;
    
    let agent = Agent::new("WSAgent", config).await?;
    let shared_agent = Arc::new(Mutex::new(agent));
    
    // Create routes
    let app = Router::new()
        .route("/ws", get(websocket_handler))
        .with_state(shared_agent);
    
    // Start server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await?;
    println!("WebSocket server started at ws://0.0.0.0:3001/ws");
    
    axum::serve(listener, app).await?;
    
    Ok(())
}
```

## 🤖 Multi-Agent Collaboration Examples

### News Analysis Team

Multiple agents collaborating to analyze news:

```rust
use rwkv_agent_kit::prelude::*;
use rwkv_agent_kit::multi_agent::*;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create message bus
    let message_bus = MessageBus::new();
    
    // Create news collection agent
    let collector_config = AgentConfig::builder()
        .model_path("./models/rwkv-4-world-7b.pth")
        .device(Device::Auto)
        .build()?;
    
    let mut news_collector = Agent::new("NewsCollector", collector_config).await?
        .with_role("Responsible for collecting and filtering news")
        .with_tools(vec!["web_search", "rss_reader"])
        .connect_to_bus(&message_bus);
    
    // Create sentiment analysis agent
    let sentiment_config = AgentConfig::builder()
        .model_path("./models/rwkv-4-world-7b.pth")
        .device(Device::Auto)
        .build()?;
    
    let mut sentiment_analyzer = Agent::new("SentimentAnalyzer", sentiment_config).await?
        .with_role("Analyze news sentiment")
        .with_tools(vec!["sentiment_analysis"])
        .connect_to_bus(&message_bus);
    
    // Create summary generation agent
    let summary_config = AgentConfig::builder()
        .model_path("./models/rwkv-4-world-7b.pth")
        .device(Device::Auto)
        .build()?;
    
    let mut summarizer = Agent::new("Summarizer", summary_config).await?
        .with_role("Generate news summaries")
        .with_tools(vec!["text_summarization"])
        .connect_to_bus(&message_bus);
    
    // Create report generation agent
    let report_config = AgentConfig::builder()
        .model_path("./models/rwkv-4-world-7b.pth")
        .device(Device::Auto)
        .build()?;
    
    let mut report_generator = Agent::new("ReportGenerator", report_config).await?
        .with_role("Generate final analysis reports")
        .with_tools(vec!["report_generation"])
        .connect_to_bus(&message_bus);
    
    // Collaboration workflow
    let topic = "Latest developments in artificial intelligence";
    
    println!("Starting news analysis collaboration workflow...");
    
    // 1. Collect news
    println!("Step 1: Collecting relevant news");
    let news_data = news_collector.process(&format!("Collect latest news about '{}'", topic)).await?;
    
    // 2. Sentiment analysis
    println!("Step 2: Performing sentiment analysis");
    let sentiment_message = Message::new()
        .from("NewsCollector")
        .to("SentimentAnalyzer")
        .with_content(news_data.clone())
        .with_task("Analyze the sentiment of these news articles");
    
    message_bus.send(sentiment_message).await?;
    let sentiment_result = sentiment_analyzer.receive_and_process().await?;
    
    // 3. Generate summary
    println!("Step 3: Generating news summary");
    let summary_message = Message::new()
        .from("NewsCollector")
        .to("Summarizer")
        .with_content(news_data.clone())
        .with_task("Generate concise summaries for these news articles");
    
    message_bus.send(summary_message).await?;
    let summary_result = summarizer.receive_and_process().await?;
    
    // 4. Generate final report
    println!("Step 4: Generating analysis report");
    let report_data = format!(
        "News content: {}\nSentiment analysis: {}\nSummary: {}",
        news_data, sentiment_result, summary_result
    );
    
    let report_message = Message::new()
        .from("Coordinator")
        .to("ReportGenerator")
        .with_content(report_data)
        .with_task("Generate a comprehensive analysis report based on collected information");
    
    message_bus.send(report_message).await?;
    let final_report = report_generator.receive_and_process().await?;
    
    println!("\n=== News Analysis Report ===");
    println!("{}", final_report);
    
    Ok(())
}
```

## 📱 Mobile Application Examples

### Flutter Integration

Using RWKV Agent Kit in Flutter through FFI:

```rust
// lib.rs - Rust FFI interface
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
            0 // Success
        },
        Err(_) => -1 // Failure
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
                let error = CString::new("Error processing message").unwrap();
                error.into_raw()
            }
        }
    } else {
        let error = CString::new("Agent not initialized").unwrap();
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

Corresponding Flutter code:

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
          text: 'Hello! I\'m RWKV intelligent assistant. How can I help you?',
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
          text: 'Sorry, an error occurred while processing the message.',
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
        title: Text('RWKV Intelligent Assistant'),
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
                      hintText: 'Enter message...',
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

These examples demonstrate the application of RWKV Agent Kit in different scenarios. Each example can serve as a starting point for your projects, which can be modified and extended according to specific requirements.

**More Resources**:
- [API Documentation](/api/) - Detailed interface descriptions
- [Advanced Features](/advanced/) - In-depth understanding of advanced features
- [GitHub Repository](https://github.com/Ai00-X/rwkv-agent-kit) - Get the latest code and examples
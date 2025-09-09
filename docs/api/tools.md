---
title: tools
createTime: 2025/09/08 13:20:23
permalink: /article/8cw1zm6o/
---
# 工具系统 API

## 概述

工具系统是RWKV Agent Kit的核心功能模块，提供了灵活的工具注册、管理和执行机制。支持内置工具和自定义工具，实现了智能的工具选择和参数验证。

## ToolManager

工具管理器负责工具的注册、发现和执行。

```rust
use rwkv_agent_kit::prelude::*;

// 创建工具管理器
let mut tool_manager = ToolManager::new();
```

### 构造方法

#### `new() -> Self`

创建新的工具管理器实例。

#### `with_builtin_tools() -> Self`

创建包含所有内置工具的工具管理器。

```rust
let tool_manager = ToolManager::with_builtin_tools();
```

### 工具注册

#### `register_tool<T>(&mut self, tool: T) -> Result<()>`

注册工具。

**参数:**
- `tool`: 实现了`Tool` trait的工具实例

**示例:**
```rust
// 注册内置工具
tool_manager.register_tool(FileReadTool::new())?;
tool_manager.register_tool(WebSearchTool::new(api_key))?;

// 注册自定义工具
struct CustomTool;

impl Tool for CustomTool {
    fn name(&self) -> &str {
        "custom_tool"
    }
    
    fn description(&self) -> &str {
        "自定义工具示例"
    }
    
    fn parameters(&self) -> ToolParameters {
        ToolParameters::new()
            .add_required("input", ParameterType::String, "输入文本")
            .add_optional("format", ParameterType::String, "输出格式")
    }
    
    async fn execute(&self, params: ToolParameters) -> Result<ToolResult> {
        let input = params.get_string("input")?;
        let format = params.get_string("format").unwrap_or("text".to_string());
        
        // 工具逻辑
        let output = format!("处理结果: {} (格式: {})", input, format);
        
        Ok(ToolResult::success(output))
    }
}

tool_manager.register_tool(CustomTool)?;
```

#### `register_builtin_tools(&mut self) -> Result<()>`

注册所有内置工具。

#### `unregister_tool(&mut self, name: &str) -> Result<()>`

注销工具。

**参数:**
- `name`: 工具名称

### 工具查询

#### `get_tool(&self, name: &str) -> Option<&dyn Tool>`

根据名称获取工具。

**参数:**
- `name`: 工具名称

**返回值:**
- `Option<&dyn Tool>`: 工具引用（如果存在）

#### `list_tools(&self) -> Vec<ToolInfo>`

列出所有已注册的工具。

**返回值:**
- `Vec<ToolInfo>`: 工具信息列表

**示例:**
```rust
let tools = tool_manager.list_tools();

for tool_info in tools {
    println!("工具: {}", tool_info.name);
    println!("描述: {}", tool_info.description);
    println!("参数: {:?}", tool_info.parameters);
}
```

#### `search_tools(&self, query: &str) -> Vec<ToolInfo>`

搜索工具。

**参数:**
- `query`: 搜索查询

**返回值:**
- `Vec<ToolInfo>`: 匹配的工具列表

### 工具执行

#### `execute_tool(&self, name: &str, params: ToolParameters) -> Result<ToolResult>`

执行工具。

**参数:**
- `name`: 工具名称
- `params`: 工具参数

**返回值:**
- `Result<ToolResult>`: 执行结果

**示例:**
```rust
let mut params = ToolParameters::new();
params.set_string("path", "/path/to/file.txt");
params.set_string("encoding", "utf-8");

let result = tool_manager.execute_tool("file_read", params).await?;

if result.is_success() {
    println!("文件内容: {}", result.output);
} else {
    eprintln!("执行失败: {}", result.error.unwrap());
}
```

#### `execute_tool_with_validation(&self, name: &str, params: ToolParameters) -> Result<ToolResult>`

执行工具并进行参数验证。

#### `batch_execute(&self, requests: Vec<ToolRequest>) -> Result<Vec<ToolResult>>`

批量执行工具。

**参数:**
- `requests`: 工具请求列表

**返回值:**
- `Result<Vec<ToolResult>>`: 执行结果列表

### 工具选择

#### `suggest_tools(&self, task_description: &str, context: &Context) -> Result<Vec<ToolSuggestion>>`

根据任务描述建议合适的工具。

**参数:**
- `task_description`: 任务描述
- `context`: 上下文信息

**返回值:**
- `Result<Vec<ToolSuggestion>>`: 工具建议列表

**示例:**
```rust
let task = "我需要读取一个文件并搜索其中的关键词";
let context = Context::default();

let suggestions = tool_manager.suggest_tools(task, &context).await?;

for suggestion in suggestions {
    println!("建议工具: {}", suggestion.tool_name);
    println!("置信度: {:.2}", suggestion.confidence);
    println!("原因: {}", suggestion.reason);
}
```

## Tool Trait

所有工具都必须实现`Tool` trait。

```rust
#[async_trait]
pub trait Tool: Send + Sync {
    /// 工具名称
    fn name(&self) -> &str;
    
    /// 工具描述
    fn description(&self) -> &str;
    
    /// 工具参数定义
    fn parameters(&self) -> ToolParameters;
    
    /// 执行工具
    async fn execute(&self, params: ToolParameters) -> Result<ToolResult>;
    
    /// 工具类别（可选）
    fn category(&self) -> ToolCategory {
        ToolCategory::General
    }
    
    /// 工具标签（可选）
    fn tags(&self) -> Vec<String> {
        vec![]
    }
    
    /// 是否需要网络访问（可选）
    fn requires_network(&self) -> bool {
        false
    }
    
    /// 是否需要文件系统访问（可选）
    fn requires_filesystem(&self) -> bool {
        false
    }
}
```

### 自定义工具示例

```rust
use async_trait::async_trait;
use serde_json::Value;

struct CalculatorTool;

#[async_trait]
impl Tool for CalculatorTool {
    fn name(&self) -> &str {
        "calculator"
    }
    
    fn description(&self) -> &str {
        "执行基本数学计算"
    }
    
    fn parameters(&self) -> ToolParameters {
        ToolParameters::new()
            .add_required("expression", ParameterType::String, "数学表达式")
            .add_optional("precision", ParameterType::Integer, "小数精度")
    }
    
    fn category(&self) -> ToolCategory {
        ToolCategory::Math
    }
    
    fn tags(&self) -> Vec<String> {
        vec!["计算".to_string(), "数学".to_string()]
    }
    
    async fn execute(&self, params: ToolParameters) -> Result<ToolResult> {
        let expression = params.get_string("expression")?;
        let precision = params.get_integer("precision").unwrap_or(2);
        
        // 解析和计算表达式
        match self.evaluate_expression(&expression) {
            Ok(result) => {
                let formatted = format!("{:.precision$}", result, precision = precision as usize);
                Ok(ToolResult::success(formatted))
            }
            Err(e) => {
                Ok(ToolResult::error(format!("计算错误: {}", e)))
            }
        }
    }
}

impl CalculatorTool {
    fn evaluate_expression(&self, expr: &str) -> Result<f64> {
        // 简单的表达式求值实现
        // 实际应用中可以使用更复杂的解析器
        expr.parse::<f64>()
            .map_err(|e| format!("无法解析表达式: {}", e).into())
    }
}
```

## 内置工具

### 文件操作工具

#### FileReadTool

读取文件内容。

**参数:**
- `path` (必需): 文件路径
- `encoding` (可选): 文件编码，默认UTF-8

**示例:**
```rust
let mut params = ToolParameters::new();
params.set_string("path", "./data/input.txt");

let result = tool_manager.execute_tool("file_read", params).await?;
```

#### FileWriteTool

写入文件内容。

**参数:**
- `path` (必需): 文件路径
- `content` (必需): 文件内容
- `encoding` (可选): 文件编码，默认UTF-8
- `append` (可选): 是否追加，默认false

#### DirectoryListTool

列出目录内容。

**参数:**
- `path` (必需): 目录路径
- `recursive` (可选): 是否递归，默认false
- `include_hidden` (可选): 是否包含隐藏文件，默认false

### 网络工具

#### WebSearchTool

网络搜索工具。

**参数:**
- `query` (必需): 搜索查询
- `num_results` (可选): 结果数量，默认10
- `language` (可选): 搜索语言，默认auto

**示例:**
```rust
let mut params = ToolParameters::new();
params.set_string("query", "Rust编程语言");
params.set_integer("num_results", 5);

let result = tool_manager.execute_tool("web_search", params).await?;
```

#### HttpRequestTool

HTTP请求工具。

**参数:**
- `url` (必需): 请求URL
- `method` (可选): HTTP方法，默认GET
- `headers` (可选): 请求头
- `body` (可选): 请求体

### 文本处理工具

#### TextSummaryTool

文本摘要工具。

**参数:**
- `text` (必需): 输入文本
- `max_length` (可选): 最大摘要长度
- `language` (可选): 文本语言

#### TextTranslateTool

文本翻译工具。

**参数:**
- `text` (必需): 输入文本
- `target_language` (必需): 目标语言
- `source_language` (可选): 源语言，默认auto

### 数据处理工具

#### JsonParseTool

JSON解析工具。

**参数:**
- `json_string` (必需): JSON字符串
- `query` (可选): JSONPath查询

#### CsvParseTool

CSV解析工具。

**参数:**
- `csv_data` (必需): CSV数据
- `delimiter` (可选): 分隔符，默认逗号
- `has_header` (可选): 是否有标题行，默认true

## 数据类型

### ToolParameters

工具参数容器。

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolParameters {
    parameters: HashMap<String, ParameterDefinition>,
    values: HashMap<String, Value>,
}

impl ToolParameters {
    pub fn new() -> Self;
    pub fn add_required(mut self, name: &str, param_type: ParameterType, description: &str) -> Self;
    pub fn add_optional(mut self, name: &str, param_type: ParameterType, description: &str) -> Self;
    pub fn set_string(&mut self, name: &str, value: &str);
    pub fn set_integer(&mut self, name: &str, value: i64);
    pub fn set_float(&mut self, name: &str, value: f64);
    pub fn set_boolean(&mut self, name: &str, value: bool);
    pub fn get_string(&self, name: &str) -> Result<String>;
    pub fn get_integer(&self, name: &str) -> Result<i64>;
    pub fn get_float(&self, name: &str) -> Result<f64>;
    pub fn get_boolean(&self, name: &str) -> Result<bool>;
}
```

### ToolResult

工具执行结果。

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult {
    /// 是否成功
    pub success: bool,
    /// 输出内容
    pub output: String,
    /// 错误信息（如果失败）
    pub error: Option<String>,
    /// 执行时间（毫秒）
    pub execution_time_ms: u64,
    /// 元数据
    pub metadata: HashMap<String, Value>,
}

impl ToolResult {
    pub fn success(output: String) -> Self;
    pub fn error(error: String) -> Self;
    pub fn is_success(&self) -> bool;
    pub fn is_error(&self) -> bool;
}
```

### ToolInfo

工具信息。

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolInfo {
    /// 工具名称
    pub name: String,
    /// 工具描述
    pub description: String,
    /// 参数定义
    pub parameters: ToolParameters,
    /// 工具类别
    pub category: ToolCategory,
    /// 工具标签
    pub tags: Vec<String>,
    /// 是否需要网络
    pub requires_network: bool,
    /// 是否需要文件系统
    pub requires_filesystem: bool,
}
```

### ToolCategory

工具类别枚举。

```rust
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ToolCategory {
    /// 通用工具
    General,
    /// 文件操作
    FileSystem,
    /// 网络工具
    Network,
    /// 文本处理
    TextProcessing,
    /// 数据处理
    DataProcessing,
    /// 数学计算
    Math,
    /// 图像处理
    ImageProcessing,
    /// 音频处理
    AudioProcessing,
    /// 系统工具
    System,
    /// 自定义
    Custom(String),
}
```

## 完整示例

```rust
use rwkv_agent_kit::prelude::*;
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    // 创建工具管理器并注册内置工具
    let mut tool_manager = ToolManager::with_builtin_tools();
    
    // 注册自定义工具
    tool_manager.register_tool(CalculatorTool)?;
    
    // 列出所有工具
    let tools = tool_manager.list_tools();
    println!("可用工具 ({} 个):", tools.len());
    for tool in tools {
        println!("  - {}: {}", tool.name, tool.description);
    }
    
    // 执行文件读取
    let mut read_params = ToolParameters::new();
    read_params.set_string("path", "./README.md");
    
    let read_result = tool_manager.execute_tool("file_read", read_params).await?;
    if read_result.is_success() {
        println!("文件读取成功，内容长度: {} 字符", read_result.output.len());
    }
    
    // 执行网络搜索
    let mut search_params = ToolParameters::new();
    search_params.set_string("query", "RWKV语言模型");
    search_params.set_integer("num_results", 3);
    
    let search_result = tool_manager.execute_tool("web_search", search_params).await?;
    if search_result.is_success() {
        println!("搜索完成: {}", search_result.output);
    }
    
    // 执行计算
    let mut calc_params = ToolParameters::new();
    calc_params.set_string("expression", "42");
    calc_params.set_integer("precision", 2);
    
    let calc_result = tool_manager.execute_tool("calculator", calc_params).await?;
    if calc_result.is_success() {
        println!("计算结果: {}", calc_result.output);
    }
    
    // 工具建议
    let task = "我需要处理一个CSV文件并生成摘要";
    let context = Context::default();
    
    let suggestions = tool_manager.suggest_tools(task, &context).await?;
    println!("任务建议的工具:");
    for suggestion in suggestions {
        println!("  - {} (置信度: {:.2})", suggestion.tool_name, suggestion.confidence);
    }
    
    // 批量执行
    let requests = vec![
        ToolRequest {
            tool_name: "calculator".to_string(),
            parameters: {
                let mut params = ToolParameters::new();
                params.set_string("expression", "10");
                params
            },
        },
        ToolRequest {
            tool_name: "calculator".to_string(),
            parameters: {
                let mut params = ToolParameters::new();
                params.set_string("expression", "20");
                params
            },
        },
    ];
    
    let batch_results = tool_manager.batch_execute(requests).await?;
    println!("批量执行结果:");
    for (i, result) in batch_results.iter().enumerate() {
        println!("  请求 {}: {}", i + 1, result.output);
    }
    
    Ok(())
}
```

## 下一步

- [记忆管理API](./memory.md)
- [数据库API](./database.md)
- [核心类型定义](./types.md)
- [配置选项](../config/README.md)
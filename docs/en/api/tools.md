---
title: tools
createTime: 2025/09/08 22:30:24
permalink: /en/article/rvty3nke/
---
# Tools API

The Tools API provides functionality for extending agent capabilities with external tools and services.

## Tool

Trait for implementing custom tools that agents can use.

### Required Methods

#### `name(&self) -> &str`

Returns the tool's name.

**Returns:**
- `&str`: Tool name

#### `description(&self) -> &str`

Returns a description of what the tool does.

**Returns:**
- `&str`: Tool description

#### `parameters(&self) -> serde_json::Value`

Returns the JSON schema for tool parameters.

**Returns:**
- `serde_json::Value`: JSON schema describing parameters

#### `execute(&self, params: serde_json::Value) -> Result<ToolResult, ToolError>`

Executes the tool with given parameters.

**Parameters:**
- `params`: Tool parameters as JSON

**Returns:**
- `Result<ToolResult, ToolError>`: Tool result or error

### Example Implementation

```rust
use rwkv_agent_kit::tools::{Tool, ToolResult, ToolError};
use async_trait::async_trait;
use serde_json::Value;

#[derive(Debug)]
struct Calculator;

#[async_trait]
impl Tool for Calculator {
    fn name(&self) -> &str {
        "calculator"
    }
    
    fn description(&self) -> &str {
        "Performs basic mathematical calculations"
    }
    
    fn parameters(&self) -> Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "expression": {
                    "type": "string",
                    "description": "Mathematical expression to evaluate"
                }
            },
            "required": ["expression"]
        })
    }
    
    async fn execute(&self, params: Value) -> Result<ToolResult, ToolError> {
        let expression = params["expression"].as_str()
            .ok_or_else(|| ToolError::InvalidParameters("Missing expression".to_string()))?;
        
        // Evaluate expression (simplified)
        let result = self.evaluate(expression)?;
        
        Ok(ToolResult::success(serde_json::json!({
            "result": result,
            "expression": expression
        })))
    }
}
```

## ToolResult

Represents the result of a tool execution.

### Fields

- `success: bool` - Whether the tool executed successfully
- `data: serde_json::Value` - Result data
- `message: Option<String>` - Optional message
- `metadata: Option<ToolMetadata>` - Optional metadata

### Constructor Methods

#### `ToolResult::success(data: serde_json::Value) -> ToolResult`

Creates a successful result.

**Parameters:**
- `data`: Result data

**Returns:**
- `ToolResult`: Success result

#### `ToolResult::error(message: &str) -> ToolResult`

Creates an error result.

**Parameters:**
- `message`: Error message

**Returns:**
- `ToolResult`: Error result

#### `ToolResult::with_message(self, message: &str) -> ToolResult`

Adds a message to the result.

**Parameters:**
- `message`: Message text

**Returns:**
- `ToolResult`: Updated result

#### `ToolResult::with_metadata(self, metadata: ToolMetadata) -> ToolResult`

Adds metadata to the result.

**Parameters:**
- `metadata`: Tool metadata

**Returns:**
- `ToolResult`: Updated result

## Built-in Tools

### Calculator

Performs mathematical calculations.

#### Constructor

```rust
use rwkv_agent_kit::tools::Calculator;

let calculator = Calculator::new();
```

#### Parameters

- `expression: string` - Mathematical expression to evaluate

#### Example Usage

```rust
let result = calculator.execute(serde_json::json!({
    "expression": "2 + 3 * 4"
})).await?;

println!("Result: {}", result.data["result"]); // 14
```

### WebSearch

Performs web searches using a search API.

#### Constructor

```rust
use rwkv_agent_kit::tools::WebSearch;

let web_search = WebSearch::new("your-api-key");
```

#### Parameters

- `query: string` - Search query
- `limit: number` (optional) - Maximum number of results (default: 5)
- `language: string` (optional) - Search language (default: "en")

#### Example Usage

```rust
let result = web_search.execute(serde_json::json!({
    "query": "Rust programming language",
    "limit": 3
})).await?;

for item in result.data["results"].as_array().unwrap() {
    println!("Title: {}", item["title"]);
    println!("URL: {}", item["url"]);
}
```

### FileReader

Reads content from files.

#### Constructor

```rust
use rwkv_agent_kit::tools::FileReader;

let file_reader = FileReader::new();
```

#### Parameters

- `path: string` - File path to read
- `encoding: string` (optional) - File encoding (default: "utf-8")

#### Example Usage

```rust
let result = file_reader.execute(serde_json::json!({
    "path": "/path/to/file.txt"
})).await?;

println!("Content: {}", result.data["content"]);
```

### FileWriter

Writes content to files.

#### Constructor

```rust
use rwkv_agent_kit::tools::FileWriter;

let file_writer = FileWriter::new();
```

#### Parameters

- `path: string` - File path to write
- `content: string` - Content to write
- `append: boolean` (optional) - Whether to append (default: false)

#### Example Usage

```rust
let result = file_writer.execute(serde_json::json!({
    "path": "/path/to/output.txt",
    "content": "Hello, world!",
    "append": false
})).await?;
```

### HttpClient

Makes HTTP requests.

#### Constructor

```rust
use rwkv_agent_kit::tools::HttpClient;

let http_client = HttpClient::new();
```

#### Parameters

- `url: string` - Request URL
- `method: string` (optional) - HTTP method (default: "GET")
- `headers: object` (optional) - Request headers
- `body: string` (optional) - Request body

#### Example Usage

```rust
let result = http_client.execute(serde_json::json!({
    "url": "https://api.example.com/data",
    "method": "POST",
    "headers": {
        "Content-Type": "application/json"
    },
    "body": "{\"key\": \"value\"}"
})).await?;

println!("Status: {}", result.data["status"]);
println!("Response: {}", result.data["body"]);
```

### DatabaseQuery

Executes database queries.

#### Constructor

```rust
use rwkv_agent_kit::tools::DatabaseQuery;

let db_query = DatabaseQuery::new("postgresql://user:pass@localhost/db").await?;
```

#### Parameters

- `query: string` - SQL query to execute
- `params: array` (optional) - Query parameters

#### Example Usage

```rust
let result = db_query.execute(serde_json::json!({
    "query": "SELECT * FROM users WHERE age > $1",
    "params": [25]
})).await?;

for row in result.data["rows"].as_array().unwrap() {
    println!("User: {}", row);
}
```

## ToolManager

Manages a collection of tools for an agent.

### Constructor

#### `ToolManager::new() -> ToolManager`

Creates a new tool manager.

**Returns:**
- `ToolManager`: New tool manager

### Methods

#### `add_tool(&mut self, tool: Box<dyn Tool>) -> Result<(), ToolError>`

Adds a tool to the manager.

**Parameters:**
- `tool`: Tool to add

**Returns:**
- `Result<(), ToolError>`: Success or error

#### `remove_tool(&mut self, name: &str) -> Result<(), ToolError>`

Removes a tool by name.

**Parameters:**
- `name`: Tool name

**Returns:**
- `Result<(), ToolError>`: Success or error

#### `get_tool(&self, name: &str) -> Option<&dyn Tool>`

Gets a tool by name.

**Parameters:**
- `name`: Tool name

**Returns:**
- `Option<&dyn Tool>`: Tool reference or None

#### `list_tools(&self) -> Vec<&str>`

Lists all tool names.

**Returns:**
- `Vec<&str>`: List of tool names

#### `execute_tool(&self, name: &str, params: serde_json::Value) -> Result<ToolResult, ToolError>`

Executes a tool by name.

**Parameters:**
- `name`: Tool name
- `params`: Tool parameters

**Returns:**
- `Result<ToolResult, ToolError>`: Tool result or error

#### `get_tool_schema(&self, name: &str) -> Option<serde_json::Value>`

Gets the parameter schema for a tool.

**Parameters:**
- `name`: Tool name

**Returns:**
- `Option<serde_json::Value>`: Parameter schema or None

## CompositeToolBuilder

Builder for creating composite tools that combine multiple tools.

### Constructor

#### `CompositeToolBuilder::new() -> CompositeToolBuilder`

Creates a new composite tool builder.

**Returns:**
- `CompositeToolBuilder`: New builder

### Builder Methods

#### `add_tool(self, tool: Box<dyn Tool>) -> CompositeToolBuilder`

Adds a tool to the composite.

**Parameters:**
- `tool`: Tool to add

**Returns:**
- `CompositeToolBuilder`: Updated builder

#### `with_execution_strategy(self, strategy: ExecutionStrategy) -> CompositeToolBuilder`

Sets the execution strategy.

**Parameters:**
- `strategy`: Execution strategy

**Returns:**
- `CompositeToolBuilder`: Updated builder

#### `with_name(self, name: &str) -> CompositeToolBuilder`

Sets the composite tool name.

**Parameters:**
- `name`: Tool name

**Returns:**
- `CompositeToolBuilder`: Updated builder

#### `build(self) -> Result<CompositeTool, ToolError>`

Builds the composite tool.

**Returns:**
- `Result<CompositeTool, ToolError>`: Composite tool or error

## ExecutionStrategy

Enum defining how composite tools execute their sub-tools.

### Variants

- `Sequential` - Execute tools one after another
- `Parallel` - Execute tools concurrently
- `Conditional` - Execute tools based on conditions
- `Pipeline` - Pass output from one tool to the next

## ToolError

Error types for tool operations.

### Variants

- `ExecutionError(String)` - Tool execution errors
- `InvalidParameters(String)` - Invalid parameter errors
- `NotFound(String)` - Tool not found
- `NetworkError(String)` - Network-related errors
- `FileError(std::io::Error)` - File operation errors
- `SerializationError(String)` - Serialization errors
- `AuthenticationError(String)` - Authentication errors
- `RateLimitError(String)` - Rate limiting errors

### Methods

#### `to_string(&self) -> String`

Converts the error to a string.

**Returns:**
- `String`: Error description

## Examples

### Creating a Custom Tool

```rust
use rwkv_agent_kit::tools::{Tool, ToolResult, ToolError};
use async_trait::async_trait;
use serde_json::Value;

#[derive(Debug)]
struct WeatherTool {
    api_key: String,
}

impl WeatherTool {
    fn new(api_key: &str) -> Self {
        Self {
            api_key: api_key.to_string(),
        }
    }
}

#[async_trait]
impl Tool for WeatherTool {
    fn name(&self) -> &str {
        "get_weather"
    }
    
    fn description(&self) -> &str {
        "Get current weather information for a location"
    }
    
    fn parameters(&self) -> Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "location": {
                    "type": "string",
                    "description": "City name or coordinates"
                },
                "units": {
                    "type": "string",
                    "enum": ["metric", "imperial"],
                    "default": "metric"
                }
            },
            "required": ["location"]
        })
    }
    
    async fn execute(&self, params: Value) -> Result<ToolResult, ToolError> {
        let location = params["location"].as_str()
            .ok_or_else(|| ToolError::InvalidParameters("Missing location".to_string()))?;
        
        let units = params["units"].as_str().unwrap_or("metric");
        
        // Make API call to weather service
        let weather_data = self.fetch_weather(location, units).await?;
        
        Ok(ToolResult::success(weather_data))
    }
}

impl WeatherTool {
    async fn fetch_weather(&self, location: &str, units: &str) -> Result<Value, ToolError> {
        // Implementation would make actual API call
        Ok(serde_json::json!({
            "location": location,
            "temperature": 22.5,
            "humidity": 65,
            "condition": "Partly cloudy",
            "units": units
        }))
    }
}
```

### Using Tools with Agents

```rust
use rwkv_agent_kit::{RwkvAgentKit, AgentConfig};
use rwkv_agent_kit::tools::{Calculator, WebSearch};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let kit = RwkvAgentKit::new("config.toml").await?;
    
    // Create agent with tools
    let agent = kit.create_agent(
        AgentConfig::new()
            .with_name("assistant")
            .with_tool(Box::new(Calculator::new()))
            .with_tool(Box::new(WebSearch::new("api-key")))
            .with_tool(Box::new(WeatherTool::new("weather-api-key")))
    ).await?;
    
    // Agent can now use tools
    let response = agent.chat("What's the weather like in Tokyo and what's 15 * 23?").await?;
    println!("Agent: {}", response);
    
    Ok(())
}
```

### Composite Tools

```rust
use rwkv_agent_kit::tools::{CompositeToolBuilder, ExecutionStrategy};

// Create a composite tool that combines multiple tools
let research_tool = CompositeToolBuilder::new()
    .with_name("research_assistant")
    .add_tool(Box::new(WebSearch::new("api-key")))
    .add_tool(Box::new(FileWriter::new()))
    .add_tool(Box::new(Calculator::new()))
    .with_execution_strategy(ExecutionStrategy::Pipeline)
    .build()?;

// Add to agent
agent.add_tool(Box::new(research_tool)).await?;
```
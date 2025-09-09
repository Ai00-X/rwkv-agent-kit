---
title: agent
createTime: 2025/09/08 22:28:53
permalink: /en/article/1cpj76ss/
---
# Agent API

The Agent API provides the core functionality for creating and managing intelligent agents.

## Agent

The main agent struct that handles conversations and interactions.

### Constructor

#### `Agent::new(config: AgentConfig) -> Result<Agent, AgentError>`

Creates a new agent with the specified configuration.

**Parameters:**
- `config`: Agent configuration settings

**Returns:**
- `Result<Agent, AgentError>`: The created agent or an error

**Example:**
```rust
use rwkv_agent_kit::{Agent, AgentConfig};

let config = AgentConfig::new()
    .with_name("assistant")
    .with_system_prompt("You are a helpful assistant.");

let agent = Agent::new(config).await?;
```

### Methods

#### `chat(&self, message: &str) -> Result<String, AgentError>`

Sends a message to the agent and returns the response.

**Parameters:**
- `message`: The user's message

**Returns:**
- `Result<String, AgentError>`: The agent's response or an error

**Example:**
```rust
let response = agent.chat("Hello, how are you?").await?;
println!("Agent: {}", response);
```

#### `chat_with_options(&self, message: &str, options: ChatOptions) -> Result<String, AgentError>`

Sends a message with custom options.

**Parameters:**
- `message`: The user's message
- `options`: Chat configuration options

**Returns:**
- `Result<String, AgentError>`: The agent's response or an error

**Example:**
```rust
let options = ChatOptions::new()
    .with_temperature(0.8)
    .with_max_tokens(512);

let response = agent.chat_with_options("Tell me a story", options).await?;
```

#### `get_id(&self) -> &str`

Returns the agent's unique identifier.

**Returns:**
- `&str`: The agent's ID

#### `get_name(&self) -> &str`

Returns the agent's name.

**Returns:**
- `&str`: The agent's name

#### `set_system_prompt(&mut self, prompt: &str) -> Result<(), AgentError>`

Updates the agent's system prompt.

**Parameters:**
- `prompt`: The new system prompt

**Returns:**
- `Result<(), AgentError>`: Success or error

#### `get_conversation_history(&self) -> Result<Vec<Message>, AgentError>`

Retrieves the conversation history.

**Returns:**
- `Result<Vec<Message>, AgentError>`: List of messages or error

#### `clear_conversation(&mut self) -> Result<(), AgentError>`

Clears the conversation history.

**Returns:**
- `Result<(), AgentError>`: Success or error

#### `add_tool(&mut self, tool: Box<dyn Tool>) -> Result<(), AgentError>`

Adds a tool to the agent.

**Parameters:**
- `tool`: The tool to add

**Returns:**
- `Result<(), AgentError>`: Success or error

**Example:**
```rust
use rwkv_agent_kit::tools::Calculator;

agent.add_tool(Box::new(Calculator::new())).await?;
```

#### `remove_tool(&mut self, tool_name: &str) -> Result<(), AgentError>`

Removes a tool from the agent.

**Parameters:**
- `tool_name`: Name of the tool to remove

**Returns:**
- `Result<(), AgentError>`: Success or error

#### `list_tools(&self) -> Vec<&str>`

Lists all available tools.

**Returns:**
- `Vec<&str>`: List of tool names

#### `get_memory_stats(&self) -> Result<MemoryStats, AgentError>`

Retrieves memory usage statistics.

**Returns:**
- `Result<MemoryStats, AgentError>`: Memory statistics or error

#### `search_memories(&self, query: &str, options: SearchOptions) -> Result<Vec<Memory>, AgentError>`

Searches through the agent's memories.

**Parameters:**
- `query`: Search query
- `options`: Search configuration

**Returns:**
- `Result<Vec<Memory>, AgentError>`: Matching memories or error

#### `save_memory(&mut self, content: &str, metadata: Option<MemoryMetadata>) -> Result<String, AgentError>`

Saves a memory with optional metadata.

**Parameters:**
- `content`: Memory content
- `metadata`: Optional metadata

**Returns:**
- `Result<String, AgentError>`: Memory ID or error

#### `delete_memory(&mut self, memory_id: &str) -> Result<(), AgentError>`

Deletes a specific memory.

**Parameters:**
- `memory_id`: ID of the memory to delete

**Returns:**
- `Result<(), AgentError>`: Success or error

#### `clone(&self) -> Agent`

Creates a clone of the agent with shared state.

**Returns:**
- `Agent`: Cloned agent

## AgentConfig

Configuration struct for creating agents.

### Constructor

#### `AgentConfig::new() -> AgentConfig`

Creates a new agent configuration with default values.

**Returns:**
- `AgentConfig`: Default configuration

#### `AgentConfig::default() -> AgentConfig`

Creates a default configuration (same as `new()`).

**Returns:**
- `AgentConfig`: Default configuration

### Builder Methods

#### `with_name(self, name: &str) -> AgentConfig`

Sets the agent's name.

**Parameters:**
- `name`: Agent name

**Returns:**
- `AgentConfig`: Updated configuration

#### `with_system_prompt(self, prompt: &str) -> AgentConfig`

Sets the system prompt.

**Parameters:**
- `prompt`: System prompt text

**Returns:**
- `AgentConfig`: Updated configuration

#### `with_model_path(self, path: &str) -> AgentConfig`

Sets the model file path.

**Parameters:**
- `path`: Path to the model file

**Returns:**
- `AgentConfig`: Updated configuration

#### `with_max_tokens(self, max_tokens: usize) -> AgentConfig`

Sets the maximum number of tokens for responses.

**Parameters:**
- `max_tokens`: Maximum token count

**Returns:**
- `AgentConfig`: Updated configuration

#### `with_temperature(self, temperature: f32) -> AgentConfig`

Sets the sampling temperature.

**Parameters:**
- `temperature`: Temperature value (0.0 to 2.0)

**Returns:**
- `AgentConfig`: Updated configuration

#### `with_top_p(self, top_p: f32) -> AgentConfig`

Sets the nucleus sampling parameter.

**Parameters:**
- `top_p`: Top-p value (0.0 to 1.0)

**Returns:**
- `AgentConfig`: Updated configuration

#### `with_memory_limit(self, limit: usize) -> AgentConfig`

Sets the memory limit for conversation history.

**Parameters:**
- `limit`: Maximum number of messages to remember

**Returns:**
- `AgentConfig`: Updated configuration

#### `with_tool(self, tool: Box<dyn Tool>) -> AgentConfig`

Adds a tool to the agent configuration.

**Parameters:**
- `tool`: Tool to add

**Returns:**
- `AgentConfig`: Updated configuration

#### `with_memory_backend(self, backend: Box<dyn MemoryBackend>) -> AgentConfig`

Sets a custom memory backend.

**Parameters:**
- `backend`: Memory backend implementation

**Returns:**
- `AgentConfig`: Updated configuration

## ChatOptions

Options for customizing individual chat requests.

### Constructor

#### `ChatOptions::new() -> ChatOptions`

Creates new chat options with default values.

**Returns:**
- `ChatOptions`: Default options

### Builder Methods

#### `with_temperature(self, temperature: f32) -> ChatOptions`

Sets the temperature for this chat.

**Parameters:**
- `temperature`: Temperature value

**Returns:**
- `ChatOptions`: Updated options

#### `with_max_tokens(self, max_tokens: usize) -> ChatOptions`

Sets the maximum tokens for this chat.

**Parameters:**
- `max_tokens`: Maximum token count

**Returns:**
- `ChatOptions`: Updated options

#### `with_top_p(self, top_p: f32) -> ChatOptions`

Sets the top-p value for this chat.

**Parameters:**
- `top_p`: Top-p value

**Returns:**
- `ChatOptions`: Updated options

#### `with_stop_sequences(self, sequences: Vec<String>) -> ChatOptions`

Sets stop sequences for generation.

**Parameters:**
- `sequences`: List of stop sequences

**Returns:**
- `ChatOptions`: Updated options

#### `with_context(self, context: &str) -> ChatOptions`

Adds additional context for this chat.

**Parameters:**
- `context`: Context information

**Returns:**
- `ChatOptions`: Updated options

## Message

Represents a message in the conversation.

### Fields

- `id: String` - Unique message identifier
- `role: MessageRole` - Message role (User, Assistant, System)
- `content: String` - Message content
- `timestamp: DateTime<Utc>` - When the message was created
- `metadata: Option<MessageMetadata>` - Optional metadata

### Methods

#### `new(role: MessageRole, content: &str) -> Message`

Creates a new message.

**Parameters:**
- `role`: Message role
- `content`: Message content

**Returns:**
- `Message`: New message

## MessageRole

Enum representing message roles.

### Variants

- `User` - Message from the user
- `Assistant` - Message from the agent
- `System` - System message
- `Tool` - Message from a tool

## AgentError

Error types that can occur during agent operations.

### Variants

- `ModelError(String)` - Model-related errors
- `MemoryError(String)` - Memory system errors
- `ToolError(String)` - Tool execution errors
- `ConfigError(String)` - Configuration errors
- `IoError(std::io::Error)` - I/O errors
- `SerializationError(String)` - Serialization errors

### Methods

#### `to_string(&self) -> String`

Converts the error to a string representation.

**Returns:**
- `String`: Error description

## Examples

### Basic Agent Usage

```rust
use rwkv_agent_kit::{RwkvAgentKit, AgentConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let kit = RwkvAgentKit::new("config.toml").await?;
    
    let agent = kit.create_agent(
        AgentConfig::new()
            .with_name("helper")
            .with_system_prompt("You are a helpful assistant.")
            .with_temperature(0.7)
    ).await?;
    
    let response = agent.chat("What is Rust?").await?;
    println!("Agent: {}", response);
    
    Ok(())
}
```

### Agent with Tools

```rust
use rwkv_agent_kit::{RwkvAgentKit, AgentConfig};
use rwkv_agent_kit::tools::{Calculator, WebSearch};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let kit = RwkvAgentKit::new("config.toml").await?;
    
    let agent = kit.create_agent(
        AgentConfig::new()
            .with_tool(Box::new(Calculator::new()))
            .with_tool(Box::new(WebSearch::new("api-key")))
    ).await?;
    
    let response = agent.chat("What's 15 * 23?").await?;
    println!("Agent: {}", response);
    
    Ok(())
}
```

### Memory Management

```rust
// Save important information
agent.save_memory("User prefers concise responses", None).await?;

// Search memories
let memories = agent.search_memories(
    "preferences",
    SearchOptions::new().with_limit(5)
).await?;

for memory in memories {
    println!("Remembered: {}", memory.content);
}
```
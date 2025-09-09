---
title: tool-development
createTime: 2025/09/08 15:42:56
permalink: /en/article/rk5pb9gl/
---
# Tool Development

RWKV Agent Kit provides a powerful tool extension system that supports custom tool development, tool chain composition, and tool permission management. This chapter will detail how to develop and use custom tools.

## 🛠️ Advanced Tool System

### Custom Tool Development

Develop custom tools to extend agent capabilities:

```rust
use rwkv_agent_kit::tools::{Tool, ToolResult, ToolError};
use serde::{Deserialize, Serialize};
use async_trait::async_trait;

#[derive(Debug, Serialize, Deserialize)]
struct WeatherToolInput {
    city: String,
    country: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct WeatherToolOutput {
    temperature: f32,
    humidity: f32,
    description: String,
}

struct WeatherTool {
    api_key: String,
}

#[async_trait]
impl Tool for WeatherTool {
    type Input = WeatherToolInput;
    type Output = WeatherToolOutput;

    fn name(&self) -> &str {
        "weather_query"
    }

    fn description(&self) -> &str {
        "Query weather information for a specified city"
    }

    async fn execute(&self, input: Self::Input) -> Result<ToolResult<Self::Output>, ToolError> {
        // Call weather API
        let weather_data = self.fetch_weather(&input.city, input.country.as_deref()).await?;
        
        let output = WeatherToolOutput {
            temperature: weather_data.temperature,
            humidity: weather_data.humidity,
            description: weather_data.description,
        };

        Ok(ToolResult::success(output))
    }
}

impl WeatherTool {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }

    async fn fetch_weather(&self, city: &str, country: Option<&str>) -> Result<WeatherData, ToolError> {
        // Implement weather API call logic
        // ...
        todo!("Implement weather API call")
    }
}
```

### Tool Chain Composition

Combine multiple tools into complex workflows:

```rust
use rwkv_agent_kit::tools::{ToolChain, ToolStep, ConditionalStep};

// Create tool chain
let mut chain = ToolChain::new("research_workflow");

// Add tool steps
chain.add_step(ToolStep::new("web_search")
    .with_input(json!({
        "query": "{{user_query}}",
        "max_results": 5
    }))
    .with_output_mapping("search_results"));

// Conditional step
chain.add_step(ConditionalStep::new()
    .when("search_results.length > 0")
    .then(ToolStep::new("content_summarizer")
        .with_input(json!({
            "content": "{{search_results}}",
            "max_length": 500
        }))
        .with_output_mapping("summary"))
    .otherwise(ToolStep::new("fallback_response")
        .with_input(json!({
            "message": "No relevant information found"
        }))));

// Final processing step
chain.add_step(ToolStep::new("response_formatter")
    .with_input(json!({
        "summary": "{{summary}}",
        "sources": "{{search_results}}"
    })));

// Execute tool chain
let context = json!({
    "user_query": "Latest developments in artificial intelligence"
});

let result = chain.execute(context).await?;
println!("Tool chain execution result: {:?}", result);
```

### Tool Permission Management

Implement fine-grained tool access control:

```rust
use rwkv_agent_kit::tools::{ToolPermission, PermissionManager, AccessLevel};

// Create permission manager
let mut permission_manager = PermissionManager::new();

// Define tool permissions
let file_read_permission = ToolPermission::new("file_read")
    .with_access_level(AccessLevel::Read)
    .with_resource_pattern("/safe/directory/*")
    .with_rate_limit(100, Duration::from_minutes(1));

let file_write_permission = ToolPermission::new("file_write")
    .with_access_level(AccessLevel::Write)
    .with_resource_pattern("/safe/directory/*")
    .with_rate_limit(10, Duration::from_minutes(1))
    .with_approval_required(true);

let system_command_permission = ToolPermission::new("system_command")
    .with_access_level(AccessLevel::Execute)
    .with_whitelist(vec!["ls", "cat", "grep"])
    .with_blacklist(vec!["rm", "sudo", "chmod"])
    .with_sandbox(true);

// Register permissions
permission_manager.register_permission(file_read_permission);
permission_manager.register_permission(file_write_permission);
permission_manager.register_permission(system_command_permission);

// Check permissions
let can_read = permission_manager
    .check_permission("agent_001", "file_read", "/safe/directory/data.txt")
    .await?;

if can_read {
    println!("File read allowed");
} else {
    println!("Access denied");
}
```

## Tool Development Best Practices

### Tool Design Principles

1. **Single Responsibility**: Each tool focuses on one specific function
2. **Idempotency**: Same input should produce same output
3. **Error Handling**: Gracefully handle various exception cases
4. **Complete Documentation**: Provide clear usage instructions

### Tool Interface Design

```rust
use rwkv_agent_kit::tools::{ToolMetadata, ParameterSchema, ToolCategory};

// Define tool metadata
let metadata = ToolMetadata::new("data_processor")
    .with_description("Process and transform data")
    .with_category(ToolCategory::DataProcessing)
    .with_version("1.0.0")
    .with_author("Developer")
    .with_tags(vec!["data", "processing", "transformation"]);

// Define parameter schema
let input_schema = ParameterSchema::object()
    .with_property("data", ParameterSchema::string().required())
    .with_property("format", ParameterSchema::string()
        .with_enum(vec!["json", "csv", "xml"])
        .with_default("json"))
    .with_property("options", ParameterSchema::object()
        .with_property("validate", ParameterSchema::boolean().with_default(true))
        .with_property("compress", ParameterSchema::boolean().with_default(false)));

let output_schema = ParameterSchema::object()
    .with_property("processed_data", ParameterSchema::string().required())
    .with_property("metadata", ParameterSchema::object()
        .with_property("size", ParameterSchema::integer())
        .with_property("format", ParameterSchema::string()));

// Register tool
let tool_registry = ToolRegistry::new();
tool_registry.register_tool_with_schema(
    Box::new(DataProcessorTool::new()),
    metadata,
    input_schema,
    output_schema
).await?;
```

### Async Tool Development

Develop tools that support asynchronous operations:

```rust
use rwkv_agent_kit::tools::{AsyncTool, ToolProgress, ProgressCallback};
use tokio::time::{sleep, Duration};

struct LongRunningTool;

#[async_trait]
impl AsyncTool for LongRunningTool {
    type Input = String;
    type Output = String;

    async fn execute_async(
        &self,
        input: Self::Input,
        progress_callback: Option<ProgressCallback>,
    ) -> Result<ToolResult<Self::Output>, ToolError> {
        let total_steps = 10;
        
        for step in 1..=total_steps {
            // Simulate long-running operation
            sleep(Duration::from_secs(1)).await;
            
            // Report progress
            if let Some(ref callback) = progress_callback {
                let progress = ToolProgress::new(step, total_steps)
                    .with_message(format!("Processing step {}/{}", step, total_steps));
                callback(progress).await;
            }
        }
        
        Ok(ToolResult::success(format!("Processing completed: {}", input)))
    }
    
    fn supports_cancellation(&self) -> bool {
        true
    }
    
    async fn cancel(&self) -> Result<(), ToolError> {
        // Implement cancellation logic
        Ok(())
    }
}
```

### Tool Testing

Write comprehensive tests for tools:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use rwkv_agent_kit::tools::testing::{ToolTester, MockContext};

    #[tokio::test]
    async fn test_weather_tool() {
        let tool = WeatherTool::new("test_api_key".to_string());
        let tester = ToolTester::new(tool);
        
        // Test normal case
        let input = WeatherToolInput {
            city: "Beijing".to_string(),
            country: Some("China".to_string()),
        };
        
        let result = tester.test_execution(input).await;
        assert!(result.is_ok());
        
        // Test error case
        let invalid_input = WeatherToolInput {
            city: "".to_string(),
            country: None,
        };
        
        let result = tester.test_execution(invalid_input).await;
        assert!(result.is_err());
    }
    
    #[tokio::test]
    async fn test_tool_permissions() {
        let mut context = MockContext::new();
        context.set_user_id("test_user");
        context.set_permissions(vec!["file_read"]);
        
        let tool = FileReadTool::new();
        let result = tool.execute_with_context(
            FileReadInput {
                path: "/safe/directory/test.txt".to_string(),
            },
            &context
        ).await;
        
        assert!(result.is_ok());
    }
}
```

## Advanced Tool Features

### Tool Caching

Implement tool result caching to improve performance:

```rust
use rwkv_agent_kit::tools::{ToolCache, CacheStrategy, CacheKey};

struct CachedTool<T: Tool> {
    inner: T,
    cache: ToolCache,
}

impl<T: Tool> CachedTool<T> {
    pub fn new(tool: T, cache_strategy: CacheStrategy) -> Self {
        Self {
            inner: tool,
            cache: ToolCache::new(cache_strategy),
        }
    }
}

#[async_trait]
impl<T: Tool + Send + Sync> Tool for CachedTool<T> {
    type Input = T::Input;
    type Output = T::Output;

    fn name(&self) -> &str {
        self.inner.name()
    }

    fn description(&self) -> &str {
        self.inner.description()
    }

    async fn execute(&self, input: Self::Input) -> Result<ToolResult<Self::Output>, ToolError> {
        let cache_key = CacheKey::from_input(&input);
        
        // Check cache
        if let Some(cached_result) = self.cache.get(&cache_key).await? {
            return Ok(cached_result);
        }
        
        // Execute tool
        let result = self.inner.execute(input).await?;
        
        // Cache result
        self.cache.set(cache_key, &result).await?;
        
        Ok(result)
    }
}
```

### Tool Monitoring

Monitor tool usage and performance:

```rust
use rwkv_agent_kit::tools::{ToolMonitor, ToolMetrics, ToolEvent};

struct MonitoredTool<T: Tool> {
    inner: T,
    monitor: ToolMonitor,
}

#[async_trait]
impl<T: Tool + Send + Sync> Tool for MonitoredTool<T> {
    type Input = T::Input;
    type Output = T::Output;

    fn name(&self) -> &str {
        self.inner.name()
    }

    fn description(&self) -> &str {
        self.inner.description()
    }

    async fn execute(&self, input: Self::Input) -> Result<ToolResult<Self::Output>, ToolError> {
        let start_time = std::time::Instant::now();
        
        // Record start event
        self.monitor.record_event(ToolEvent::ExecutionStarted {
            tool_name: self.name().to_string(),
            timestamp: chrono::Utc::now(),
        }).await;
        
        // Execute tool
        let result = self.inner.execute(input).await;
        
        let duration = start_time.elapsed();
        
        // Record completion event
        match &result {
            Ok(_) => {
                self.monitor.record_event(ToolEvent::ExecutionCompleted {
                    tool_name: self.name().to_string(),
                    duration,
                    timestamp: chrono::Utc::now(),
                }).await;
            }
            Err(error) => {
                self.monitor.record_event(ToolEvent::ExecutionFailed {
                    tool_name: self.name().to_string(),
                    error: error.to_string(),
                    duration,
                    timestamp: chrono::Utc::now(),
                }).await;
            }
        }
        
        result
    }
}
```

### Tool Version Management

Manage different versions of tools:

```rust
use rwkv_agent_kit::tools::{ToolVersion, VersionManager, MigrationStrategy};

struct VersionedToolRegistry {
    version_manager: VersionManager,
    tools: HashMap<String, HashMap<ToolVersion, Box<dyn Tool>>>,
}

impl VersionedToolRegistry {
    pub fn new() -> Self {
        Self {
            version_manager: VersionManager::new(),
            tools: HashMap::new(),
        }
    }
    
    pub fn register_tool_version(
        &mut self,
        tool: Box<dyn Tool>,
        version: ToolVersion,
    ) -> Result<(), ToolError> {
        let tool_name = tool.name().to_string();
        
        self.tools
            .entry(tool_name.clone())
            .or_insert_with(HashMap::new)
            .insert(version.clone(), tool);
            
        self.version_manager.register_version(tool_name, version)?;
        
        Ok(())
    }
    
    pub async fn execute_tool(
        &self,
        tool_name: &str,
        version: Option<ToolVersion>,
        input: serde_json::Value,
    ) -> Result<ToolResult<serde_json::Value>, ToolError> {
        let version = match version {
            Some(v) => v,
            None => self.version_manager.get_latest_version(tool_name)?,
        };
        
        let tool = self.tools
            .get(tool_name)
            .and_then(|versions| versions.get(&version))
            .ok_or_else(|| ToolError::ToolNotFound(tool_name.to_string()))?;
            
        // Execute tool (requires type conversion)
        // This needs to be adjusted based on specific implementation
        todo!("Implement generic tool execution")
    }
}
```

## Tool Ecosystem

### Tool Marketplace

Create a tool sharing and discovery platform:

```rust
use rwkv_agent_kit::tools::{ToolMarketplace, ToolPackage, ToolRating};

struct ToolMarketplace {
    packages: HashMap<String, ToolPackage>,
    ratings: HashMap<String, Vec<ToolRating>>,
}

impl ToolMarketplace {
    pub fn new() -> Self {
        Self {
            packages: HashMap::new(),
            ratings: HashMap::new(),
        }
    }
    
    pub async fn publish_tool(
        &mut self,
        package: ToolPackage,
    ) -> Result<(), ToolError> {
        // Validate tool package
        self.validate_package(&package).await?;
        
        // Publish tool
        self.packages.insert(package.name.clone(), package);
        
        Ok(())
    }
    
    pub async fn search_tools(
        &self,
        query: &str,
        category: Option<ToolCategory>,
    ) -> Vec<&ToolPackage> {
        self.packages
            .values()
            .filter(|package| {
                let matches_query = package.name.contains(query) 
                    || package.description.contains(query)
                    || package.tags.iter().any(|tag| tag.contains(query));
                    
                let matches_category = category
                    .map(|cat| package.category == cat)
                    .unwrap_or(true);
                    
                matches_query && matches_category
            })
            .collect()
    }
    
    pub async fn install_tool(
        &self,
        package_name: &str,
        version: Option<&str>,
    ) -> Result<Box<dyn Tool>, ToolError> {
        let package = self.packages
            .get(package_name)
            .ok_or_else(|| ToolError::PackageNotFound(package_name.to_string()))?;
            
        // Download and install tool
        package.install(version).await
    }
}
```

### Tool Composer

Visual tool composition interface:

```rust
use rwkv_agent_kit::tools::{ToolComposer, WorkflowBuilder, VisualNode};

struct VisualToolComposer {
    builder: WorkflowBuilder,
    nodes: Vec<VisualNode>,
    connections: Vec<(usize, usize)>,
}

impl VisualToolComposer {
    pub fn new() -> Self {
        Self {
            builder: WorkflowBuilder::new(),
            nodes: Vec::new(),
            connections: Vec::new(),
        }
    }
    
    pub fn add_tool_node(
        &mut self,
        tool_name: &str,
        position: (f32, f32),
    ) -> usize {
        let node = VisualNode::new(tool_name, position);
        self.nodes.push(node);
        self.nodes.len() - 1
    }
    
    pub fn connect_nodes(
        &mut self,
        from: usize,
        to: usize,
    ) -> Result<(), ToolError> {
        if from >= self.nodes.len() || to >= self.nodes.len() {
            return Err(ToolError::InvalidConnection);
        }
        
        self.connections.push((from, to));
        Ok(())
    }
    
    pub fn build_workflow(&self) -> Result<ToolChain, ToolError> {
        // Build tool chain based on visual connections
        let mut chain = ToolChain::new("visual_workflow");
        
        // Topological sort to determine execution order
        let execution_order = self.topological_sort()?;
        
        for node_index in execution_order {
            let node = &self.nodes[node_index];
            chain.add_step(ToolStep::new(&node.tool_name));
        }
        
        Ok(chain)
    }
    
    fn topological_sort(&self) -> Result<Vec<usize>, ToolError> {
        // Implement topological sort algorithm
        todo!("Implement topological sort")
    }
}
```

## Best Practices Summary

### Tool Development Guidelines

1. **Modular Design**: Break complex functionality into multiple simple tools
2. **Standardized Interface**: Follow unified tool interface specifications
3. **Error Handling**: Provide detailed error information and recovery suggestions
4. **Performance Optimization**: Use caching, batching, and other techniques to improve performance
5. **Security Considerations**: Implement appropriate permission control and input validation

### Tool Usage Recommendations

- **Reasonable Combination**: Choose appropriate tool combinations based on task requirements
- **Monitor Usage**: Regularly check tool usage and performance
- **Version Management**: Maintain tool version consistency and compatibility
- **Documentation Maintenance**: Keep tool documentation and usage examples up to date

---

**Related Links**:
- [Custom Agents](./custom-agents.md) - Learn about agent personalization
- [Memory System](./memory-system.md) - Learn memory system configuration
- [API Documentation](/api/) - View detailed API interface documentation
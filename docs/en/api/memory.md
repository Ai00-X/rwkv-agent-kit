---
title: memory
createTime: 2025/09/08 22:29:35
permalink: /en/article/mj6so256/
---
# Memory API

The Memory API provides functionality for storing, retrieving, and managing agent memories.

## Memory

Represents a stored memory with content and metadata.

### Fields

- `id: String` - Unique memory identifier
- `content: String` - Memory content
- `embedding: Option<Vec<f32>>` - Vector embedding for semantic search
- `metadata: MemoryMetadata` - Associated metadata
- `created_at: DateTime<Utc>` - Creation timestamp
- `updated_at: DateTime<Utc>` - Last update timestamp
- `access_count: u32` - Number of times accessed
- `importance: f32` - Importance score (0.0 to 1.0)

### Methods

#### `new(content: &str) -> Memory`

Creates a new memory with the given content.

**Parameters:**
- `content`: Memory content

**Returns:**
- `Memory`: New memory instance

**Example:**
```rust
let memory = Memory::new("User likes coffee");
```

#### `with_metadata(self, metadata: MemoryMetadata) -> Memory`

Adds metadata to the memory.

**Parameters:**
- `metadata`: Memory metadata

**Returns:**
- `Memory`: Updated memory

#### `with_importance(self, importance: f32) -> Memory`

Sets the importance score.

**Parameters:**
- `importance`: Importance value (0.0 to 1.0)

**Returns:**
- `Memory`: Updated memory

#### `update_content(&mut self, content: &str)`

Updates the memory content.

**Parameters:**
- `content`: New content

#### `increment_access(&mut self)`

Increments the access count.

#### `calculate_relevance(&self, query: &str) -> f32`

Calculates relevance score for a query.

**Parameters:**
- `query`: Search query

**Returns:**
- `f32`: Relevance score (0.0 to 1.0)

## MemoryMetadata

Metadata associated with memories.

### Fields

- `tags: Vec<String>` - Memory tags
- `category: Option<String>` - Memory category
- `source: Option<String>` - Memory source
- `confidence: f32` - Confidence score
- `context: Option<String>` - Additional context
- `relationships: Vec<MemoryRelationship>` - Related memories

### Methods

#### `new() -> MemoryMetadata`

Creates new metadata with default values.

**Returns:**
- `MemoryMetadata`: Default metadata

#### `with_tags(self, tags: Vec<String>) -> MemoryMetadata`

Sets memory tags.

**Parameters:**
- `tags`: List of tags

**Returns:**
- `MemoryMetadata`: Updated metadata

#### `with_category(self, category: &str) -> MemoryMetadata`

Sets the memory category.

**Parameters:**
- `category`: Category name

**Returns:**
- `MemoryMetadata`: Updated metadata

#### `with_source(self, source: &str) -> MemoryMetadata`

Sets the memory source.

**Parameters:**
- `source`: Source identifier

**Returns:**
- `MemoryMetadata`: Updated metadata

#### `with_confidence(self, confidence: f32) -> MemoryMetadata`

Sets the confidence score.

**Parameters:**
- `confidence`: Confidence value (0.0 to 1.0)

**Returns:**
- `MemoryMetadata`: Updated metadata

## MemoryBackend

Trait for implementing custom memory storage backends.

### Required Methods

#### `store(&mut self, memory: Memory) -> Result<String, MemoryError>`

Stores a memory and returns its ID.

**Parameters:**
- `memory`: Memory to store

**Returns:**
- `Result<String, MemoryError>`: Memory ID or error

#### `retrieve(&self, id: &str) -> Result<Option<Memory>, MemoryError>`

Retrieves a memory by ID.

**Parameters:**
- `id`: Memory ID

**Returns:**
- `Result<Option<Memory>, MemoryError>`: Memory or None if not found

#### `search(&self, query: &str, options: SearchOptions) -> Result<Vec<Memory>, MemoryError>`

Searches for memories matching a query.

**Parameters:**
- `query`: Search query
- `options`: Search options

**Returns:**
- `Result<Vec<Memory>, MemoryError>`: Matching memories or error

#### `delete(&mut self, id: &str) -> Result<bool, MemoryError>`

Deletes a memory by ID.

**Parameters:**
- `id`: Memory ID

**Returns:**
- `Result<bool, MemoryError>`: True if deleted, false if not found

#### `list_all(&self) -> Result<Vec<Memory>, MemoryError>`

Lists all stored memories.

**Returns:**
- `Result<Vec<Memory>, MemoryError>`: All memories or error

#### `clear(&mut self) -> Result<(), MemoryError>`

Clears all memories.

**Returns:**
- `Result<(), MemoryError>`: Success or error

### Optional Methods

#### `update(&mut self, id: &str, memory: Memory) -> Result<(), MemoryError>`

Updates an existing memory.

**Parameters:**
- `id`: Memory ID
- `memory`: Updated memory

**Returns:**
- `Result<(), MemoryError>`: Success or error

#### `get_stats(&self) -> Result<MemoryStats, MemoryError>`

Returns memory statistics.

**Returns:**
- `Result<MemoryStats, MemoryError>`: Statistics or error

## VectorMemory

Vector-based memory backend for semantic search.

### Constructor

#### `VectorMemory::new() -> VectorMemoryBuilder`

Creates a new vector memory builder.

**Returns:**
- `VectorMemoryBuilder`: Builder for configuration

### Builder Methods

#### `with_embedding_model(self, model: &str) -> VectorMemoryBuilder`

Sets the embedding model.

**Parameters:**
- `model`: Model name or path

**Returns:**
- `VectorMemoryBuilder`: Updated builder

#### `with_dimension(self, dim: usize) -> VectorMemoryBuilder`

Sets the embedding dimension.

**Parameters:**
- `dim`: Vector dimension

**Returns:**
- `VectorMemoryBuilder`: Updated builder

#### `with_similarity_threshold(self, threshold: f32) -> VectorMemoryBuilder`

Sets the similarity threshold for search.

**Parameters:**
- `threshold`: Threshold value (0.0 to 1.0)

**Returns:**
- `VectorMemoryBuilder`: Updated builder

#### `build(self) -> Result<VectorMemory, MemoryError>`

Builds the vector memory backend.

**Returns:**
- `Result<VectorMemory, MemoryError>`: Vector memory or error

### Methods

#### `add_embedding(&mut self, memory_id: &str, embedding: Vec<f32>) -> Result<(), MemoryError>`

Adds an embedding for a memory.

**Parameters:**
- `memory_id`: Memory ID
- `embedding`: Vector embedding

**Returns:**
- `Result<(), MemoryError>`: Success or error

#### `find_similar(&self, embedding: &[f32], limit: usize) -> Result<Vec<(String, f32)>, MemoryError>`

Finds similar memories by embedding.

**Parameters:**
- `embedding`: Query embedding
- `limit`: Maximum results

**Returns:**
- `Result<Vec<(String, f32)>, MemoryError>`: Memory IDs with similarity scores

## GraphMemory

Graph-based memory backend for relationship tracking.

### Constructor

#### `GraphMemory::new() -> GraphMemoryBuilder`

Creates a new graph memory builder.

**Returns:**
- `GraphMemoryBuilder`: Builder for configuration

### Builder Methods

#### `with_max_nodes(self, max_nodes: usize) -> GraphMemoryBuilder`

Sets the maximum number of nodes.

**Parameters:**
- `max_nodes`: Maximum node count

**Returns:**
- `GraphMemoryBuilder`: Updated builder

#### `with_relationship_types(self, types: Vec<&str>) -> GraphMemoryBuilder`

Sets allowed relationship types.

**Parameters:**
- `types`: List of relationship types

**Returns:**
- `GraphMemoryBuilder`: Updated builder

#### `build(self) -> Result<GraphMemory, MemoryError>`

Builds the graph memory backend.

**Returns:**
- `Result<GraphMemory, MemoryError>`: Graph memory or error

### Methods

#### `add_relationship(&mut self, from_id: &str, to_id: &str, relationship_type: &str) -> Result<(), MemoryError>`

Adds a relationship between memories.

**Parameters:**
- `from_id`: Source memory ID
- `to_id`: Target memory ID
- `relationship_type`: Type of relationship

**Returns:**
- `Result<(), MemoryError>`: Success or error

#### `get_connected(&self, memory_id: &str, options: TraversalOptions) -> Result<Vec<Memory>, MemoryError>`

Gets connected memories through graph traversal.

**Parameters:**
- `memory_id`: Starting memory ID
- `options`: Traversal options

**Returns:**
- `Result<Vec<Memory>, MemoryError>`: Connected memories or error

#### `remove_relationship(&mut self, from_id: &str, to_id: &str) -> Result<(), MemoryError>`

Removes a relationship.

**Parameters:**
- `from_id`: Source memory ID
- `to_id`: Target memory ID

**Returns:**
- `Result<(), MemoryError>`: Success or error

## SearchOptions

Options for memory search operations.

### Fields

- `limit: usize` - Maximum number of results
- `similarity_threshold: f32` - Minimum similarity score
- `include_metadata: bool` - Whether to include metadata
- `sort_by: SortBy` - Sort criteria
- `filters: Vec<MemoryFilter>` - Search filters

### Constructor

#### `SearchOptions::new() -> SearchOptions`

Creates default search options.

**Returns:**
- `SearchOptions`: Default options

### Builder Methods

#### `with_limit(self, limit: usize) -> SearchOptions`

Sets the result limit.

**Parameters:**
- `limit`: Maximum results

**Returns:**
- `SearchOptions`: Updated options

#### `with_similarity_threshold(self, threshold: f32) -> SearchOptions`

Sets the similarity threshold.

**Parameters:**
- `threshold`: Minimum similarity (0.0 to 1.0)

**Returns:**
- `SearchOptions`: Updated options

#### `with_sort_by(self, sort_by: SortBy) -> SearchOptions`

Sets the sort criteria.

**Parameters:**
- `sort_by`: Sort criteria

**Returns:**
- `SearchOptions`: Updated options

#### `with_filter(self, filter: MemoryFilter) -> SearchOptions`

Adds a search filter.

**Parameters:**
- `filter`: Memory filter

**Returns:**
- `SearchOptions`: Updated options

## MemoryStats

Statistics about memory usage.

### Fields

- `total_memories: usize` - Total number of memories
- `memory_size_bytes: usize` - Total memory size in bytes
- `average_access_count: f32` - Average access count
- `most_accessed_memory: Option<String>` - ID of most accessed memory
- `oldest_memory: Option<DateTime<Utc>>` - Timestamp of oldest memory
- `newest_memory: Option<DateTime<Utc>>` - Timestamp of newest memory

### Methods

#### `new() -> MemoryStats`

Creates empty statistics.

**Returns:**
- `MemoryStats`: Empty statistics

#### `calculate_from_memories(memories: &[Memory]) -> MemoryStats`

Calculates statistics from a list of memories.

**Parameters:**
- `memories`: List of memories

**Returns:**
- `MemoryStats`: Calculated statistics

## MemoryError

Error types for memory operations.

### Variants

- `StorageError(String)` - Storage backend errors
- `EmbeddingError(String)` - Embedding generation errors
- `SearchError(String)` - Search operation errors
- `SerializationError(String)` - Serialization errors
- `NotFound(String)` - Memory not found
- `InvalidInput(String)` - Invalid input parameters
- `CapacityExceeded` - Memory capacity exceeded

## Examples

### Basic Memory Usage

```rust
use rwkv_agent_kit::memory::{Memory, MemoryMetadata};

// Create a memory
let metadata = MemoryMetadata::new()
    .with_tags(vec!["preference".to_string()])
    .with_category("user_info");

let memory = Memory::new("User prefers dark mode")
    .with_metadata(metadata)
    .with_importance(0.8);

// Store in agent
let memory_id = agent.save_memory(&memory.content, Some(memory.metadata)).await?;
```

### Vector Memory Backend

```rust
use rwkv_agent_kit::memory::VectorMemory;

// Create vector memory backend
let vector_memory = VectorMemory::new()
    .with_embedding_model("sentence-transformers/all-MiniLM-L6-v2")
    .with_dimension(384)
    .with_similarity_threshold(0.7)
    .build()?;

// Use with agent
let agent = kit.create_agent(
    AgentConfig::new()
        .with_memory_backend(Box::new(vector_memory))
).await?;
```

### Memory Search

```rust
use rwkv_agent_kit::memory::{SearchOptions, SortBy};

// Search memories
let options = SearchOptions::new()
    .with_limit(10)
    .with_similarity_threshold(0.6)
    .with_sort_by(SortBy::Relevance);

let memories = agent.search_memories("user preferences", options).await?;

for memory in memories {
    println!("Found: {} (importance: {})", memory.content, memory.importance);
}
```

### Graph Memory Relationships

```rust
use rwkv_agent_kit::memory::GraphMemory;

// Create graph memory
let graph_memory = GraphMemory::new()
    .with_max_nodes(1000)
    .with_relationship_types(vec!["related_to", "caused_by", "leads_to"])
    .build()?;

// Add relationships
graph_memory.add_relationship("memory1", "memory2", "related_to").await?;

// Find connected memories
let connected = graph_memory.get_connected(
    "memory1",
    TraversalOptions::new().with_max_depth(2)
).await?;
```
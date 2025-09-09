---
title: database
createTime: 2025/09/08 13:18:31
permalink: /article/z6dpx030/
---
# 数据库 API

## 概述

数据库模块提供了向量图数据库的实现，结合了传统关系型数据库的结构化存储和向量数据库的语义搜索能力。支持复杂的图结构查询和高效的向量相似度搜索。

## VectorGraphDB

向量图数据库是系统的核心存储组件。

```rust
use rwkv_agent_kit::prelude::*;

// 创建数据库实例
let config = Config::default();
let db = VectorGraphDB::new(config).await?;
```

### 构造方法

#### `new(config: Config) -> Result<Self>`

创建新的向量图数据库实例。

**参数:**
- `config`: 数据库配置

**返回值:**
- `Result<VectorGraphDB>`: 数据库实例

**示例:**
```rust
let mut config = Config::default();
config.database.url = "sqlite:./data/rwkv_agent.db".to_string();
config.database.max_connections = 10;
config.database.connection_timeout = Duration::from_secs(30);

let db = VectorGraphDB::new(config).await?;
```

### 连接管理

#### `connect(&self) -> Result<()>`

建立数据库连接。

#### `disconnect(&self) -> Result<()>`

断开数据库连接。

#### `is_connected(&self) -> bool`

检查连接状态。

#### `get_connection_info(&self) -> ConnectionInfo`

获取连接信息。

### 记忆操作

#### `insert_memory(&self, memory: &Memory) -> Result<Uuid>`

插入新记忆。

**参数:**
- `memory`: 记忆对象

**返回值:**
- `Result<Uuid>`: 记忆ID

**示例:**
```rust
let memory = Memory::new(
    "用户询问了关于Rust的问题".to_string(),
    MemoryType::Conversation,
    embedding,
    MemoryAttributes::default()
);

let memory_id = db.insert_memory(&memory).await?;
```

#### `get_memory(&self, id: &Uuid) -> Result<Option<Memory>>`

根据ID获取记忆。

**参数:**
- `id`: 记忆ID

**返回值:**
- `Result<Option<Memory>>`: 记忆对象（如果存在）

#### `update_memory(&self, id: &Uuid, memory: &Memory) -> Result<()>`

更新记忆。

**参数:**
- `id`: 记忆ID
- `memory`: 更新后的记忆对象

#### `delete_memory(&self, id: &Uuid) -> Result<()>`

删除记忆。

**参数:**
- `id`: 记忆ID

#### `list_memories(&self, filters: Option<MemoryFilters>, pagination: Option<Pagination>) -> Result<Vec<Memory>>`

列出记忆。

**参数:**
- `filters`: 过滤条件（可选）
- `pagination`: 分页参数（可选）

**返回值:**
- `Result<Vec<Memory>>`: 记忆列表

**示例:**
```rust
let filters = MemoryFilters {
    memory_types: Some(vec![MemoryType::Knowledge, MemoryType::UserProfile]),
    importance_range: Some((0.7, 1.0)),
    date_range: Some((start_date, end_date)),
    tags: Some(vec!["重要".to_string()]),
    ..Default::default()
};

let pagination = Pagination {
    offset: 0,
    limit: 50,
};

let memories = db.list_memories(Some(filters), Some(pagination)).await?;
```

### 向量搜索

#### `vector_search(&self, query_vector: &[f32], limit: usize, threshold: Option<f32>) -> Result<Vec<VectorSearchResult>>`

执行向量相似度搜索。

**参数:**
- `query_vector`: 查询向量
- `limit`: 结果数量限制
- `threshold`: 相似度阈值（可选）

**返回值:**
- `Result<Vec<VectorSearchResult>>`: 搜索结果

**示例:**
```rust
let query_text = "机器学习算法";
let query_vector = embedding_model.encode(query_text).await?;

let results = db.vector_search(&query_vector, 10, Some(0.7)).await?;

for result in results {
    println!("记忆ID: {}, 相似度: {:.3}", result.memory_id, result.similarity);
}
```

#### `hybrid_search(&self, query: &HybridQuery) -> Result<Vec<HybridSearchResult>>`

执行混合搜索（向量 + 关键词）。

**参数:**
- `query`: 混合查询对象

**返回值:**
- `Result<Vec<HybridSearchResult>>`: 搜索结果

**示例:**
```rust
let hybrid_query = HybridQuery {
    vector: query_vector,
    keywords: vec!["机器学习".to_string(), "算法".to_string()],
    vector_weight: 0.7,
    keyword_weight: 0.3,
    limit: 10,
    filters: Some(MemoryFilters {
        memory_types: Some(vec![MemoryType::Knowledge]),
        ..Default::default()
    }),
};

let results = db.hybrid_search(&hybrid_query).await?;
```

### 图操作

#### `insert_connection(&self, connection: &MemoryConnection) -> Result<()>`

插入记忆连接。

**参数:**
- `connection`: 连接对象

**示例:**
```rust
let connection = MemoryConnection {
    from_memory_id: memory_id_1,
    to_memory_id: memory_id_2,
    connection_type: ConnectionType::Semantic,
    strength: 0.8,
    metadata: ConnectionMetadata::default(),
};

db.insert_connection(&connection).await?;
```

#### `get_connections(&self, memory_id: &Uuid, direction: Option<ConnectionDirection>) -> Result<Vec<MemoryConnection>>`

获取记忆的连接。

**参数:**
- `memory_id`: 记忆ID
- `direction`: 连接方向（可选）

**返回值:**
- `Result<Vec<MemoryConnection>>`: 连接列表

#### `update_connection(&self, from_id: &Uuid, to_id: &Uuid, updates: &ConnectionUpdates) -> Result<()>`

更新连接。

**参数:**
- `from_id`: 源记忆ID
- `to_id`: 目标记忆ID
- `updates`: 更新内容

#### `delete_connection(&self, from_id: &Uuid, to_id: &Uuid) -> Result<()>`

删除连接。

**参数:**
- `from_id`: 源记忆ID
- `to_id`: 目标记忆ID

#### `graph_traversal(&self, start_id: &Uuid, traversal_config: &TraversalConfig) -> Result<Vec<GraphPath>>`

执行图遍历。

**参数:**
- `start_id`: 起始记忆ID
- `traversal_config`: 遍历配置

**返回值:**
- `Result<Vec<GraphPath>>`: 遍历路径

**示例:**
```rust
let traversal_config = TraversalConfig {
    max_depth: 3,
    connection_types: Some(vec![ConnectionType::Semantic, ConnectionType::Temporal]),
    min_strength: Some(0.5),
    max_results: Some(20),
    ..Default::default()
};

let paths = db.graph_traversal(&start_memory_id, &traversal_config).await?;

for path in paths {
    println!("路径长度: {}, 总强度: {:.3}", path.length, path.total_strength);
    for node in path.nodes {
        println!("  记忆: {}", node.memory_id);
    }
}
```

### 批量操作

#### `batch_insert_memories(&self, memories: &[Memory]) -> Result<Vec<Uuid>>`

批量插入记忆。

**参数:**
- `memories`: 记忆列表

**返回值:**
- `Result<Vec<Uuid>>`: 记忆ID列表

#### `batch_update_memories(&self, updates: &[(Uuid, Memory)]) -> Result<()>`

批量更新记忆。

**参数:**
- `updates`: 更新列表（ID和记忆对）

#### `batch_delete_memories(&self, ids: &[Uuid]) -> Result<()>`

批量删除记忆。

**参数:**
- `ids`: 记忆ID列表

#### `batch_insert_connections(&self, connections: &[MemoryConnection]) -> Result<()>`

批量插入连接。

**参数:**
- `connections`: 连接列表

### 统计和分析

#### `get_memory_count(&self, filters: Option<MemoryFilters>) -> Result<u64>`

获取记忆数量。

**参数:**
- `filters`: 过滤条件（可选）

**返回值:**
- `Result<u64>`: 记忆数量

#### `get_connection_count(&self, memory_id: Option<Uuid>) -> Result<u64>`

获取连接数量。

**参数:**
- `memory_id`: 特定记忆ID（可选）

**返回值:**
- `Result<u64>`: 连接数量

#### `get_database_stats(&self) -> Result<DatabaseStats>`

获取数据库统计信息。

**返回值:**
- `Result<DatabaseStats>`: 统计信息

**示例:**
```rust
let stats = db.get_database_stats().await?;

println!("总记忆数: {}", stats.total_memories);
println!("总连接数: {}", stats.total_connections);
println!("平均连接度: {:.2}", stats.average_connectivity);
println!("数据库大小: {} MB", stats.database_size_mb);
```

### 维护操作

#### `optimize_database(&self) -> Result<()>`

优化数据库性能。

#### `vacuum_database(&self) -> Result<()>`

清理数据库空间。

#### `rebuild_indexes(&self) -> Result<()>`

重建索引。

#### `backup_database(&self, backup_path: &str) -> Result<()>`

备份数据库。

**参数:**
- `backup_path`: 备份文件路径

#### `restore_database(&self, backup_path: &str) -> Result<()>`

恢复数据库。

**参数:**
- `backup_path`: 备份文件路径

## 数据类型

### VectorSearchResult

向量搜索结果。

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorSearchResult {
    /// 记忆ID
    pub memory_id: Uuid,
    /// 相似度分数
    pub similarity: f32,
    /// 记忆对象（可选）
    pub memory: Option<Memory>,
}
```

### HybridQuery

混合查询对象。

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HybridQuery {
    /// 查询向量
    pub vector: Vec<f32>,
    /// 关键词列表
    pub keywords: Vec<String>,
    /// 向量权重
    pub vector_weight: f32,
    /// 关键词权重
    pub keyword_weight: f32,
    /// 结果限制
    pub limit: usize,
    /// 过滤条件
    pub filters: Option<MemoryFilters>,
}
```

### HybridSearchResult

混合搜索结果。

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HybridSearchResult {
    /// 记忆ID
    pub memory_id: Uuid,
    /// 向量相似度
    pub vector_similarity: f32,
    /// 关键词匹配分数
    pub keyword_score: f32,
    /// 综合分数
    pub combined_score: f32,
    /// 记忆对象（可选）
    pub memory: Option<Memory>,
}
```

### TraversalConfig

图遍历配置。

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraversalConfig {
    /// 最大深度
    pub max_depth: usize,
    /// 连接类型过滤
    pub connection_types: Option<Vec<ConnectionType>>,
    /// 最小连接强度
    pub min_strength: Option<f32>,
    /// 最大结果数
    pub max_results: Option<usize>,
    /// 遍历方向
    pub direction: TraversalDirection,
    /// 是否包含起始节点
    pub include_start: bool,
}
```

### GraphPath

图遍历路径。

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphPath {
    /// 路径节点
    pub nodes: Vec<GraphNode>,
    /// 路径长度
    pub length: usize,
    /// 总连接强度
    pub total_strength: f32,
    /// 平均连接强度
    pub average_strength: f32,
}
```

### DatabaseStats

数据库统计信息。

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseStats {
    /// 总记忆数
    pub total_memories: u64,
    /// 总连接数
    pub total_connections: u64,
    /// 平均连接度
    pub average_connectivity: f32,
    /// 数据库大小（MB）
    pub database_size_mb: f64,
    /// 索引大小（MB）
    pub index_size_mb: f64,
    /// 最后优化时间
    pub last_optimized: Option<DateTime<Utc>>,
    /// 记忆类型分布
    pub memory_type_distribution: HashMap<MemoryType, u64>,
    /// 连接类型分布
    pub connection_type_distribution: HashMap<ConnectionType, u64>,
}
```

## 完整示例

```rust
use rwkv_agent_kit::prelude::*;
use std::collections::HashMap;
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    // 创建数据库配置
    let mut config = Config::default();
    config.database.url = "sqlite:./example.db".to_string();
    config.database.max_connections = 10;
    
    // 创建数据库实例
    let db = VectorGraphDB::new(config).await?;
    
    // 创建一些示例记忆
    let memories = vec![
        Memory::new(
            "用户是一名Rust开发者".to_string(),
            MemoryType::UserProfile,
            vec![0.1, 0.2, 0.3], // 示例嵌入
            MemoryAttributes {
                keywords: vec!["Rust".to_string(), "开发者".to_string()],
                importance: 0.9,
                ..Default::default()
            }
        ),
        Memory::new(
            "用户询问了异步编程问题".to_string(),
            MemoryType::Conversation,
            vec![0.2, 0.3, 0.4], // 示例嵌入
            MemoryAttributes {
                keywords: vec!["异步编程".to_string(), "问题".to_string()],
                importance: 0.7,
                ..Default::default()
            }
        ),
    ];
    
    // 批量插入记忆
    let memory_ids = db.batch_insert_memories(&memories).await?;
    println!("插入了 {} 条记忆", memory_ids.len());
    
    // 创建记忆连接
    let connection = MemoryConnection {
        from_memory_id: memory_ids[0],
        to_memory_id: memory_ids[1],
        connection_type: ConnectionType::Semantic,
        strength: 0.8,
        metadata: ConnectionMetadata::default(),
    };
    
    db.insert_connection(&connection).await?;
    
    // 执行向量搜索
    let query_vector = vec![0.15, 0.25, 0.35];
    let search_results = db.vector_search(&query_vector, 5, Some(0.5)).await?;
    
    println!("向量搜索结果:");
    for result in search_results {
        println!("  记忆ID: {}, 相似度: {:.3}", result.memory_id, result.similarity);
    }
    
    // 执行混合搜索
    let hybrid_query = HybridQuery {
        vector: query_vector,
        keywords: vec!["Rust".to_string()],
        vector_weight: 0.7,
        keyword_weight: 0.3,
        limit: 5,
        filters: None,
    };
    
    let hybrid_results = db.hybrid_search(&hybrid_query).await?;
    
    println!("混合搜索结果:");
    for result in hybrid_results {
        println!("  记忆ID: {}, 综合分数: {:.3}", result.memory_id, result.combined_score);
    }
    
    // 执行图遍历
    let traversal_config = TraversalConfig {
        max_depth: 2,
        connection_types: Some(vec![ConnectionType::Semantic]),
        min_strength: Some(0.5),
        max_results: Some(10),
        direction: TraversalDirection::Both,
        include_start: true,
    };
    
    let paths = db.graph_traversal(&memory_ids[0], &traversal_config).await?;
    
    println!("图遍历结果:");
    for path in paths {
        println!("  路径长度: {}, 平均强度: {:.3}", path.length, path.average_strength);
    }
    
    // 获取数据库统计
    let stats = db.get_database_stats().await?;
    println!("数据库统计:");
    println!("  总记忆数: {}", stats.total_memories);
    println!("  总连接数: {}", stats.total_connections);
    println!("  平均连接度: {:.2}", stats.average_connectivity);
    
    // 优化数据库
    db.optimize_database().await?;
    println!("数据库优化完成");
    
    Ok(())
}
```

## 下一步

- [记忆管理API](./memory.md)
- [工具系统API](./tools.md)
- [核心类型定义](./types.md)
- [配置选项](../config/README.md)
---
title: memory
createTime: 2025/09/08 13:19:20
permalink: /article/0tbz664k/
---
# 记忆管理 API

## 概述

记忆管理模块是RWKV Agent Kit的核心组件，提供了智能的记忆存储、检索和管理功能。基于A-Mem和HippoRAG论文的研究成果，实现了动态记忆组织和神经生物学启发的检索机制。

## MemoryManager

记忆管理器是记忆系统的主要接口。

```rust
use rwkv_agent_kit::prelude::*;

// 创建记忆管理器
let config = Config::default();
let db = Arc::new(VectorGraphDB::new(config.clone()).await?);
let memory_manager = MemoryManager::new(db, config).await?;
```

### 构造方法

#### `new(db: Arc<VectorGraphDB>, config: Config) -> Result<Self>`

创建新的记忆管理器实例。

**参数:**
- `db`: 向量图数据库实例
- `config`: 系统配置

**返回值:**
- `Result<MemoryManager>`: 记忆管理器实例

### 记忆操作

#### `create_memory(&self, memory: &Memory) -> Result<Uuid>`

创建新的记忆。

**参数:**
- `memory`: 记忆对象

**返回值:**
- `Result<Uuid>`: 记忆的唯一标识符

**示例:**
```rust
let attributes = MemoryAttributes {
    keywords: vec!["机器学习".to_string()],
    tags: vec!["技术".to_string()],
    importance: 0.8,
    ..Default::default()
};

let content = "用户询问了关于机器学习的基础概念";
let embedding = memory_manager.generate_embedding(content).await?;
let memory = Memory::new(
    content.to_string(),
    MemoryType::Knowledge,
    embedding,
    attributes
);

let memory_id = memory_manager.create_memory(&memory).await?;
```

#### `get_memory(&self, id: &Uuid) -> Result<Option<Memory>>`

根据ID获取记忆。

**参数:**
- `id`: 记忆的唯一标识符

**返回值:**
- `Result<Option<Memory>>`: 记忆对象（如果存在）

#### `update_memory(&self, id: &Uuid, memory: &Memory) -> Result<()>`

更新现有记忆。

**参数:**
- `id`: 记忆的唯一标识符
- `memory`: 更新后的记忆对象

#### `delete_memory(&self, id: &Uuid) -> Result<()>`

删除记忆。

**参数:**
- `id`: 记忆的唯一标识符

### 记忆检索

#### `retrieve_memories(&self, query: &Query, context: &Context) -> Result<Vec<Memory>>`

检索相关记忆。

**参数:**
- `query`: 查询对象
- `context`: 上下文信息

**返回值:**
- `Result<Vec<Memory>>`: 相关记忆列表

**示例:**
```rust
let query = Query {
    text: "机器学习算法".to_string(),
    query_type: QueryType::Semantic,
    filters: QueryFilters {
        memory_types: Some(vec![MemoryType::Knowledge]),
        tags: Some(vec!["技术".to_string()]),
        importance_range: Some((0.5, 1.0)),
        ..Default::default()
    },
    limit: Some(10),
    weights: QueryWeights {
        semantic_weight: 0.7,
        keyword_weight: 0.2,
        recency_weight: 0.1,
        ..Default::default()
    },
    ..Default::default()
};

let context = Context::default();
let memories = memory_manager.retrieve_memories(&query, &context).await?;
```

#### `search_by_keywords(&self, keywords: &[String]) -> Result<Vec<Memory>>`

基于关键词搜索记忆。

**参数:**
- `keywords`: 关键词列表

**返回值:**
- `Result<Vec<Memory>>`: 匹配的记忆列表

#### `search_by_semantic(&self, text: &str, threshold: f32) -> Result<Vec<Memory>>`

基于语义相似度搜索记忆。

**参数:**
- `text`: 查询文本
- `threshold`: 相似度阈值

**返回值:**
- `Result<Vec<Memory>>`: 语义相似的记忆列表

### 记忆连接

#### `create_connection(&self, from_id: &Uuid, to_id: &Uuid, connection_type: ConnectionType, strength: f32) -> Result<()>`

在两个记忆之间创建连接。

**参数:**
- `from_id`: 源记忆ID
- `to_id`: 目标记忆ID
- `connection_type`: 连接类型
- `strength`: 连接强度

**示例:**
```rust
memory_manager.create_connection(
    &memory_id_1,
    &memory_id_2,
    ConnectionType::Semantic,
    0.8
).await?;
```

#### `get_connections(&self, memory_id: &Uuid) -> Result<Vec<MemoryConnection>>`

获取记忆的所有连接。

**参数:**
- `memory_id`: 记忆ID

**返回值:**
- `Result<Vec<MemoryConnection>>`: 连接列表

#### `update_connection_strength(&self, from_id: &Uuid, to_id: &Uuid, new_strength: f32) -> Result<()>`

更新连接强度。

### 记忆演化

#### `evolve_memories(&self) -> Result<()>`

执行记忆演化过程，包括重要性调整和连接更新。

```rust
// 定期执行记忆演化
memory_manager.evolve_memories().await?;
```

#### `decay_memories(&self, decay_rate: f32) -> Result<()>`

应用记忆衰减。

**参数:**
- `decay_rate`: 衰减率

#### `consolidate_memories(&self) -> Result<()>`

合并相似的记忆。

### 嵌入生成

#### `generate_embedding(&self, text: &str) -> Result<Vec<f32>>`

为文本生成嵌入向量。

**参数:**
- `text`: 输入文本

**返回值:**
- `Result<Vec<f32>>`: 嵌入向量

## Memory

记忆对象表示单个记忆项。

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Memory {
    pub id: Uuid,
    pub content: String,
    pub memory_type: MemoryType,
    pub embedding: Vec<f32>,
    pub attributes: MemoryAttributes,
    pub metadata: MemoryMetadata,
    pub connections: MemoryConnections,
}
```

### 构造方法

#### `new(content: String, memory_type: MemoryType, embedding: Vec<f32>, attributes: MemoryAttributes) -> Self`

创建新的记忆对象。

**示例:**
```rust
let memory = Memory::new(
    "用户喜欢喝咖啡".to_string(),
    MemoryType::UserPreference,
    embedding,
    MemoryAttributes {
        keywords: vec!["咖啡".to_string(), "偏好".to_string()],
        importance: 0.6,
        ..Default::default()
    }
);
```

### 方法

#### `update_importance(&mut self, new_importance: f32)`

更新记忆重要性。

#### `add_keyword(&mut self, keyword: String)`

添加关键词。

#### `add_tag(&mut self, tag: String)`

添加标签。

#### `is_similar_to(&self, other: &Memory, threshold: f32) -> bool`

检查与另一个记忆的相似度。

## MemoryType

记忆类型枚举。

```rust
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MemoryType {
    /// 用户个人信息
    UserProfile,
    /// 用户偏好
    UserPreference,
    /// 对话历史
    Conversation,
    /// 知识信息
    Knowledge,
    /// 任务相关
    Task,
    /// 情感记忆
    Emotional,
    /// 程序性记忆
    Procedural,
    /// 情景记忆
    Episodic,
    /// 语义记忆
    Semantic,
}
```

## MemoryAttributes

记忆属性结构。

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryAttributes {
    /// 关键词列表
    pub keywords: Vec<String>,
    /// 标签列表
    pub tags: Vec<String>,
    /// 重要性分数 (0.0-1.0)
    pub importance: f32,
    /// 情感倾向 (-1.0到1.0)
    pub emotional_valence: f32,
    /// 置信度 (0.0-1.0)
    pub confidence: f32,
    /// 访问频率
    pub access_frequency: u32,
    /// 最后访问时间
    pub last_accessed: DateTime<Utc>,
    /// 自定义属性
    pub custom_attributes: HashMap<String, Value>,
}
```

## Query

查询对象用于记忆检索。

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Query {
    /// 查询文本
    pub text: String,
    /// 查询类型
    pub query_type: QueryType,
    /// 过滤条件
    pub filters: QueryFilters,
    /// 结果限制
    pub limit: Option<usize>,
    /// 偏移量
    pub offset: Option<usize>,
    /// 排序方式
    pub sort_by: Option<SortBy>,
    /// 权重配置
    pub weights: QueryWeights,
}
```

### QueryType

```rust
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum QueryType {
    /// 语义搜索
    Semantic,
    /// 关键词搜索
    Keyword,
    /// 混合搜索
    Hybrid,
    /// 图遍历搜索
    Graph,
}
```

## 完整示例

```rust
use rwkv_agent_kit::prelude::*;
use std::sync::Arc;
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    // 创建配置和数据库
    let mut config = Config::default();
    config.database.url = "sqlite:memory_test.db".to_string();
    
    let db = Arc::new(VectorGraphDB::new(config.clone()).await?);
    let memory_manager = MemoryManager::new(db, config).await?;
    
    // 创建记忆
    let content = "用户是一名Rust开发者，专注于系统编程";
    let embedding = memory_manager.generate_embedding(content).await?;
    
    let attributes = MemoryAttributes {
        keywords: vec!["Rust".to_string(), "开发者".to_string(), "系统编程".to_string()],
        tags: vec!["用户信息".to_string(), "技能".to_string()],
        importance: 0.9,
        emotional_valence: 0.2,
        confidence: 0.95,
        ..Default::default()
    };
    
    let memory = Memory::new(
        content.to_string(),
        MemoryType::UserProfile,
        embedding,
        attributes
    );
    
    let memory_id = memory_manager.create_memory(&memory).await?;
    println!("创建记忆: {}", memory_id);
    
    // 创建相关记忆
    let related_content = "用户最近在学习异步编程";
    let related_embedding = memory_manager.generate_embedding(related_content).await?;
    
    let related_memory = Memory::new(
        related_content.to_string(),
        MemoryType::Knowledge,
        related_embedding,
        MemoryAttributes {
            keywords: vec!["异步编程".to_string(), "学习".to_string()],
            importance: 0.7,
            ..Default::default()
        }
    );
    
    let related_id = memory_manager.create_memory(&related_memory).await?;
    
    // 创建记忆连接
    memory_manager.create_connection(
        &memory_id,
        &related_id,
        ConnectionType::Semantic,
        0.8
    ).await?;
    
    // 检索记忆
    let query = Query {
        text: "Rust编程".to_string(),
        query_type: QueryType::Semantic,
        filters: QueryFilters {
            memory_types: Some(vec![MemoryType::UserProfile, MemoryType::Knowledge]),
            importance_range: Some((0.5, 1.0)),
            ..Default::default()
        },
        limit: Some(5),
        weights: QueryWeights {
            semantic_weight: 0.8,
            keyword_weight: 0.2,
            ..Default::default()
        },
        ..Default::default()
    };
    
    let context = Context::default();
    let results = memory_manager.retrieve_memories(&query, &context).await?;
    
    println!("找到 {} 条相关记忆:", results.len());
    for memory in results {
        println!("- {}", memory.content);
        println!("  重要性: {:.2}", memory.attributes.importance);
        println!("  关键词: {:?}", memory.attributes.keywords);
    }
    
    // 执行记忆演化
    memory_manager.evolve_memories().await?;
    
    Ok(())
}
```

## 下一步

- [数据库API](./database.md)
- [工具系统API](./tools.md)
- [核心类型定义](./types.md)
- [配置选项](../config/README.md)
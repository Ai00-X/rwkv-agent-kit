//! 查询优化和索引管理模块
//!
//! 提供SQL查询优化、索引管理、查询分析等功能

use serde::{Deserialize, Serialize};
use sqlx::{Row, SqlitePool};
use std::collections::HashMap;

use crate::core::error::{ErrorCode, RwkvError, RwkvResult};

/// 索引类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum IndexType {
    /// B-Tree索引（默认）
    BTree,
    /// 唯一索引
    Unique,
    /// 部分索引
    Partial,
    /// 表达式索引
    Expression,
    /// 复合索引
    Composite,
}

/// 索引定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexDefinition {
    /// 索引名称
    pub name: String,
    /// 表名
    pub table: String,
    /// 列名列表
    pub columns: Vec<String>,
    /// 索引类型
    pub index_type: IndexType,
    /// 是否是唯一索引
    pub unique: bool,
    /// WHERE条件（用于部分索引）
    pub where_clause: Option<String>,
    /// 表达式（用于表达式索引）
    pub expression: Option<String>,
    /// 是否自动创建
    pub auto_create: bool,
}

/// 查询分析结果
#[derive(Debug, Clone, Serialize)]
pub struct QueryAnalysis {
    /// SQL语句
    pub sql: String,
    /// 查询计划
    pub query_plan: String,
    /// 扫描的行数
    pub rows_scanned: Option<u64>,
    /// 返回的行数
    pub rows_returned: Option<u64>,
    /// 使用的索引
    pub used_indexes: Vec<String>,
    /// 建议的优化
    pub optimization_suggestions: Vec<String>,
    /// 复杂度评分（1-10）
    pub complexity_score: u32,
}

/// 慢查询信息
#[derive(Debug, Clone, Serialize)]
pub struct SlowQuery {
    /// SQL语句
    pub sql: String,
    /// 执行时间（毫秒）
    pub duration_ms: u64,
    /// 执行次数
    pub execution_count: u64,
    /// 平均执行时间
    pub avg_duration_ms: f64,
    /// 最后执行时间
    pub last_executed: chrono::DateTime<chrono::Utc>,
    /// 优化建议
    pub suggestions: Vec<String>,
}

/// 查询优化器
pub struct QueryOptimizer {
    pool: SqlitePool,
    /// 预定义索引
    predefined_indexes: Vec<IndexDefinition>,
    /// 查询统计
    #[allow(dead_code)] // 预留功能，后续使用
    query_stats: HashMap<String, SlowQuery>,
}

impl QueryOptimizer {
    /// 创建新的查询优化器
    pub fn new(pool: SqlitePool) -> Self {
        Self {
            pool,
            predefined_indexes: Self::get_predefined_indexes(),
            query_stats: HashMap::new(),
        }
    }

    /// 获取预定义索引
    fn get_predefined_indexes() -> Vec<IndexDefinition> {
        vec![
            // memory_events表索引
            IndexDefinition {
                name: "idx_memory_events_session_agent".to_string(),
                table: "memory_events".to_string(),
                columns: vec!["session_id".to_string(), "agent_name".to_string()],
                index_type: IndexType::Composite,
                unique: false,
                where_clause: None,
                expression: None,
                auto_create: true,
            },
            IndexDefinition {
                name: "idx_memory_events_timestamp".to_string(),
                table: "memory_events".to_string(),
                columns: vec!["created_at".to_string()],
                index_type: IndexType::BTree,
                unique: false,
                where_clause: None,
                expression: None,
                auto_create: true,
            },
            IndexDefinition {
                name: "idx_memory_events_importance".to_string(),
                table: "memory_events".to_string(),
                columns: vec!["importance".to_string()],
                index_type: IndexType::Partial,
                unique: false,
                where_clause: Some("importance IS NOT NULL".to_string()),
                expression: None,
                auto_create: true,
            },
            IndexDefinition {
                name: "idx_memory_events_role_agent".to_string(),
                table: "memory_events".to_string(),
                columns: vec!["role".to_string(), "agent_name".to_string()],
                index_type: IndexType::Composite,
                unique: false,
                where_clause: None,
                expression: None,
                auto_create: true,
            },
            // semantic_chunks表索引
            IndexDefinition {
                name: "idx_semantic_chunks_weight".to_string(),
                table: "semantic_chunks".to_string(),
                columns: vec!["weight".to_string()],
                index_type: IndexType::BTree,
                unique: false,
                where_clause: None,
                expression: None,
                auto_create: true,
            },
            IndexDefinition {
                name: "idx_semantic_chunks_last_ref".to_string(),
                table: "semantic_chunks".to_string(),
                columns: vec!["last_ref_ts".to_string()],
                index_type: IndexType::BTree,
                unique: false,
                where_clause: None,
                expression: None,
                auto_create: true,
            },
            // graph_nodes表索引
            IndexDefinition {
                name: "idx_graph_nodes_entity".to_string(),
                table: "graph_nodes".to_string(),
                columns: vec!["entity_type".to_string(), "entity_name".to_string()],
                index_type: IndexType::Composite,
                unique: true,
                where_clause: None,
                expression: None,
                auto_create: true,
            },
            IndexDefinition {
                name: "idx_graph_nodes_type".to_string(),
                table: "graph_nodes".to_string(),
                columns: vec!["entity_type".to_string()],
                index_type: IndexType::BTree,
                unique: false,
                where_clause: None,
                expression: None,
                auto_create: true,
            },
            // graph_edges表索引
            IndexDefinition {
                name: "idx_graph_edges_from_to".to_string(),
                table: "graph_edges".to_string(),
                columns: vec!["from_node".to_string(), "to_node".to_string()],
                index_type: IndexType::Composite,
                unique: false,
                where_clause: None,
                expression: None,
                auto_create: true,
            },
            IndexDefinition {
                name: "idx_graph_edges_weight".to_string(),
                table: "graph_edges".to_string(),
                columns: vec!["weight".to_string()],
                index_type: IndexType::BTree,
                unique: false,
                where_clause: None,
                expression: None,
                auto_create: true,
            },
            IndexDefinition {
                name: "idx_graph_edges_relation".to_string(),
                table: "graph_edges".to_string(),
                columns: vec!["relation_type".to_string()],
                index_type: IndexType::BTree,
                unique: false,
                where_clause: None,
                expression: None,
                auto_create: true,
            },
            // persona_traits表索引
            IndexDefinition {
                name: "idx_persona_traits_agent_type".to_string(),
                table: "persona_traits".to_string(),
                columns: vec!["agent_name".to_string(), "trait_type".to_string()],
                index_type: IndexType::Composite,
                unique: false,
                where_clause: None,
                expression: None,
                auto_create: true,
            },
            IndexDefinition {
                name: "idx_persona_traits_confidence".to_string(),
                table: "persona_traits".to_string(),
                columns: vec!["confidence".to_string()],
                index_type: IndexType::BTree,
                unique: false,
                where_clause: None,
                expression: None,
                auto_create: true,
            },
            IndexDefinition {
                name: "idx_persona_traits_last_seen".to_string(),
                table: "persona_traits".to_string(),
                columns: vec!["last_seen".to_string()],
                index_type: IndexType::BTree,
                unique: false,
                where_clause: None,
                expression: None,
                auto_create: true,
            },
            // sessions表索引
            IndexDefinition {
                name: "idx_sessions_agent_active".to_string(),
                table: "sessions".to_string(),
                columns: vec!["agent_name".to_string(), "is_active".to_string()],
                index_type: IndexType::Composite,
                unique: false,
                where_clause: None,
                expression: None,
                auto_create: true,
            },
            IndexDefinition {
                name: "idx_sessions_created_at".to_string(),
                table: "sessions".to_string(),
                columns: vec!["created_at".to_string()],
                index_type: IndexType::BTree,
                unique: false,
                where_clause: None,
                expression: None,
                auto_create: true,
            },
        ]
    }

    /// 初始化所有索引
    pub async fn initialize_indexes(&self) -> RwkvResult<()> {
        log::info!("开始初始化数据库索引...");

        for index_def in &self.predefined_indexes {
            if index_def.auto_create {
                if let Err(e) = self.create_index(index_def).await {
                    log::warn!("创建索引 {} 失败: {}", index_def.name, e);
                    // 继续创建其他索引，不中断流程
                }
            }
        }

        log::info!("数据库索引初始化完成");
        Ok(())
    }

    /// 创建索引
    pub async fn create_index(&self, index_def: &IndexDefinition) -> RwkvResult<()> {
        // 检查索引是否已存在
        if self.index_exists(&index_def.name).await? {
            log::debug!("索引 {} 已存在，跳过创建", index_def.name);
            return Ok(());
        }

        let sql = self.generate_create_index_sql(index_def)?;

        sqlx::query(&sql).execute(&self.pool).await.map_err(|e| {
            RwkvError::new(
                ErrorCode::DatabaseQueryFailed,
                format!("创建索引失败: {}", e),
            )
            .with_context(format!("索引: {}, SQL: {}", index_def.name, sql))
        })?;

        log::info!("索引 {} 创建成功", index_def.name);
        Ok(())
    }

    /// 检查索引是否存在
    async fn index_exists(&self, index_name: &str) -> RwkvResult<bool> {
        let result = sqlx::query("SELECT name FROM sqlite_master WHERE type='index' AND name=?")
            .bind(index_name)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| {
                RwkvError::new(
                    ErrorCode::DatabaseQueryFailed,
                    format!("检查索引存在性失败: {}", e),
                )
            })?;

        Ok(result.is_some())
    }

    /// 生成创建索引的SQL
    fn generate_create_index_sql(&self, index_def: &IndexDefinition) -> RwkvResult<String> {
        let mut sql = String::new();

        sql.push_str("CREATE ");

        if index_def.unique {
            sql.push_str("UNIQUE ");
        }

        sql.push_str("INDEX IF NOT EXISTS ");
        sql.push_str(&index_def.name);
        sql.push_str(" ON ");
        sql.push_str(&index_def.table);

        if let Some(expression) = &index_def.expression {
            // 表达式索引
            sql.push_str(" (");
            sql.push_str(expression);
            sql.push(')');
        } else {
            // 普通列索引
            sql.push_str(" (");
            sql.push_str(&index_def.columns.join(", "));
            sql.push(')');
        }

        if let Some(where_clause) = &index_def.where_clause {
            sql.push_str(" WHERE ");
            sql.push_str(where_clause);
        }

        Ok(sql)
    }

    /// 分析查询
    pub async fn analyze_query(&self, sql: &str) -> RwkvResult<QueryAnalysis> {
        let explain_sql = format!("EXPLAIN QUERY PLAN {}", sql);

        let rows = sqlx::query(&explain_sql)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| {
                RwkvError::new(
                    ErrorCode::DatabaseQueryFailed,
                    format!("查询分析失败: {}", e),
                )
            })?;

        let mut query_plan = String::new();
        let mut used_indexes = Vec::new();
        let mut optimization_suggestions = Vec::new();

        for row in rows {
            let detail: String = row.get("detail");
            query_plan.push_str(&detail);
            query_plan.push('\n');

            // 检查是否使用了索引
            if detail.contains("USING INDEX") {
                if let Some(index_start) = detail.find("USING INDEX ") {
                    let index_part = &detail[index_start + 12..];
                    if let Some(index_end) = index_part.find(' ') {
                        used_indexes.push(index_part[..index_end].to_string());
                    } else {
                        used_indexes.push(index_part.to_string());
                    }
                }
            }

            // 检查是否需要优化
            if detail.contains("SCAN TABLE") && !detail.contains("USING INDEX") {
                optimization_suggestions.push(format!("考虑为表扫描添加索引: {}", detail));
            }

            if detail.contains("TEMP B-TREE") {
                optimization_suggestions
                    .push("查询使用了临时B-Tree，考虑添加ORDER BY索引".to_string());
            }
        }

        // 计算复杂度评分
        let complexity_score = self.calculate_complexity_score(sql, &query_plan);

        Ok(QueryAnalysis {
            sql: sql.to_string(),
            query_plan,
            rows_scanned: None, // SQLite的EXPLAIN QUERY PLAN不提供此信息
            rows_returned: None,
            used_indexes,
            optimization_suggestions,
            complexity_score,
        })
    }

    /// 计算查询复杂度评分
    fn calculate_complexity_score(&self, sql: &str, query_plan: &str) -> u32 {
        let mut score = 1;

        // 基于SQL复杂度评分
        if sql.contains("JOIN") {
            score += 2;
        }
        if sql.contains("SUBQUERY") || sql.contains("SELECT") && sql.matches("SELECT").count() > 1 {
            score += 3;
        }
        if sql.contains("ORDER BY") {
            score += 1;
        }
        if sql.contains("GROUP BY") {
            score += 2;
        }
        if sql.contains("HAVING") {
            score += 2;
        }

        // 基于查询计划评分
        if query_plan.contains("SCAN TABLE") {
            score += 3;
        }
        if query_plan.contains("TEMP B-TREE") {
            score += 2;
        }
        if query_plan.contains("NESTED LOOP") {
            score += 2;
        }

        score.min(10)
    }

    /// 获取慢查询建议
    pub async fn get_slow_query_suggestions(&self, sql: &str) -> RwkvResult<Vec<String>> {
        let analysis = self.analyze_query(sql).await?;
        let mut suggestions = analysis.optimization_suggestions;

        // 添加通用优化建议
        if sql.contains("SELECT *") {
            suggestions.push("避免使用SELECT *，明确指定需要的列".to_string());
        }

        if sql.contains("LIKE '%") && sql.contains("%'") {
            suggestions.push("避免在LIKE条件的开头使用通配符，考虑全文搜索".to_string());
        }

        if sql.contains("ORDER BY")
            && !analysis
                .used_indexes
                .iter()
                .any(|idx| idx.contains("ORDER"))
        {
            suggestions.push("为ORDER BY子句添加索引以避免排序开销".to_string());
        }

        if analysis.complexity_score >= 7 {
            suggestions.push("查询复杂度较高，考虑分解为多个简单查询".to_string());
        }

        Ok(suggestions)
    }

    /// 优化表结构
    pub async fn optimize_table_structure(&self, table_name: &str) -> RwkvResult<Vec<String>> {
        let mut suggestions = Vec::new();

        // 分析表统计信息
        let _table_info = self.get_table_info(table_name).await?;

        // 检查是否需要VACUUM
        if self.should_vacuum_table(table_name).await? {
            suggestions.push(format!("建议对表 {} 执行VACUUM操作以回收空间", table_name));
        }

        // 检查是否需要ANALYZE
        if self.should_analyze_table(table_name).await? {
            suggestions.push(format!(
                "建议对表 {} 执行ANALYZE操作以更新统计信息",
                table_name
            ));
        }

        // 检查索引使用情况
        let unused_indexes = self.find_unused_indexes(table_name).await?;
        for index in unused_indexes {
            suggestions.push(format!("索引 {} 可能未被使用，考虑删除", index));
        }

        Ok(suggestions)
    }

    /// 获取表信息
    async fn get_table_info(
        &self,
        table_name: &str,
    ) -> RwkvResult<HashMap<String, serde_json::Value>> {
        let mut info = HashMap::new();

        // 获取表行数
        let count_sql = format!("SELECT COUNT(*) as count FROM {}", table_name);
        let row = sqlx::query(&count_sql)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| {
                RwkvError::new(
                    ErrorCode::DatabaseQueryFailed,
                    format!("获取表行数失败: {}", e),
                )
            })?;

        let count: i64 = row.get("count");
        info.insert(
            "row_count".to_string(),
            serde_json::Value::Number(count.into()),
        );

        // 获取表大小信息（SQLite特定）
        let _size_sql = "SELECT 
            name,
            (page_count * page_size) as size_bytes
        FROM pragma_table_info(?) t
        JOIN (SELECT page_count, page_size FROM pragma_page_count, pragma_page_size) p";

        // 注意：这里需要更准确的SQLite表大小查询方法

        Ok(info)
    }

    /// 检查是否需要VACUUM
    async fn should_vacuum_table(&self, _table_name: &str) -> RwkvResult<bool> {
        // 简化实现：基于空闲页面比例判断
        let result = sqlx::query("PRAGMA freelist_count")
            .fetch_one(&self.pool)
            .await
            .map_err(|e| {
                RwkvError::new(
                    ErrorCode::DatabaseQueryFailed,
                    format!("检查空闲页面失败: {}", e),
                )
            })?;

        let freelist_count: i64 = result.get(0);

        // 如果空闲页面超过1000，建议VACUUM
        Ok(freelist_count > 1000)
    }

    /// 检查是否需要ANALYZE
    async fn should_analyze_table(&self, table_name: &str) -> RwkvResult<bool> {
        // 检查统计信息是否过期
        let result = sqlx::query("SELECT COUNT(*) as count FROM sqlite_stat1 WHERE tbl = ?")
            .bind(table_name)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| {
                RwkvError::new(
                    ErrorCode::DatabaseQueryFailed,
                    format!("检查统计信息失败: {}", e),
                )
            })?;

        let stat_count: i64 = result.get("count");

        // 如果没有统计信息，建议ANALYZE
        Ok(stat_count == 0)
    }

    /// 查找未使用的索引
    async fn find_unused_indexes(&self, table_name: &str) -> RwkvResult<Vec<String>> {
        // 注意：SQLite没有直接的索引使用统计
        // 这里返回一个空列表，实际实现需要结合查询日志分析
        let _unused_indexes: Vec<String> = Vec::new();

        // 获取表的所有索引
        let indexes = sqlx::query(
            "SELECT name FROM sqlite_master WHERE type='index' AND tbl_name=? AND name NOT LIKE 'sqlite_%'"
        )
        .bind(table_name)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| RwkvError::new(
            ErrorCode::DatabaseQueryFailed,
            format!("获取索引列表失败: {}", e)
        ))?;

        let index_names: Vec<String> = indexes
            .iter()
            .map(|row| row.get::<String, _>("name"))
            .collect();

        // 实际应用中，这里需要分析查询日志来确定哪些索引未被使用
        Ok(index_names)
    }

    /// 执行数据库优化
    pub async fn optimize_database(&self) -> RwkvResult<Vec<String>> {
        let mut results = Vec::new();

        // 执行ANALYZE更新统计信息
        sqlx::query("ANALYZE")
            .execute(&self.pool)
            .await
            .map_err(|e| {
                RwkvError::new(
                    ErrorCode::DatabaseQueryFailed,
                    format!("ANALYZE执行失败: {}", e),
                )
            })?;

        results.push("数据库统计信息已更新".to_string());

        // 检查是否需要VACUUM
        if self.should_vacuum_table("").await? {
            sqlx::query("VACUUM")
                .execute(&self.pool)
                .await
                .map_err(|e| {
                    RwkvError::new(
                        ErrorCode::DatabaseQueryFailed,
                        format!("VACUUM执行失败: {}", e),
                    )
                })?;

            results.push("数据库空间已优化".to_string());
        }

        // 优化SQLite设置
        let optimizations = vec![
            "PRAGMA journal_mode = WAL",
            "PRAGMA synchronous = NORMAL",
            "PRAGMA cache_size = -64000", // 64MB缓存
            "PRAGMA temp_store = MEMORY",
            "PRAGMA mmap_size = 134217728", // 128MB内存映射
        ];

        for pragma in optimizations {
            sqlx::query(pragma).execute(&self.pool).await.map_err(|e| {
                RwkvError::new(
                    ErrorCode::DatabaseQueryFailed,
                    format!("PRAGMA设置失败 {}: {}", pragma, e),
                )
            })?;

            results.push(format!("已应用优化设置: {}", pragma));
        }

        Ok(results)
    }

    /// 获取数据库统计信息
    pub async fn get_database_stats(&self) -> RwkvResult<HashMap<String, serde_json::Value>> {
        let mut stats = HashMap::new();

        // 数据库大小
        let size_result = sqlx::query(
            "SELECT page_count * page_size as size FROM pragma_page_count(), pragma_page_size()",
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            RwkvError::new(
                ErrorCode::DatabaseQueryFailed,
                format!("获取数据库大小失败: {}", e),
            )
        })?;

        let db_size: i64 = size_result.get("size");
        stats.insert(
            "database_size_bytes".to_string(),
            serde_json::Value::Number(db_size.into()),
        );

        // 空闲页面数
        let freelist_result = sqlx::query("PRAGMA freelist_count")
            .fetch_one(&self.pool)
            .await
            .map_err(|e| {
                RwkvError::new(
                    ErrorCode::DatabaseQueryFailed,
                    format!("获取空闲页面数失败: {}", e),
                )
            })?;

        let freelist_count: i64 = freelist_result.get(0);
        stats.insert(
            "freelist_count".to_string(),
            serde_json::Value::Number(freelist_count.into()),
        );

        // 索引数量
        let index_result =
            sqlx::query("SELECT COUNT(*) as count FROM sqlite_master WHERE type='index'")
                .fetch_one(&self.pool)
                .await
                .map_err(|e| {
                    RwkvError::new(
                        ErrorCode::DatabaseQueryFailed,
                        format!("获取索引数量失败: {}", e),
                    )
                })?;

        let index_count: i64 = index_result.get("count");
        stats.insert(
            "index_count".to_string(),
            serde_json::Value::Number(index_count.into()),
        );

        Ok(stats)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::SqlitePool;
    use tempfile::NamedTempFile;

    async fn setup_test_db() -> SqlitePool {
        let temp_file = NamedTempFile::new().unwrap();
        let database_url = format!("sqlite:{}", temp_file.path().to_str().unwrap());

        let pool = SqlitePool::connect(&database_url).await.unwrap();

        // 创建测试表
        sqlx::query(
            "CREATE TABLE test_table (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                value INTEGER,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
        )
        .execute(&pool)
        .await
        .unwrap();

        pool
    }

    #[tokio::test]
    async fn test_index_creation() {
        let pool = setup_test_db().await;
        let optimizer = QueryOptimizer::new(pool);

        let index_def = IndexDefinition {
            name: "test_index".to_string(),
            table: "test_table".to_string(),
            columns: vec!["name".to_string()],
            index_type: IndexType::BTree,
            unique: false,
            where_clause: None,
            expression: None,
            auto_create: true,
        };

        let result = optimizer.create_index(&index_def).await;
        assert!(result.is_ok());

        // 检查索引是否存在
        let exists = optimizer.index_exists("test_index").await.unwrap();
        assert!(exists);
    }

    #[tokio::test]
    async fn test_query_analysis() {
        let pool = setup_test_db().await;
        let optimizer = QueryOptimizer::new(pool);

        let sql = "SELECT * FROM test_table WHERE name = 'test'";
        let analysis = optimizer.analyze_query(sql).await.unwrap();

        assert_eq!(analysis.sql, sql);
        assert!(!analysis.query_plan.is_empty());
        assert!(analysis.complexity_score >= 1);
    }

    #[tokio::test]
    async fn test_database_optimization() {
        let pool = setup_test_db().await;
        let optimizer = QueryOptimizer::new(pool);

        let results = optimizer.optimize_database().await.unwrap();
        assert!(!results.is_empty());
        assert!(results.iter().any(|r| r.contains("统计信息已更新")));
    }
}

//! 工具模块
//!
//! 提供记忆系统的实用工具函数，包括向量操作、文本处理、时间处理、
//! 序列化/反序列化、缓存管理等。

use crate::core_types::{ConnectionId, MemoryId};
use crate::error::{MemoryError, Result};
use chrono::{DateTime, Duration, Timelike, Utc};
use flate2::{read::GzDecoder, write::GzEncoder, Compression};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{HashMap, HashSet};
use std::io::{Read, Write};
use uuid::Uuid;

/// 向量工具
pub struct VectorUtils;

/// 文本工具
pub struct TextUtils;

/// 时间工具
pub struct TimeUtils;

/// 序列化工具
pub struct SerializationUtils;

/// 缓存工具
pub struct CacheUtils;

/// 哈希工具
pub struct HashUtils;

/// ID生成器
pub struct IdGenerator;

/// 相似度计算结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimilarityResult {
    pub score: f32,
    pub method: SimilarityMethod,
    pub details: HashMap<String, f32>,
}

/// 相似度计算方法
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SimilarityMethod {
    Cosine,
    Euclidean,
    Manhattan,
    Jaccard,
    Semantic,
    Hybrid,
}

/// 文本统计信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextStats {
    pub char_count: usize,
    pub word_count: usize,
    pub sentence_count: usize,
    pub paragraph_count: usize,
    pub language: Option<String>,
    pub complexity_score: f32,
    pub readability_score: f32,
}

/// 时间范围
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeRange {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

/// 压缩结果
#[derive(Debug, Clone)]
pub struct CompressionResult {
    pub compressed_data: Vec<u8>,
    pub original_size: usize,
    pub compressed_size: usize,
    pub compression_ratio: f32,
}

/// 缓存统计信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheStats {
    pub hit_count: u64,
    pub miss_count: u64,
    pub hit_rate: f32,
    pub total_size: usize,
    pub entry_count: usize,
    pub last_cleanup: Option<DateTime<Utc>>,
}

impl VectorUtils {
    /// 计算余弦相似度
    pub fn cosine_similarity(a: &[f32], b: &[f32]) -> Result<f32> {
        if a.len() != b.len() {
            return Err(MemoryError::InvalidVectorDimension {
                expected: a.len(),
                actual: b.len(),
            });
        }

        if a.is_empty() {
            return Ok(0.0);
        }

        let dot_product: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
        let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
        let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();

        if norm_a == 0.0 || norm_b == 0.0 {
            return Ok(0.0);
        }

        Ok(dot_product / (norm_a * norm_b))
    }

    /// 计算欧几里得距离
    pub fn euclidean_distance(a: &[f32], b: &[f32]) -> Result<f32> {
        if a.len() != b.len() {
            return Err(MemoryError::InvalidVectorDimension {
                expected: a.len(),
                actual: b.len(),
            });
        }

        let distance: f32 = a
            .iter()
            .zip(b.iter())
            .map(|(x, y)| (x - y).powi(2))
            .sum::<f32>()
            .sqrt();

        Ok(distance)
    }

    /// 计算曼哈顿距离
    pub fn manhattan_distance(a: &[f32], b: &[f32]) -> Result<f32> {
        if a.len() != b.len() {
            return Err(MemoryError::InvalidVectorDimension {
                expected: a.len(),
                actual: b.len(),
            });
        }

        let distance: f32 = a.iter().zip(b.iter()).map(|(x, y)| (x - y).abs()).sum();

        Ok(distance)
    }

    /// 向量归一化
    pub fn normalize(vector: &mut [f32]) -> Result<()> {
        let norm: f32 = vector.iter().map(|x| x * x).sum::<f32>().sqrt();

        if norm == 0.0 {
            return Err(MemoryError::Internal {
                message: "Cannot normalize zero vector".to_string(),
            });
        }

        for value in vector.iter_mut() {
            *value /= norm;
        }

        Ok(())
    }

    /// 向量加权平均
    pub fn weighted_average(vectors: &[Vec<f32>], weights: &[f32]) -> Result<Vec<f32>> {
        if vectors.is_empty() {
            return Err(MemoryError::Internal {
                message: "No vectors provided".to_string(),
            });
        }

        if vectors.len() != weights.len() {
            return Err(MemoryError::Internal {
                message: "Vector and weight count mismatch".to_string(),
            });
        }

        let dim = vectors[0].len();
        if !vectors.iter().all(|v| v.len() == dim) {
            return Err(MemoryError::Internal {
                message: "Inconsistent vector dimensions".to_string(),
            });
        }

        let weight_sum: f32 = weights.iter().sum();
        if weight_sum == 0.0 {
            return Err(MemoryError::Internal {
                message: "Zero weight sum".to_string(),
            });
        }

        let mut result = vec![0.0; dim];
        for (vector, &weight) in vectors.iter().zip(weights.iter()) {
            for (i, &value) in vector.iter().enumerate() {
                result[i] += value * weight / weight_sum;
            }
        }

        Ok(result)
    }

    /// 计算向量中心点
    pub fn centroid(vectors: &[Vec<f32>]) -> Result<Vec<f32>> {
        if vectors.is_empty() {
            return Err(MemoryError::Internal {
                message: "No vectors provided".to_string(),
            });
        }

        let weights = vec![1.0; vectors.len()];
        Self::weighted_average(vectors, &weights)
    }

    /// 向量量化（减少精度以节省空间）
    pub fn quantize(vector: &[f32], levels: u8) -> Vec<u8> {
        let min_val = vector.iter().fold(f32::INFINITY, |a, &b| a.min(b));
        let max_val = vector.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
        let range = max_val - min_val;

        if range == 0.0 {
            return vec![0; vector.len()];
        }

        let scale = (levels - 1) as f32 / range;

        vector
            .iter()
            .map(|&val| ((val - min_val) * scale).round() as u8)
            .collect()
    }

    /// 反量化
    pub fn dequantize(quantized: &[u8], min_val: f32, max_val: f32, levels: u8) -> Vec<f32> {
        let range = max_val - min_val;
        let scale = range / (levels - 1) as f32;

        quantized
            .iter()
            .map(|&val| min_val + val as f32 * scale)
            .collect()
    }
}

impl TextUtils {
    /// 计算文本统计信息
    pub fn analyze_text(text: &str) -> TextStats {
        let char_count = text.chars().count();
        let word_count = text.split_whitespace().count();
        // 改进句子计数逻辑，支持中英文标点符号
        let sentence_count = if text.trim().is_empty() {
            0
        } else {
            // 支持中英文标点符号
            let english_count = text.matches(&['.', '!', '?'][..]).count();
            let chinese_count = text.matches(&['。', '！', '？'][..]).count();
            let total_count = english_count + chinese_count;
            if total_count == 0 {
                1 // 如果没有句子结束符，认为是一个句子
            } else {
                total_count
            }
        };
        let paragraph_count = text
            .split("\n\n")
            .filter(|p| !p.trim().is_empty())
            .count()
            .max(1);

        let complexity_score = Self::calculate_complexity(text);
        let readability_score = Self::calculate_readability(text, word_count, sentence_count);
        let language = Self::detect_language(text);

        TextStats {
            char_count,
            word_count,
            sentence_count,
            paragraph_count,
            language,
            complexity_score,
            readability_score,
        }
    }

    /// 计算文本复杂度
    fn calculate_complexity(text: &str) -> f32 {
        let words: Vec<&str> = text.split_whitespace().collect();
        if words.is_empty() {
            return 0.0;
        }

        let avg_word_length: f32 = words
            .iter()
            .map(|word| word.chars().count() as f32)
            .sum::<f32>()
            / words.len() as f32;

        let unique_words: HashSet<&str> = words.iter().cloned().collect();
        let vocabulary_diversity = unique_words.len() as f32 / words.len() as f32;

        // 简化的复杂度计算
        (avg_word_length / 10.0 + vocabulary_diversity) / 2.0
    }

    /// 计算可读性分数（简化的Flesch Reading Ease）
    fn calculate_readability(text: &str, word_count: usize, sentence_count: usize) -> f32 {
        if word_count == 0 || sentence_count == 0 {
            return 0.0;
        }

        let syllable_count = Self::estimate_syllables(text);
        let avg_sentence_length = word_count as f32 / sentence_count as f32;
        let avg_syllables_per_word = syllable_count as f32 / word_count as f32;

        // 简化的Flesch公式
        206.835 - (1.015 * avg_sentence_length) - (84.6 * avg_syllables_per_word)
    }

    /// 估算音节数
    fn estimate_syllables(text: &str) -> usize {
        let vowels = ['a', 'e', 'i', 'o', 'u', 'y'];
        let mut syllable_count = 0;

        for word in text.split_whitespace() {
            let word = word.to_lowercase();
            let mut word_syllables = 0;
            let mut prev_was_vowel = false;

            for ch in word.chars() {
                let is_vowel = vowels.contains(&ch);
                if is_vowel && !prev_was_vowel {
                    word_syllables += 1;
                }
                prev_was_vowel = is_vowel;
            }

            // 每个单词至少有一个音节
            syllable_count += word_syllables.max(1);
        }

        syllable_count
    }

    /// 简单的语言检测
    fn detect_language(text: &str) -> Option<String> {
        let chinese_chars = text
            .chars()
            .filter(|c| {
                let code = *c as u32;
                (0x4E00..=0x9FFF).contains(&code) // CJK统一汉字
            })
            .count();

        let total_chars = text.chars().filter(|c| c.is_alphabetic()).count();

        if total_chars == 0 {
            return None;
        }

        if chinese_chars as f32 / total_chars as f32 > 0.3 {
            Some("zh".to_string())
        } else {
            Some("en".to_string())
        }
    }

    /// 提取关键词（简单实现）
    pub fn extract_keywords(text: &str, max_keywords: usize) -> Vec<String> {
        let stop_words: HashSet<&str> = [
            "the", "a", "an", "and", "or", "but", "in", "on", "at", "to", "for", "of", "with",
            "by", "is", "are", "was", "were", "be", "been", "have", "has", "had", "do", "does",
            "did", "will", "would", "could", "should", "的", "了", "在", "是", "我", "你", "他",
            "她", "它", "们", "这", "那", "有", "和", "与", "或", "但", "如果", "因为", "所以",
            "然后", "现在", "一个", "多个", "包含", "还有",
        ]
        .iter()
        .cloned()
        .collect();

        let mut word_freq: HashMap<String, usize> = HashMap::new();

        // 处理中文文本，按字符分割
        let chinese_chars = text
            .chars()
            .filter(|c| {
                let code = *c as u32;
                (0x4E00..=0x9FFF).contains(&code)
            })
            .count();

        if chinese_chars > text.chars().count() / 3 {
            // 中文文本处理
            for word in text.split_whitespace() {
                let cleaned = word
                    .chars()
                    .filter(|c| c.is_alphanumeric())
                    .collect::<String>();

                if cleaned.len() >= 2 && !stop_words.contains(cleaned.as_str()) {
                    *word_freq.entry(cleaned).or_insert(0) += 1;
                }
            }
        } else {
            // 英文文本处理
            for word in text.split_whitespace() {
                let word = word
                    .to_lowercase()
                    .trim_matches(|c: char| !c.is_alphanumeric())
                    .to_string();

                if word.len() > 2 && !stop_words.contains(word.as_str()) {
                    *word_freq.entry(word).or_insert(0) += 1;
                }
            }
        }

        let mut keywords: Vec<(String, usize)> = word_freq.into_iter().collect();
        keywords.sort_by(|a, b| b.1.cmp(&a.1));

        keywords
            .into_iter()
            .take(max_keywords)
            .map(|(word, _)| word)
            .collect()
    }

    /// 计算Jaccard相似度
    pub fn jaccard_similarity(text1: &str, text2: &str) -> f32 {
        let words1: HashSet<&str> = text1.split_whitespace().collect();
        let words2: HashSet<&str> = text2.split_whitespace().collect();

        if words1.is_empty() && words2.is_empty() {
            return 1.0;
        }

        let intersection = words1.intersection(&words2).count();
        let union = words1.union(&words2).count();

        if union == 0 {
            0.0
        } else {
            intersection as f32 / union as f32
        }
    }

    /// 文本摘要（简单实现）
    pub fn summarize(text: &str, max_sentences: usize) -> String {
        let sentences: Vec<&str> = text
            .split(&['.', '!', '?'][..])
            .filter(|s| !s.trim().is_empty())
            .collect();

        if sentences.len() <= max_sentences {
            return text.to_string();
        }

        // 简单地取前几句
        sentences
            .iter()
            .take(max_sentences)
            .map(|s| s.trim())
            .collect::<Vec<_>>()
            .join(". ")
            + "."
    }
}

impl TimeUtils {
    /// 创建时间范围
    pub fn create_range(start: DateTime<Utc>, end: DateTime<Utc>) -> Result<TimeRange> {
        if start > end {
            return Err(MemoryError::Internal {
                message: "Start time must be before end time".to_string(),
            });
        }

        Ok(TimeRange { start, end })
    }

    /// 检查时间是否在范围内
    pub fn is_in_range(time: DateTime<Utc>, range: &TimeRange) -> bool {
        time >= range.start && time <= range.end
    }

    /// 计算时间衰减因子
    pub fn calculate_decay_factor(timestamp: DateTime<Utc>, half_life: Duration) -> f32 {
        let elapsed = Utc::now() - timestamp;
        let half_life_seconds = half_life.num_seconds() as f64;
        let elapsed_seconds = elapsed.num_seconds() as f64;

        if half_life_seconds <= 0.0 {
            return 1.0;
        }

        (0.5_f64.powf(elapsed_seconds / half_life_seconds)) as f32
    }

    /// 获取时间段描述
    pub fn get_time_period_description(timestamp: DateTime<Utc>) -> String {
        let now = Utc::now();
        let diff = now - timestamp;

        if diff < Duration::minutes(1) {
            "刚刚".to_string()
        } else if diff < Duration::hours(1) {
            format!("{}分钟前", diff.num_minutes())
        } else if diff < Duration::days(1) {
            format!("{}小时前", diff.num_hours())
        } else if diff < Duration::days(7) {
            format!("{}天前", diff.num_days())
        } else if diff < Duration::days(30) {
            format!("{}周前", diff.num_weeks())
        } else if diff < Duration::days(365) {
            format!("{}个月前", diff.num_days() / 30)
        } else {
            format!("{}年前", diff.num_days() / 365)
        }
    }

    /// 获取一天中的时间段
    pub fn get_time_of_day_period(timestamp: DateTime<Utc>) -> String {
        let hour = timestamp.hour();

        match hour {
            5..=11 => "上午".to_string(),
            12..=17 => "下午".to_string(),
            18..=22 => "晚上".to_string(),
            _ => "深夜".to_string(),
        }
    }

    /// 计算时间相似度
    pub fn temporal_similarity(
        time1: DateTime<Utc>,
        time2: DateTime<Utc>,
        window: Duration,
    ) -> f32 {
        let diff = (time1 - time2).abs();
        let window_seconds = window.num_seconds() as f64;
        let diff_seconds = diff.num_seconds() as f64;

        if diff_seconds >= window_seconds {
            0.0
        } else {
            1.0 - (diff_seconds / window_seconds) as f32
        }
    }
}

impl SerializationUtils {
    /// 压缩数据
    pub fn compress(data: &[u8]) -> Result<CompressionResult> {
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(data).map_err(MemoryError::Io)?;

        let compressed_data = encoder.finish().map_err(MemoryError::Io)?;

        let original_size = data.len();
        let compressed_size = compressed_data.len();
        let compression_ratio = if original_size > 0 {
            compressed_size as f32 / original_size as f32
        } else {
            1.0
        };

        Ok(CompressionResult {
            compressed_data,
            original_size,
            compressed_size,
            compression_ratio,
        })
    }

    /// 解压缩数据
    pub fn decompress(compressed_data: &[u8]) -> Result<Vec<u8>> {
        let mut decoder = GzDecoder::new(compressed_data);
        let mut decompressed = Vec::new();

        decoder
            .read_to_end(&mut decompressed)
            .map_err(MemoryError::Io)?;

        Ok(decompressed)
    }

    /// 序列化为JSON
    pub fn to_json<T: Serialize>(data: &T) -> Result<String> {
        serde_json::to_string(data).map_err(MemoryError::Serialization)
    }

    /// 从JSON反序列化
    pub fn from_json<T: for<'de> Deserialize<'de>>(json: &str) -> Result<T> {
        serde_json::from_str(json).map_err(MemoryError::Serialization)
    }

    /// 序列化为二进制
    pub fn to_binary<T: Serialize + bincode::Encode>(data: &T) -> Result<Vec<u8>> {
        bincode::encode_to_vec(data, bincode::config::standard()).map_err(|e| {
            MemoryError::Internal {
                message: format!("Binary serialization failed: {}", e),
            }
        })
    }

    /// 从二进制反序列化
    pub fn from_binary<T: for<'de> Deserialize<'de> + bincode::Decode<()>>(
        data: &[u8],
    ) -> Result<T> {
        bincode::decode_from_slice(data, bincode::config::standard())
            .map(|(result, _)| result)
            .map_err(|e| MemoryError::Internal {
                message: format!("Binary deserialization failed: {}", e),
            })
    }
}

impl CacheUtils {
    /// 计算缓存命中率
    pub fn calculate_hit_rate(hit_count: u64, miss_count: u64) -> f32 {
        let total = hit_count + miss_count;
        if total == 0 {
            0.0
        } else {
            hit_count as f32 / total as f32
        }
    }

    /// 估算内存使用量
    pub fn estimate_memory_usage<T>(items: &[T]) -> usize {
        std::mem::size_of_val(items)
    }

    /// 生成缓存键
    pub fn generate_cache_key(components: &[&str]) -> String {
        let mut hasher = Sha256::new();
        for component in components {
            hasher.update(component.as_bytes());
        }
        format!("{:x}", hasher.finalize())
    }

    /// 检查缓存是否需要清理
    pub fn should_cleanup(last_cleanup: Option<DateTime<Utc>>, interval: Duration) -> bool {
        match last_cleanup {
            Some(last) => Utc::now() - last > interval,
            None => true,
        }
    }
}

impl HashUtils {
    /// 计算字符串哈希
    pub fn hash_string(input: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    /// 计算数据哈希
    pub fn hash_bytes(data: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        format!("{:x}", hasher.finalize())
    }

    /// 生成短哈希（用于显示）
    pub fn short_hash(input: &str, length: usize) -> String {
        let full_hash = Self::hash_string(input);
        full_hash
            .chars()
            .take(length.min(full_hash.len()))
            .collect()
    }

    /// 验证哈希
    pub fn verify_hash(input: &str, expected_hash: &str) -> bool {
        Self::hash_string(input) == expected_hash
    }
}

impl IdGenerator {
    /// 生成记忆ID
    pub fn generate_memory_id() -> MemoryId {
        format!("mem_{}", Uuid::new_v4())
    }

    /// 生成连接ID
    pub fn generate_connection_id() -> ConnectionId {
        format!("conn_{}", Uuid::new_v4())
    }

    /// 生成会话ID
    pub fn generate_session_id() -> String {
        format!("session_{}", Uuid::new_v4())
    }

    /// 生成查询ID
    pub fn generate_query_id() -> String {
        format!("query_{}", Uuid::new_v4())
    }

    /// 生成交互ID
    pub fn generate_interaction_id() -> String {
        format!("interaction_{}", Uuid::new_v4())
    }

    /// 从时间戳生成ID
    pub fn generate_time_based_id(prefix: &str) -> String {
        let timestamp = Utc::now().timestamp_millis();
        let uuid = Uuid::new_v4();
        format!("{}_{:x}_{}", prefix, timestamp, uuid.simple())
    }

    /// 生成短ID（用于显示）
    pub fn generate_short_id(length: usize) -> String {
        let uuid = Uuid::new_v4();
        uuid.simple().to_string().chars().take(length).collect()
    }
}

/// 批处理工具
pub struct BatchUtils;

impl BatchUtils {
    /// 将数据分批处理
    pub fn chunk<T: Clone>(data: Vec<T>, chunk_size: usize) -> Vec<Vec<T>> {
        if chunk_size == 0 {
            return vec![data];
        }

        data.chunks(chunk_size)
            .map(|chunk| chunk.to_vec())
            .collect()
    }

    /// 并行处理批次
    pub async fn process_batches<T, F, R>(batches: Vec<Vec<T>>, processor: F) -> Vec<Result<R>>
    where
        T: Send + 'static,
        F: Fn(Vec<T>) -> Result<R> + Send + Sync + 'static + Clone,
        R: Send + 'static,
    {
        let mut handles = Vec::new();

        for batch in batches {
            let processor = processor.clone();
            let handle = tokio::spawn(async move { processor(batch) });
            handles.push(handle);
        }

        let mut results = Vec::new();
        for handle in handles {
            match handle.await {
                Ok(result) => results.push(result),
                Err(e) => results.push(Err(MemoryError::Internal {
                    message: e.to_string(),
                })),
            }
        }

        results
    }
}

/// 性能监控工具
pub struct PerformanceUtils;

impl PerformanceUtils {
    /// 测量执行时间
    pub async fn measure_time<F, T>(operation: F) -> (T, Duration)
    where
        F: std::future::Future<Output = T>,
    {
        let start = Utc::now();
        let result = operation.await;
        let duration = Utc::now() - start;
        (result, duration)
    }

    /// 创建性能报告
    pub fn create_performance_report(
        operation_name: &str,
        duration: Duration,
        items_processed: usize,
    ) -> HashMap<String, serde_json::Value> {
        let mut report = HashMap::new();

        report.insert(
            "operation".to_string(),
            serde_json::Value::String(operation_name.to_string()),
        );
        report.insert(
            "duration_ms".to_string(),
            serde_json::Value::Number(duration.num_milliseconds().into()),
        );
        report.insert(
            "items_processed".to_string(),
            serde_json::Value::Number(items_processed.into()),
        );

        if items_processed > 0 && duration.num_milliseconds() > 0 {
            let throughput = items_processed as f64 / (duration.num_milliseconds() as f64 / 1000.0);
            report.insert(
                "throughput_per_second".to_string(),
                serde_json::Value::Number(
                    serde_json::Number::from_f64(throughput).unwrap_or_else(|| 0.into()),
                ),
            );
        }

        report
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_cosine_similarity() {
        let a = vec![1.0, 2.0, 3.0];
        let b = vec![4.0, 5.0, 6.0];

        let similarity = VectorUtils::cosine_similarity(&a, &b).unwrap();
        assert!(similarity > 0.9); // 应该很相似
    }

    #[test]
    fn test_vector_normalization() {
        let mut vector = vec![3.0, 4.0];
        VectorUtils::normalize(&mut vector).unwrap();

        let norm: f32 = vector.iter().map(|x| x * x).sum::<f32>().sqrt();
        assert!((norm - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_text_analysis() {
        let text = "这是一个测试文本。它包含多个句子！还有问号吗？";
        let stats = TextUtils::analyze_text(text);

        assert!(stats.char_count > 0);
        assert!(stats.word_count > 0);
        assert_eq!(stats.sentence_count, 3);
        assert_eq!(stats.language, Some("zh".to_string()));
    }

    #[test]
    fn test_keyword_extraction() {
        let text = "机器学习是人工智能的一个重要分支。深度学习是机器学习的子领域。";
        let keywords = TextUtils::extract_keywords(text, 3);

        assert!(!keywords.is_empty());
        assert!(keywords.len() > 0);
    }

    #[test]
    fn test_time_decay() {
        let now = Utc::now();
        let past = now - Duration::hours(1);
        let half_life = Duration::hours(2);

        let decay = TimeUtils::calculate_decay_factor(past, half_life);
        assert!(decay > 0.7 && decay < 0.8); // 应该约为0.707
    }

    #[test]
    fn test_compression() {
        // 使用更长的重复文本以确保压缩效果
        let data = "这是一些测试数据，应该可以被压缩。重复的内容重复的内容重复的内容重复的内容重复的内容重复的内容重复的内容重复的内容重复的内容重复的内容重复的内容重复的内容重复的内容重复的内容重复的内容重复的内容重复的内容重复的内容重复的内容重复的内容。".as_bytes();
        let result = SerializationUtils::compress(data).unwrap();

        // 对于重复内容，压缩应该有效果
        assert!(result.compressed_size <= result.original_size);
        assert!(result.compression_ratio <= 1.0);

        let decompressed = SerializationUtils::decompress(&result.compressed_data).unwrap();
        assert_eq!(data, decompressed.as_slice());
    }

    #[test]
    fn test_hash_generation() {
        let input = "测试字符串";
        let hash1 = HashUtils::hash_string(input);
        let hash2 = HashUtils::hash_string(input);

        assert_eq!(hash1, hash2); // 相同输入应该产生相同哈希
        assert_eq!(hash1.len(), 64); // SHA256产生64字符的十六进制字符串

        let short_hash = HashUtils::short_hash(input, 8);
        assert_eq!(short_hash.len(), 8);
    }

    #[test]
    fn test_id_generation() {
        let memory_id = IdGenerator::generate_memory_id();
        let connection_id = IdGenerator::generate_connection_id();

        assert!(memory_id.starts_with("mem_"));
        assert!(connection_id.starts_with("conn_"));

        // 确保生成的ID是唯一的
        let id1 = IdGenerator::generate_memory_id();
        let id2 = IdGenerator::generate_memory_id();
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_batch_chunking() {
        let data: Vec<i32> = (1..=10).collect();
        let chunks = BatchUtils::chunk(data, 3);

        assert_eq!(chunks.len(), 4); // 10个元素，每批3个，应该有4批
        assert_eq!(chunks[0], vec![1, 2, 3]);
        assert_eq!(chunks[3], vec![10]); // 最后一批只有1个元素
    }

    #[test]
    fn test_cache_hit_rate() {
        let hit_rate = CacheUtils::calculate_hit_rate(80, 20);
        assert_eq!(hit_rate, 0.8);

        let hit_rate_zero = CacheUtils::calculate_hit_rate(0, 0);
        assert_eq!(hit_rate_zero, 0.0);
    }

    #[tokio::test]
    async fn test_performance_measurement() {
        let (result, duration) = PerformanceUtils::measure_time(async {
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            42
        })
        .await;

        assert_eq!(result, 42);
        assert!(duration.num_milliseconds() >= 100);
    }
}

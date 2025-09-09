//! 嵌入模型服务
//! 基于 model2vec-rs 提供快速文本嵌入功能

use anyhow::Result;
use model2vec_rs::model::StaticModel;
use std::sync::Arc;
use tokio::sync::Mutex;

/// 嵌入模型服务
pub struct EmbeddingService {
    model: Arc<Mutex<Option<StaticModel>>>,
    model_path: String,
    embedding_dim: usize,
}

impl EmbeddingService {
    /// 创建新的嵌入服务实例
    pub fn new(model_path: String) -> Self {
        Self {
            model: Arc::new(Mutex::new(None)),
            model_path,
            embedding_dim: 0,
        }
    }

    /// 初始化模型（延迟加载）
    pub async fn initialize(&mut self) -> Result<()> {
        let mut model_guard = self.model.lock().await;

        if model_guard.is_some() {
            return Ok(());
        }

        log::info!("Loading model2vec model from: {}", self.model_path);

        // 加载 model2vec 模型
        let static_model = StaticModel::from_pretrained(self.model_path.clone(), None, None, None)
            .map_err(|e| anyhow::anyhow!("Failed to load model2vec model: {}", e))?;

        // 检测嵌入维度
        let test_embedding = static_model.encode(&["test".to_string()]);

        self.embedding_dim = test_embedding.first().map(|emb| emb.len()).unwrap_or(0);

        log::info!(
            "Model2vec model loaded successfully, embedding dimension: {}",
            self.embedding_dim
        );

        *model_guard = Some(static_model);
        Ok(())
    }

    /// 编码文本为嵌入向量
    pub async fn encode(&self, texts: &[&str]) -> Result<Vec<Vec<f32>>> {
        let model_guard = self.model.lock().await;

        let model = model_guard
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Model not initialized"))?;

        log::debug!("Encoding {} texts", texts.len());

        let inputs: Vec<String> = texts.iter().map(|s| s.to_string()).collect();
        let embeddings = model.encode(&inputs);

        log::debug!("Successfully encoded {} embeddings", embeddings.len());

        Ok(embeddings)
    }

    /// 编码单个文本
    pub async fn encode_single(&self, text: &str) -> Result<Vec<f32>> {
        let embeddings = self.encode(&[text]).await?;
        embeddings
            .into_iter()
            .next()
            .ok_or_else(|| anyhow::anyhow!("No embedding returned"))
    }

    /// 获取嵌入维度
    pub fn embedding_dim(&self) -> usize {
        self.embedding_dim
    }

    /// 计算余弦相似度
    pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
        if a.len() != b.len() {
            return 0.0;
        }

        let dot_product: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
        let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
        let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();

        if norm_a == 0.0 || norm_b == 0.0 {
            return 0.0;
        }

        dot_product / (norm_a * norm_b)
    }

    /// 序列化嵌入向量为字节
    pub fn serialize_embedding(embedding: &[f32]) -> Result<Vec<u8>> {
        let cfg = bincode::config::standard();
        bincode::encode_to_vec(embedding.to_vec(), cfg)
            .map_err(|e| anyhow::anyhow!("Failed to serialize embedding: {}", e))
    }

    /// 反序列化字节为嵌入向量
    pub fn deserialize_embedding(bytes: &[u8]) -> Result<Vec<f32>> {
        let cfg = bincode::config::standard();
        let (vec, _len): (Vec<f32>, usize) = bincode::decode_from_slice(bytes, cfg)
            .map_err(|e| anyhow::anyhow!("Failed to deserialize embedding: {}", e))?;
        Ok(vec)
    }
}

impl Default for EmbeddingService {
    fn default() -> Self {
        Self::new("./qwen3-model2vec".to_string())
    }
}

/// 全局嵌入服务单例
static EMBEDDING_SERVICE: once_cell::sync::OnceCell<Arc<Mutex<EmbeddingService>>> =
    once_cell::sync::OnceCell::new();

/// 初始化全局嵌入服务
pub async fn initialize_global_embedding_service(model_path: Option<String>) -> Result<()> {
    let path = model_path.unwrap_or_else(|| "./qwen3-model2vec".to_string());
    let mut service = EmbeddingService::new(path);
    service.initialize().await?;

    EMBEDDING_SERVICE
        .set(Arc::new(Mutex::new(service)))
        .map_err(|_| anyhow::anyhow!("Global embedding service already initialized"))?;

    Ok(())
}

/// 获取全局嵌入服务
pub fn get_global_embedding_service() -> Result<Arc<Mutex<EmbeddingService>>> {
    EMBEDDING_SERVICE
        .get()
        .ok_or_else(|| anyhow::anyhow!("Global embedding service not initialized")).cloned()
}

/// 检查全局嵌入服务是否已初始化
pub fn is_global_embedding_service_initialized() -> bool {
    EMBEDDING_SERVICE.get().is_some()
}

#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Qwen3-Embedding-0.6B 模型蒸馏为 Model2Vec 格式的脚本

此脚本将 Qwen3-Embedding-0.6B 模型蒸馏为更小、更快的 Model2Vec 静态嵌入模型。
Model2Vec 通过移除 Transformer 的注意力机制，仅保留词嵌入层来实现加速。
"""

import os
import torch
import numpy as np
from model2vec.distill import distill
from model2vec import StaticModel
import json
from pathlib import Path
import logging
from typing import Optional

# 设置日志
logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(levelname)s - %(message)s')
logger = logging.getLogger(__name__)

class Qwen3ToModel2VecDistiller:
    """
    Qwen3-Embedding-0.6B 到 Model2Vec 的蒸馏器
    """
    
    def __init__(self, 
                 model_name: str = "Qwen/Qwen3-Embedding-0.6B",
                 output_dir: str = "./qwen3-model2vec",
                 pca_dims: Optional[int] = 256):
        """
        初始化蒸馏器
        
        Args:
            model_name: 源模型名称
            output_dir: 输出目录
            pca_dims: PCA降维维度
        """
        self.model_name = model_name
        self.output_dir = Path(output_dir)
        self.pca_dims = pca_dims
        
        # 创建输出目录
        self.output_dir.mkdir(parents=True, exist_ok=True)
        
        logger.info(f"源模型: {self.model_name}")
        logger.info(f"输出目录: {self.output_dir}")
        logger.info(f"PCA维度: {self.pca_dims}")
    
    def distill_model(self) -> StaticModel:
        """
        执行模型蒸馏
        
        Returns:
            蒸馏后的 StaticModel
        """
        logger.info("开始从HuggingFace蒸馏Qwen3模型...")
        
        try:
            # 使用 model2vec.distill.distill 直接蒸馏模型
            # 检测GPU可用性
            import torch
            device = "cuda" if torch.cuda.is_available() else "cpu"
            logger.info(f"使用设备: {device}")
            
            static_model = distill(
                model_name=self.model_name,
                pca_dims=self.pca_dims,
                device=device
            )
            
            logger.info("模型蒸馏完成")
            # 获取嵌入维度
            test_embedding = static_model.encode(["test"])
            embedding_dim = test_embedding.shape[1]
            logger.info(f"蒸馏后模型嵌入维度: {embedding_dim}")
            return static_model
            
        except Exception as e:
            logger.error(f"模型蒸馏失败: {e}")
            raise
    
    def save_model(self, static_model: StaticModel):
        """
        保存蒸馏后的模型
        
        Args:
            static_model: 蒸馏后的模型
        """
        logger.info(f"保存模型到: {self.output_dir}")
        
        try:
            # 保存模型
            static_model.save_pretrained(str(self.output_dir))
            
            # 获取嵌入维度
            test_embedding = static_model.encode(["test"])
            embedding_dim = test_embedding.shape[1]
            
            # 创建简单的配置文件
            config = {
                "source_model": self.model_name,
                "embedding_dim": embedding_dim,
                "pca_dims": self.pca_dims,
                "model_type": "Model2Vec"
            }
            
            config_path = self.output_dir / "config.json"
            with open(config_path, 'w', encoding='utf-8') as f:
                json.dump(config, f, indent=2, ensure_ascii=False)
            
            logger.info(f"模型保存完成: {self.output_dir}")
            logger.info(f"配置文件: {config_path}")
            
        except Exception as e:
            logger.error(f"保存模型失败: {e}")
            raise
    
    def test_model(self, static_model: StaticModel):
        """
        测试蒸馏后的模型
        
        Args:
            static_model: 蒸馏后的模型
        """
        logger.info("测试蒸馏后的模型...")
        
        test_texts = [
            "人工智能技术发展迅速",
            "Machine learning is powerful",
            "深度学习神经网络"
        ]
        
        try:
            embeddings = static_model.encode(test_texts)
            
            logger.info(f"测试文本数量: {len(test_texts)}")
            logger.info(f"嵌入维度: {embeddings.shape}")
            logger.info(f"嵌入样例 (前5维): {embeddings[0][:5]}")
            
            # 计算相似度
            similarity = np.dot(embeddings[0], embeddings[2]) / (
                np.linalg.norm(embeddings[0]) * np.linalg.norm(embeddings[2])
            )
            logger.info(f"中文文本相似度: {similarity:.4f}")
            
        except Exception as e:
            logger.error(f"模型测试失败: {e}")
            raise
    
    def run_distillation(self, test_model: bool = True):
        """
        运行完整的蒸馏流程
        
        Args:
            test_model: 是否测试模型
        """
        logger.info("开始 Qwen3 到 Model2Vec 蒸馏流程")
        
        try:
            # 1. 执行蒸馏
            static_model = self.distill_model()
            
            # 2. 保存模型
            self.save_model(static_model)
            
            # 3. 测试模型（可选）
            if test_model:
                self.test_model(static_model)
            
            logger.info("蒸馏流程完成！")
            logger.info(f"模型已保存到: {self.output_dir}")
            
            return static_model
            
        except Exception as e:
            logger.error(f"蒸馏流程失败: {e}")
            raise

def main():
    """
    主函数
    """
    # 配置参数
    config = {
        "model_name": "Qwen/Qwen3-Embedding-0.6B",
        "output_dir": "./qwen3-model2vec",
        "pca_dims": 256,       # PCA降维维度
        "test_model": True     # 是否测试模型
    }
    
    print("=" * 60)
    print("Qwen3-Embedding-0.6B 到 Model2Vec 蒸馏工具")
    print("=" * 60)
    print(f"源模型: {config['model_name']}")
    print(f"输出目录: {config['output_dir']}")
    print(f"PCA维度: {config['pca_dims']}")
    print("=" * 60)
    
    try:
        # 创建蒸馏器
        distiller = Qwen3ToModel2VecDistiller(
            model_name=config["model_name"],
            output_dir=config["output_dir"],
            pca_dims=config["pca_dims"]
        )
        
        # 运行蒸馏
        static_model = distiller.run_distillation(
            test_model=config["test_model"]
        )
        
        print("\n" + "=" * 60)
        print("蒸馏完成！")
        print(f"模型保存位置: {config['output_dir']}")
        print("\n使用方法:")
        print("from model2vec import StaticModel")
        print(f"model = StaticModel.from_pretrained('{config['output_dir']}')")
        print("embeddings = model.encode(['你的文本'])")
        print("=" * 60)
        
    except Exception as e:
        logger.error(f"程序执行失败: {e}")
        print(f"\n错误: {e}")
        print("\n请检查:")
        print("1. 是否安装了所需依赖: pip install model2vec[distill]")
        print("2. 是否有足够的内存和存储空间")
        print("3. 网络连接是否正常（下载模型需要）")

if __name__ == "__main__":
    main()
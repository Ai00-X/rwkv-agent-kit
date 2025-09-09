---
home: true
config:
  - type: hero
    full: true
    background: tint-plate
    tintPlate: 210
    hero:
      name: RWKV Agent Kit
      tagline: <span lang="zh">基于 RWKV 的<br>智能体开发框架</span>
      text: <span lang="zh">为 AI 智能体提供真正的记忆和思考能力</span>
      actions:
        - theme: brand
          text: 快速开始 →
          link: /guide/
        - theme: alt
          text: GitHub
          link: https://github.com/Ai00-X/rwkv-agent-kit
  - type: features
    features:
      - title: 💡 简单易用
        details: 简洁的 API 设计，几行代码即可创建智能代理，支持快速原型开发和生产部署
      - title: 🧠 真正的记忆
        details: 基于向量数据库的长期记忆系统，让 AI 智能体能够记住和学习历史对话内容
      - title: ⚡ 高性能
        details: 基于 RWKV 模型的高效推理引擎，支持 CPU 推理，毫秒级响应速度
      - title: 🔧 可扩展
        details: 灵活的工具系统和插件架构，轻松集成各种外部服务和功能模块
  - type: features
    title: 🌟 核心特性
    features:
      - title: 🤖 RWKV LLM 集成
        details: 原生支持RWKV模型，智能状态管理，多模态输入，流式输出，优化推理性能
      - title: 👥 多智能体系统
        details: 智能体编排协同工作，角色定制，通信机制，负载均衡，智能任务分配
      - title: 🧠 智能记忆系统
        details: 短期记忆动态管理，长期记忆持久化存储，语义片段聚合，图谱存储，个人画像
      - title: 🔍 向量检索能力
        details: Model2Vec集成，语义搜索，混合检索，性能优化，查询优化器和缓存机制
      - title: 🛠️ 工具生态系统
        details: 工具注册表，共享工具，扩展接口，错误处理，跨智能体工具共享机制
      - title: 🗄️ 数据存储
        details: SQLite数据库，统一接口，数据迁移，性能监控，轻量级但功能强大的本地存储
---

## 开始使用

RWKV Agent Kit 是一个基于 RWKV 模型的智能代理工具包，专注于为 AI 智能体提供真正的记忆和思考能力。

### 安装

```bash
cargo add rwkv-agent-kit
```

### 快速开始

```rust
use rwkv_agent_kit::RwkvAgentKit;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let agent = RwkvAgentKit::new("config.toml").await?;
    
    let response = agent.chat("你好，请介绍一下你自己").await?;
    println!("AI: {}", response);
    
    Ok(())
}
```

## 为什么选择 RWKV Agent Kit？

- **简单易用** - 简洁的 API 设计，几行代码即可创建智能代理
- **真正的记忆** - 基于向量数据库的长期记忆系统
- **高性能** - 基于 RWKV 模型，支持 CPU 推理，毫秒级响应
- **可扩展** - 灵活的工具系统和插件架构

## 了解更多

- [快速开始](/guide/) - 学习如何安装和使用
- [API 文档](/api/) - 详细的 API 参考
- [配置说明](/config/) - 配置选项和最佳实践
- [GitHub](https://github.com/Ai00-X/rwkv-agent-kit) - 查看源代码和贡献
---
title: installation
createTime: 2025/09/08 13:16:22
permalink: /article/a0tjfa25/
---
# 安装指南

## 环境要求

- Rust 1.70.0 或更高版本
- Cargo 包管理器

## 从 crates.io 安装

在您的 `Cargo.toml` 文件中添加以下依赖：

```toml
[dependencies]
rwkv-agent-kit = "0.1.1"
```

然后运行：

```bash
cargo build
```

## 从源码安装

1. 克隆仓库：

```bash
git clone https://github.com/Ai00-X/rwkv-agent-kit.git
cd rwkv-agent-kit
```

2. 构建项目：

```bash
cargo build --release
```

## 验证安装

创建一个简单的测试文件来验证安装：

```rust
use rwkv_agent_kit::RwkvAgentKit;

fn main() {
    println!("RWKV Agent Kit 安装成功！");
}
```

运行测试：

```bash
cargo run
```

如果看到输出信息，说明安装成功！
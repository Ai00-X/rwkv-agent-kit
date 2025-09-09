### 自动发布到 crates.io

本项目配置了 GitHub Actions 自动发布工作流。当创建新的版本标签时，会自动运行测试、格式检查、代码质量检查，并发布到 crates.io。

#### 设置 CARGO_REGISTRY_TOKEN

要启用自动发布功能，需要在 GitHub 仓库中设置 `CARGO_REGISTRY_TOKEN` 密钥：

1. **获取 crates.io API Token**
   - 访问 [crates.io](https://crates.io/)
   - 登录你的账户
   - 进入 [Account Settings](https://crates.io/settings/tokens)
   - 点击 "New Token" 创建新的 API Token
   - 复制生成的 Token（格式类似：`cio_xxxxxxxxxx`）

2. **在 GitHub 仓库中设置密钥**
   - 进入你的 GitHub 仓库
   - 点击 "Settings" 标签
   - 在左侧菜单中选择 "Secrets and variables" → "Actions"
   - 点击 "New repository secret"
   - Name: `CARGO_REGISTRY_TOKEN`
   - Secret: 粘贴你的 crates.io API Token
   - 点击 "Add secret"

3. **创建版本标签触发发布**
   ```bash
   # 更新版本号（在 Cargo.toml 中）
   git add Cargo.toml
   git commit -m "Bump version to 0.1.2"
   
   # 创建并推送标签
   git tag v0.1.2
   git push origin v0.1.2
   ```

4. **工作流功能**
   - 🧪 **多版本测试**: 在 stable、beta、nightly Rust 版本上运行测试
   - 📝 **格式检查**: 确保代码符合 Rust 格式规范
   - 🔍 **代码质量**: 运行 clippy 检查代码质量
   - 📦 **自动发布**: 测试通过后自动发布到 crates.io
   - 🏷️ **GitHub Release**: 自动创建 GitHub Release 页面

#### 工作流文件位置

自动发布工作流配置文件位于：`.github/workflows/release.yml`

如需自定义工作流，可以编辑此文件来调整触发条件、测试步骤或发布流程。
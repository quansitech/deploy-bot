## 1. 配置修改

- [x] 1.1 在 `src/config/mod.rs` 的 `ServerConfig` 中添加 `github_mirror: Option<String>` 字段
- [x] 1.2 在 `ServerConfig` 中添加 `is_github_mirror_configured()` 方法
- [x] 1.3 在 `config.yaml` 中添加示例配置注释

## 2. 下载逻辑修改

- [x] 2.1 修改 `src/self_update.rs` 中 `download_binary` 函数签名，添加 `github_mirror` 参数
- [x] 2.2 在 `download_binary` 中实现镜像 URL 转换逻辑
- [x] 2.3 修改 `handle_self_update` 函数，从 config 获取镜像配置并传入 `download_binary`

## 3. 测试

- [x] 3.1 添加 `github_mirror` 配置相关的单元测试
- [x] 3.2 运行 `cargo clippy -- -D warnings` 检查代码
- [x] 3.3 运行 `cargo test` 确保测试通过

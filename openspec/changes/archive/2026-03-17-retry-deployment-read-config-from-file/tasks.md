## 1. 修改 DeploymentManager 结构

- [x] 1.1 在 `DeploymentManager` 结构体中增加 `workspace_dir: String` 字段
- [x] 1.2 修改 `DeploymentManager::new()` 方法签名，增加 `workspace_dir: String` 参数
- [x] 1.3 在 `DeploymentManager::new()` 方法体中保存 `workspace_dir`

## 2. 修改 retry_deployment 方法

- [x] 2.1 从数据库获取 `project_name`（从现有 deployment 记录中读取）
- [x] 2.2 拼接配置文件路径：`{workspace_dir}/{project_name}/.deploy.yaml`
- [x] 2.3 调用 `ProjectConfig::load_from_file()` 读取最新配置
- [x] 2.4 使用新配置创建新的 `Deployment` 并加入队列
- [x] 2.5 处理配置读取失败的情况（文件不存在等），返回 false

## 3. 更新调用方

- [x] 3.1 找到 `main.rs` 中创建 `DeploymentManager` 的位置
- [x] 3.2 传入 `config.server.workspace_dir` 作为参数

## 4. 验证

- [x] 4.1 运行 `cargo clippy -- -D warnings` 检查代码
- [x] 4.2 运行 `cargo test` 确保所有测试通过
- [ ] 4.3 手动测试：修改 `.deploy.yaml` 后通过 Web UI 重试，验证使用了新配置

## Context

当前系统架构：
1. Webhook 触发部署时，从 `.deploy.yaml` 读取配置，存入数据库 `deployments` 表
2. Web UI 点击"重试"时，从数据库读取完整的 `Deployment` 对象（包括 `ProjectConfig`）
3. 数据库中存储的是配置的快照，而非动态引用

问题：`retry_deployment` 方法复用数据库中的 `deployment.project`，导致配置无法更新。

## Goals / Non-Goals

**Goals:**
- 重试部署时始终使用最新的 `.deploy.yaml` 配置
- 不需要用户手动更新配置或重新创建部署任务
- 保持向后兼容，不影响现有功能

**Non-Goals:**
- 不修改数据库结构
- 不改变首次部署的流程（仍然从文件读取并存入数据库）
- 不添加新的 API 或 Web UI 改动

## Decisions

### 方案 C: Manager 持有 workspace_dir

在 `DeploymentManager` 中增加 `workspace_dir` 字段，初始化时从 Config 传入。重试时根据 `project_name` 拼接配置文件路径，重新读取配置。

**优点：**
- Web UI 调用无需改动（`retry_deployment(id)` 不需要额外参数）
- 职责集中，配置管理逻辑在 Manager 内部

**替代方案：**
- 方案 A（透传 workspace_dir）：调用方需要知道 workspace_dir，改动大
- 方案 B（存 config_path）：需要改数据库结构

## Risks / Trade-offs

| 风险 | 缓解措施 |
|------|----------|
| 配置文件被删除导致重试失败 | 保持现有错误处理，配置读取失败时返回 false |
| workspace_dir 变更导致路径错误 | 配置在初始化时固定，重启服务后生效 |
| 旧部署记录没有 project_name | 从数据库读取 project_name 作为标识 |

## Migration Plan

1. 修改 `DeploymentManager::new()` 签名，增加 `workspace_dir: String` 参数
2. 修改 `retry_deployment()` 方法：
   - 从数据库获取 `project_name`
   - 拼接配置文件路径 `{workspace_dir}/{project_name}/.deploy.yaml`
   - 调用 `ProjectConfig::load_from_file()` 读取新配置
   - 用新配置创建新的 `Deployment` 入队
3. 更新 `main.rs` 中创建 `DeploymentManager` 的调用，传入 `config.server.workspace_dir`
4. 运行 `cargo clippy` 和 `cargo test` 验证

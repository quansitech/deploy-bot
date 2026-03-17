## Why

当前 Web UI 触发的重试任务使用的是数据库中存储的配置快照。如果首次部署时记录了错误的 `.deploy.yaml` 配置，后续修改了配置文件，重试任务仍然会使用旧的配置，导致部署失败或行为不符合预期。应该让重试任务始终读取最新的配置文件。

## What Changes

- 修改 `DeploymentManager` 增加 `workspace_dir` 字段
- 重写 `retry_deployment` 方法，从配置文件重新读取 `ProjectConfig`，而不是使用数据库中的旧数据
- 更新 `DeploymentManager::new()` 的调用处，传入 `workspace_dir` 参数

## Capabilities

### New Capabilities
- `retry-deployment-config-reload`: 重试部署时自动从最新的 `.deploy.yaml` 读取配置

### Modified Capabilities
- `deployment-persistence`: 需要补充说明：数据库仅存储任务元数据（id、status、时间戳），配置信息不在数据库中持久化，而是每次从文件动态读取

## Impact

- **代码改动**: `src/deploy/manager.rs` - 修改 `DeploymentManager` 结构和 `retry_deployment` 方法
- **调用方改动**: `src/main.rs` 等创建 `DeploymentManager` 的地方需要传入 `workspace_dir`
- **无数据库结构变更**: 不需要迁移

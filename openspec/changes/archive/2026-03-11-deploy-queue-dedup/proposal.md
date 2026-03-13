## Why

当前 webhook 触发部署时，每次请求都会向部署队列添加一个新任务。如果短时间内收到多个 webhook 请求（例如连续多次 push），会导致同一项目重复部署，浪费资源且可能造成部署冲突。

## What Changes

- 修改 `DeploymentManager::queue_deployment()` 方法，添加去重逻辑
- 仅当队列中不存在相同项目（project_name + branch）的 Pending 或 Running 状态任务时才入队
- 如果已存在重复任务，返回 None，webhook handler 返回提示信息

## Capabilities

### New Capabilities
- `deploy-queue-dedup`: 部署队列去重，确保同一项目同时只有一个待执行或执行中的任务

### Modified Capabilities
- 无

## Impact

- 修改 `src/deploy/manager.rs` - 添加去重逻辑
- 修改 `src/webhook/handler.rs` - 处理去重后的返回

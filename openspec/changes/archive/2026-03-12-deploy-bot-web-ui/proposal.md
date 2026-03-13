## Why

目前 deploy-bot 只提供了 webhook 触发部署的功能，缺少可视化的管理界面。运维人员无法直观查看部署任务列表、实时日志，也无法管理未执行或失败的任务。亟需一个简洁的 Web 管理界面来提升运维效率。

## What Changes

1. 新增 SQLite 数据库持久化部署任务和日志
2. 新增 Web 管理界面（纯 SSR，Axum 渲染 HTML）
3. 新增任务列表页：展示所有部署任务及状态
4. 新增任务详情页：展示任务详细信息和实时日志
5. 新增 WebSocket 实时日志推送
6. 支持删除 Pending 状态的任务
7. 支持重试 Failed 状态的任务
8. 新增 RESTful 路由和 WebSocket 路由

## Capabilities

### New Capabilities

- `web-ui`: Web 管理界面功能，包括任务列表、详情、日志展示和任务操作
- `deployment-persistence`: 部署任务和日志的 SQLite 持久化
- `deployment-logs`: 部署日志的实时推送（WebSocket）

### Modified Capabilities

（无）

## Impact

- 新增依赖：`rusqlite`（SQLite 驱动）、`tokio-tungstenite`（WebSocket）
- 新增路由：`GET /`, `GET /deploy/:id`, `WS /ws/deploy/:id`, `POST /deploy/:id/delete`, `POST /deploy/:id/retry`
- 数据库：新增 `deployments` 和 `deployment_logs` 表

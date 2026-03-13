## Context

deploy-bot 是一个基于 Rust + Axum 的自动化部署服务，当前仅支持 webhook 触发部署，部署任务存储在内存队列中，重启后数据丢失。日志通过 tracing-appender 写入文件，无结构化查询能力。

## Goals / Non-Goals

**Goals:**
- 为 deploy-bot 添加 Web 管理界面
- 实现部署任务的 SQLite 持久化
- 支持实时日志查看（WebSocket）
- 支持删除 Pending 任务和重试 Failed 任务

**Non-Goals:**
- 用户认证（内部工具，无需认证）
- 手动触发部署（通过 webhook 触发即可）
- 复杂的前端框架（纯 SSR + 简约 HTML）

## Decisions

### 1. 数据库选型：SQLite

**选择理由：**
- 轻量级，无需额外服务进程
- 足够满足单机部署工具的数据存储需求
- Rust 有成熟的 `rusqlite` 绑定

**替代方案考虑：**
- PostgreSQL：需要额外服务，不适合单机工具场景
- 内存 + 文件持久化：不如 SQLite 结构化查询方便

### 2. 前端渲染：纯 SSR（Axum + askama）

**选择理由：**
- 保持技术栈统一（纯 Rust）
- 简单高效，无需构建前端资源
- 适合内部工具的简约需求

**替代方案考虑：**
- 前后端分离 + React/Vue：增加复杂度，无需
- Leptos/Yew 全栈：学习成本高

### 3. 实时日志：tokio-tungstenite

**选择理由：**
- 与 Axum/Tokio 生态兼容
- 支持异步非阻塞日志推送

**替代方案考虑：**
- SSE（Server-Sent Events）：WebSocket 更通用
- 轮询：实时性差，增加服务端负担

### 4. 模板引擎：askama

**选择理由：**
- 类型安全，编译期检查
- 与 Axum 集成良好
- 性能好，无运行时模板解析

### 5. 持久化策略

**方案：**
- 部署任务创建时写入 SQLite
- 状态变更时更新 SQLite
- 日志实时写入 SQLite（批量写入优化）

**与内存队列的关系：**
- SQLite 作为持久化存储
- 内存队列作为运行时队列
- 启动时从 SQLite 恢复队列（如需要）

## Risks / Trade-offs

| 风险 | 影响 | 缓解措施 |
|------|------|----------|
| SQLite 并发写入 | 高频部署时写入性能 | 批量写入、写入缓冲 |
| WebSocket 连接数 | 大量并发连接 | 限制单用户连接数 |
| 日志数据膨胀 | 磁盘空间增长 | 定期清理历史日志（如保留 30 天） |

## Migration Plan

1. 添加依赖：rusqlite, tokio-tungstenite, askama
2. 创建数据库模块：初始化表结构
3. 重构 DeploymentManager：从纯内存改为 SQLite 持久化
4. 添加 Web 路由：HTML 页面和 API
5. 添加 WebSocket 路由：实时日志
6. 测试验证

## Open Questions

1. **日志保留策略**：是否需要自动清理历史日志？建议保留 30 天。
2. **启动恢复**：服务重启时是否需要从 SQLite 恢复未完成的任务？暂不考虑，保持简单。

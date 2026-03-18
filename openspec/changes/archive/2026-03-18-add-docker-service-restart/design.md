## Context

当前部署流程在 Docker 环境中执行依赖安装时，使用 `docker compose run --rm` 创建临时容器。临时容器退出后，其内部安装的依赖不会持久化到实际运行的服务容器中，导致新依赖无法生效。

用户需要在 `.deploy.yaml` 中配置服务重启，使部署流程自动化完成。

## Goals / Non-Goals

**Goals:**
- 支持在 `.deploy.yaml` 中配置需要重启的 Docker 服务
- 支持单服务和多服务（数组）配置
- 在部署流程最后串行执行服务重启
- 失败时立即标记部署失败并输出日志

**Non-Goals:**
- 不支持并行重启服务
- 不支持复杂的重启策略（如健康检查等待）
- 不支持服务依赖顺序自动解析

## Decisions

### 1. 配置字段设计

**方案A：** 使用 TOML 数组
```toml
restart_service = ["web", "worker"]
```

**方案B：** 使用逗号分隔字符串
```toml
restart_service = "web,worker"
```

**决策：** 方案A (TOML 数组)

**理由：** TOML 原生支持数组，类型安全，用户体验更好。单个字符串也能通过 TOML 解析为单元素数组。

### 2. 执行时机

**方案A：** 在 install_dependencies 之后立即重启

**方案B：** 在所有部署步骤完成后执行

**决策：** 方案B

**理由：** 更符合用户心智模型——先完成所有部署工作，最后统一重启服务。

### 3. 重启命令

使用 `docker compose restart <service>` 而非 `docker compose rm && docker compose up -d`，因为：
- restart 更轻量，不涉及容器重建
- 保留容器的网络挂载等配置
- 失败时更容易排查问题

## Risks / Trade-offs

| 风险 | 影响 |  Mitigation |
|------|------|-------------|
| 重启服务导致短暂服务中断 | 业务影响 | 用户在低峰期部署，或使用滚动更新 |
| 服务重启失败 | 部署失败 | 立即报错，用户可手动处理 |
| 多服务依赖顺序错误 | 服务启动失败 | 要求用户按正确顺序配置 |

## Migration Plan

1. 部署新版本代码（包含重启功能）
2. 用户在 `.deploy.yaml` 中添加 `restart_service` 配置
3. 下次部署时自动触发服务重启

无需回滚策略——如果不需要自动重启，不配置该字段即可。

## Open Questions

无

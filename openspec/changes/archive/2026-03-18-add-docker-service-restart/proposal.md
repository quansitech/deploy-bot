## Why

在 Docker 环境中部署 Python（或其他语言）项目时，依赖安装通常在临时容器中执行。临时容器退出后，新安装的依赖不会自动应用到实际运行的服务容器中，导致部署完成后服务仍在使用旧依赖。这需要手动执行 `docker compose restart` 才能让新依赖生效，流程不够自动化。

## What Changes

- 在 `.deploy.yaml` 配置中新增 `restart_service` 字段
- 支持两种配置形式：
  - 单服务：`restart_service = "web"`
  - 多服务：`restart_service = ["web", "worker"]`
- 在部署流程的最后阶段（所有步骤完成后）串行执行服务重启
- 重启失败时立即标记部署失败，并在日志中输出错误信息
- 重启成功时在日志中输出成功信息

## Capabilities

### New Capabilities
- `docker-service-restart`: 在部署完成后自动重启指定的 Docker 服务

### Modified Capabilities
- 无

## Impact

- **代码影响**：
  - `src/project_config/mod.rs` - 添加 `restart_service` 字段解析
  - `src/deploy/executor.rs` - 添加服务重启步骤
- **配置影响**：
  - `.deploy.yaml` 文件支持新字段
- **行为影响**：
  - 部署流程增加服务重启步骤

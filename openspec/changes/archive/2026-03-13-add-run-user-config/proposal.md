## Why

当前部署系统在执行命令时使用固定的运行用户，无法根据项目需求指定特定用户（如 www-data、nginx 等）。这导致某些需要特定用户权限的项目无法正确部署。

## What Changes

- 在 `.deploy.yaml` 配置文件中新增 `run_user` 字段
- 支持在非 Docker 环境下使用 `sudo -u <user>` 指定运行用户
- 支持在 Docker 环境下使用 `docker compose run --user <uid>:<gid>` 指定运行用户
- 用户未指定时使用当前进程用户（deploy-bot 运行用户）
- 用户不存在或无权限时返回明确错误信息

## Capabilities

### New Capabilities

- `run-user-config`: 支持在部署配置中指定命令运行用户
  - 解析 `.deploy.yaml` 中的 `run_user` 字段
  - 在非 Docker 环境使用 `sudo -u <user>` 执行命令
  - 在 Docker 环境使用 `--user` 参数指定用户
  - 用户不存在或无权限时返回明确错误

### Modified Capabilities

（无现有需求变更）

## Impact

- **影响的代码模块**：
  - `src/project_config/mod.rs` - 添加 `run_user` 字段解析
  - `src/installer/tasks.rs` - 命令执行层支持用户切换
  - `src/runner/task.rs` - 透传 `run_user` 参数

- **配置文件**：
  - `.deploy.yaml` 新增 `run_user` 可选字段

- **影响范围**：
  - `install_command`
  - `build_command`
  - `extra_command`

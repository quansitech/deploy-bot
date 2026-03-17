## Why

部分旧服务器的 Docker 版本较低，没有 `docker compose` 子命令，只有独立的 `docker-compose` 命令。当前代码硬编码使用 `docker compose`，导致在这些旧服务器上无法使用 Docker 方式部署。需要在程序启动时自动检测可用的 Docker Compose 命令。

## What Changes

- 在 `ServerConfig` 中添加 `docker_compose_command` 字段记录检测结果（`Option<DockerComposeCommand>`）
- 在程序启动时（加载配置后）检测 Docker Compose 命令可用性
- 仅当 `docker_compose_path` 有值时才进行检测
- 修改 `installer/tasks.rs` 中的命令执行逻辑，使用检测结果而非硬编码

## Capabilities

### New Capabilities
- `docker-compose-compatibility`: 自动检测并使用可用的 Docker Compose 命令（`docker compose` 或 `docker-compose`）

### Modified Capabilities
- 无

## Impact

- 修改 `src/config/mod.rs`：添加检测结果字段
- 修改 `src/main.rs`：启动时执行检测
- 修改 `src/installer/tasks.rs`：使用检测结果执行命令

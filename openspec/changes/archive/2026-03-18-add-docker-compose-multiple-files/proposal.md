## Why

当前 config.yaml 只支持单个 docker_compose_path，无法满足多配置文件场景（例如基础配置 + 环境覆盖配置）。同时 .deploy.yaml 也没有 docker_compose_path 字段，无法在项目级别覆盖全局配置。

## What Changes

- 在 `config.yaml` 中将 `docker_compose_path` 从 `Option<String>` 改为支持数组 `Option<Vec<String>>`，使用 serde untagged 兼容单字符串写法
- 在 `.deploy.yaml` 中新增 `docker_compose_path` 字段，支持字符串或数组格式
- `.deploy.yaml` 的 `docker_compose_path` 会覆盖 `config.yaml` 的配置
- 修改 `run_docker_compose` 函数，支持生成多个 `-f` 参数
- 修改 `restart_docker_services` 函数，支持多个配置文件

## Capabilities

### New Capabilities

- **multiple-docker-compose-files**: 支持在 config.yaml 和 .deploy.yaml 中配置多个 docker compose 配置文件路径，生成的命令会自动添加多个 `-f` 参数

### Modified Capabilities

- 无（现有功能的行为不变，只是扩展了配置格式）

## Impact

- `src/config/mod.rs`: ServerConfig.docker_compose_path 类型变更
- `src/project_config/mod.rs`: 新增 ProjectConfig.docker_compose_path 字段
- `src/installer/tasks.rs`: run_docker_compose 函数支持多文件
- `src/deploy/executor.rs`: restart_docker_services 函数支持多文件
- `src/runner/task.rs`: 相关函数签名调整

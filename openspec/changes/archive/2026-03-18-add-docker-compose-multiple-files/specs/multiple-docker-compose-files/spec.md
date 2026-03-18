## ADDED Requirements

### Requirement: 支持多个 docker compose 配置文件

系统 SHALL 支持在 config.yaml 和 .deploy.yaml 中配置多个 docker compose 配置文件路径。当配置多个文件时，生成的 docker compose 命令 SHALL 包含多个 `-f` 参数。

#### Scenario: config.yaml 使用单个字符串配置

- **WHEN** config.yaml 中设置 `docker_compose_path = "/path/docker-compose.yaml"`
- **THEN** docker compose 命令使用单个 `-f` 参数

#### Scenario: config.yaml 使用数组配置多个文件

- **WHEN** config.yaml 中设置 `docker_compose_path = ["/path/base.yaml", "/path/override.yaml"]`
- **THEN** docker compose 命令使用 `-f /path/base.yaml -f /path/override.yaml`

#### Scenario: .deploy.yaml 覆盖 config.yaml 配置

- **WHEN** .deploy.yaml 中设置 `docker_compose_path = ["/project/compose.yaml"]`，同时 config.yaml 设置了不同值
- **THEN** 使用 .deploy.yaml 的配置，忽略 config.yaml 的值

#### Scenario: .deploy.yaml 未配置时使用 config.yaml

- **WHEN** .deploy.yaml 未设置 docker_compose_path
- **THEN** 使用 config.yaml 的配置

#### Scenario: 未配置 docker_compose_path 时不生成 -f 参数

- **WHEN** config.yaml 和 .deploy.yaml 都未配置 docker_compose_path
- **THEN** 不使用 docker compose 方式执行命令（现有行为不变）

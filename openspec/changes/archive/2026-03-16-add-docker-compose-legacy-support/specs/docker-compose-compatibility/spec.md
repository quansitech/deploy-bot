## ADDED Requirements

### Requirement: 自动检测 Docker Compose 命令
当 `docker_compose_path` 配置有值时，系统 SHALL 在程序启动时自动检测可用的 Docker Compose 命令。

#### Scenario: 服务器支持 docker compose 子命令
- **GIVEN** 服务器安装的 Docker 版本支持 `docker compose` 子命令
- **WHEN** 程序启动且 `docker_compose_path` 有值
- **THEN** 系统检测到 `docker compose version` 执行成功
- **THEN** 系统使用 `docker compose` 执行后续命令

#### Scenario: 服务器仅支持 docker-compose 独立命令
- **GIVEN** 服务器安装的 Docker 版本较旧，仅支持 `docker-compose` 独立命令
- **WHEN** 程序启动且 `docker_compose_path` 有值
- **AND** `docker compose version` 执行失败
- **THEN** 系统检测到 `docker-compose --version` 执行成功
- **THEN** 系统使用 `docker-compose` 执行后续命令

#### Scenario: docker_compose_path 未配置
- **GIVEN** `docker_compose_path` 配置为 None
- **WHEN** 程序启动
- **THEN** 系统不执行 Docker Compose 命令检测
- **THEN** 任务在宿主机直接执行

### Requirement: 使用检测结果执行 Docker Compose 命令
系统 SHALL 根据检测结果使用正确的命令执行 Docker Compose 操作。

#### Scenario: 使用 docker compose 执行
- **GIVEN** 检测结果为 DockerCompose
- **WHEN** 执行 Docker Compose 命令时
- **THEN** 系统使用 `docker compose -f <path> run --rm <service> <command>`

#### Scenario: 使用 docker-compose 执行
- **GIVEN** 检测结果为 DockerComposeLegacy
- **WHEN** 执行 Docker Compose 命令时
- **THEN** 系统使用 `docker-compose -f <path> run --rm <service> <command>`

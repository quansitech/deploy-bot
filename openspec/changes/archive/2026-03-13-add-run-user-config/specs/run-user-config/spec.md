## ADDED Requirements

### Requirement: run_user 配置解析

系统 SHALL 支持从 `.deploy.yaml` 配置文件中解析 `run_user` 字段，该字段为可选字段。

#### Scenario: 配置文件包含 run_user
- **WHEN** `.deploy.yaml` 中存在 `run_user` 字段
- **THEN** 系统解析该字段并传递给命令执行层

#### Scenario: 配置文件不包含 run_user
- **WHEN** `.deploy.yaml` 中不存在 `run_user` 字段
- **THEN** 系统使用当前进程用户作为默认用户

### Requirement: 非 Docker 环境用户切换

在非 Docker 环境下执行命令时，系统 SHALL 使用 `sudo -u <user>` 切换到指定用户。

#### Scenario: 使用指定用户执行命令
- **WHEN** 配置了 `run_user` 且不在 Docker 环境
- **THEN** 系统使用 `sudo -u <username> <command>` 格式执行命令

#### Scenario: 使用默认用户执行命令
- **WHEN** 未配置 `run_user` 且不在 Docker 环境
- **THEN** 系统使用当前进程用户执行命令

#### Scenario: 指定用户不存在
- **WHEN** 配置的 `run_user` 对应的用户在系统中不存在
- **THEN** 系统返回错误信息："User '<username>' does not exist"

#### Scenario: 无 sudo 权限
- **WHEN** 当前用户没有 sudo 权限切换到指定用户
- **THEN** 系统返回错误信息："Permission denied: cannot run as user '<username>'"

### Requirement: Docker 环境用户切换

在 Docker 环境下执行命令时，系统 SHALL 使用 `docker compose run --user <uid>:<gid>` 参数切换用户。

#### Scenario: Docker 环境使用指定用户
- **WHEN** 配置了 `run_user` 且在 Docker 环境
- **THEN** 系统将用户名转换为 UID:GID，并使用 `--user` 参数执行 docker compose

#### Scenario: Docker 环境用户名转换
- **WHEN** Docker 环境需要切换到指定用户
- **THEN** 系统在宿主机上使用 `id -u <username>` 获取 UID，`id -g <username>` 获取 GID，生成 `--user <uid>:<gid>` 参数

#### Scenario: 宿主机用户不存在
- **WHEN** 在宿主机上指定的 `run_user` 用户不存在
- **THEN** 系统返回错误信息："User '<username>' does not exist on host"

### Requirement: 用户配置应用于所有命令

`run_user` 配置 SHALL 应用于 git 操作、install_command、build_command 和 extra_command。

#### Scenario: Git clone 使用 run_user
- **WHEN** 配置了 `run_user` 且需要 clone 仓库
- **THEN** Git clone 以指定用户执行，避免文件权限问题

#### Scenario: Git fetch/pull 使用 run_user
- **WHEN** 配置了 `run_user` 且需要更新已有仓库
- **THEN** Git fetch/checkout/pull 以指定用户执行，避免 dubious ownership 错误

#### Scenario: install_command 使用 run_user
- **WHEN** 配置了 `run_user` 且存在 `install_command`
- **THEN** `install_command` 以指定用户执行

#### Scenario: build_command 使用 run_user
- **WHEN** 配置了 `run_user` 且存在 `build_command`
- **THEN** `build_command` 以指定用户执行

#### Scenario: extra_command 使用 run_user
- **WHEN** 配置了 `run_user` 且存在 `extra_command`
- **THEN** `extra_command` 以指定用户执行

### Requirement: Web UI 显示运行用户

系统 SHALL 在 Web UI 中显示当前部署任务使用的运行用户。

#### Scenario: 部署日志显示运行用户
- **WHEN** 部署任务执行命令时
- **THEN** 系统在日志中显示当前用户，格式：`[<username>] <command>`

#### Scenario: 部署详情显示配置的用户
- **WHEN** 用户查看部署任务详情时
- **THEN** 系统显示该任务配置的 `run_user` 值（如果未配置则显示默认用户）

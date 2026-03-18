## ADDED Requirements

### Requirement: Docker 服务重启配置

用户可以在 `.deploy.yaml` 中通过 `restart_service` 字段配置部署完成后需要重启的 Docker 服务。该字段支持字符串（单服务）和数组（多服务）两种形式。

#### Scenario: 配置单个服务重启
- **WHEN** 用户在 `.deploy.yaml` 中配置 `restart_service = "web"`
- **THEN** 系统在部署完成后执行 `docker compose restart web`

#### Scenario: 配置多个服务重启
- **WHEN** 用户在 `.deploy.yaml` 中配置 `restart_service = ["web", "worker"]`
- **THEN** 系统在部署完成后按顺序执行 `docker compose restart web` 和 `docker compose restart worker`

#### Scenario: 未配置重启服务
- **WHEN** 用户未在 `.deploy.yaml` 中配置 `restart_service`
- **THEN** 系统跳过服务重启步骤，部署流程保持不变

### Requirement: 服务重启执行

系统必须在部署流程的所有步骤（git pull、依赖安装、构建、额外命令）完成后，才执行服务重启。

#### Scenario: 部署成功后的服务重启
- **WHEN** 所有部署步骤成功完成，且配置了 `restart_service`
- **THEN** 系统依次对每个服务执行重启命令，并在日志中输出重启结果

#### Scenario: 服务重启失败
- **WHEN** 服务重启过程中发生错误
- **THEN** 系统立即标记部署为失败状态，并在日志中输出错误信息

#### Scenario: 前置步骤失败时跳过重启
- **WHEN** 部署流程中的任意步骤失败（如依赖安装失败）
- **THEN** 系统不执行服务重启，直接标记部署失败

### Requirement: 日志输出

服务重启操作必须在部署日志中有清晰的输出，便于用户了解部署状态。

#### Scenario: 重启成功日志
- **WHEN** 服务重启成功
- **THEN** 系统在日志中输出类似 "Restarting service: web" 和 "Service web restarted successfully"

#### Scenario: 重启失败日志
- **WHEN** 服务重启失败
- **THEN** 系统在日志中输出类似 "Failed to restart service web: <error>" 并终止部署流程

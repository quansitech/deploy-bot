## ADDED Requirements

### Requirement: Docker 容器日志实时流式输出
系统 SHALL 使用 docker logs -f 实现容器日志实时流式输出。

#### Scenario: Docker 容器启动
- **WHEN** Docker 容器启动执行命令时
- **THEN** 立即返回容器 ID，容器在后台运行

#### Scenario: 日志实时推送
- **WHEN** Docker 容器有日志输出时
- **THEN** 日志实时推送到 WebSocket 和数据库

#### Scenario: 容器执行完成
- **WHEN** Docker 容器执行完成时
- **THEN** 等待容器完全退出，返回最终状态

### Requirement: 非 Docker 环境兼容
系统 SHALL 保持非 Docker 执行路径不变。

#### Scenario: 本地命令执行
- **WHEN** 不使用 Docker 服务时
- **THEN** 保持原有的一次性返回输出方式

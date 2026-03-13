## ADDED Requirements

### Requirement: 部署日志持久化
系统 SHALL 将部署执行过程中的日志持久化到 SQLite 数据库。

#### Scenario: 记录日志
- **WHEN** 部署执行过程中产生日志时
- **THEN** 系统将日志写入 `deployment_logs` 表

#### Scenario: 查询日志
- **WHEN** 查询特定部署的日志时
- **THEN** 系统从 `deployment_logs` 表读取该部署的所有日志，按时间顺序返回

### Requirement: 日志表结构
系统 SHALL 创建 `deployment_logs` 表存储部署日志。

#### Scenario: deployment_logs 表结构
- **WHEN** 数据库初始化时
- **THEN** 系统创建包含以下字段的 deployment_logs 表：
  - id (INTEGER PRIMARY KEY AUTOINCREMENT)
  - deployment_id (TEXT NOT NULL, FK to deployments.id)
  - timestamp (TIMESTAMP NOT NULL)
  - level (TEXT NOT NULL) - info/error
  - message (TEXT NOT NULL)

### Requirement: 实时日志推送
系统 SHALL 通过 WebSocket 实时推送部署日志到客户端。

#### Scenario: WebSocket 连接
- **WHEN** 客户端连接到 `/ws/deploy/:id`
- **THEN** 系统建立 WebSocket 连接

#### Scenario: 日志推送
- **WHEN** 部署执行过程中产生新日志时
- **THEN** 系统通过 WebSocket 推送日志消息到客户端

#### Scenario: 任务完成通知
- **THEN** 当部署任务完成时（成功或失败），系统推送完成状态消息

### Requirement: 日志查询优化
系统 SHALL 支持分页查询大量日志。

#### Scenario: 分页查询日志
- **WHEN** 请求日志时指定分页参数
- **THEN** 系统返回指定范围的日志记录

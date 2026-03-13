## ADDED Requirements

### Requirement: 部署任务队列管理
系统 SHALL 管理部署任务的队列，确保任务有序执行。

#### Scenario: 新部署任务入队
- **WHEN** Webhook 触发新的部署请求
- **THEN** 创建部署任务并加入队列，返回 deployment_id

#### Scenario: 任务串行执行
- **WHEN** 队列中存在多个任务
- **THEN** 按先进先出顺序串行执行

#### Scenario: 任务状态跟踪
- **WHEN** 任务执行过程中
- **THEN** 实时更新任务状态（pending → running → success/failed）

### Requirement: 部署日志记录
系统 SHALL 记录部署过程的详细日志。

#### Scenario: 日志存储
- **WHEN** 部署任务执行时
- **THEN** 将 stdout/stderr 输出保存到日志文件

### Requirement: 部署任务取消
系统 SHALL 支持取消正在排队的部署任务。

#### Scenario: 取消待执行任务
- **WHEN** 用户请求取消 pending 状态的任务
- **THEN** 从队列中移除任务，状态更新为 cancelled

#### Scenario: 取消运行中任务
- **WHEN** 用户请求取消 running 状态的任务
- **THEN** 发送终止信号，若超时则强制终止进程

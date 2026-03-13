## ADDED Requirements

### Requirement: 部署任务持久化
系统 SHALL 将所有部署任务持久化到 SQLite 数据库，重启后数据不丢失。

#### Scenario: 创建部署任务
- **WHEN** 新部署任务被创建时
- **THEN** 系统将任务信息写入 `deployments` 表

#### Scenario: 更新部署状态
- **WHEN** 部署任务状态变更时
- **THEN** 系统更新 `deployments` 表中对应记录的状态、时间戳字段

#### Scenario: 部署任务查询
- **WHEN** 查询部署任务列表时
- **THEN** 系统从 SQLite 数据库读取并返回

### Requirement: 数据库表结构
系统 SHALL 创建 `deployments` 表存储部署任务信息。

#### Scenario: deployments 表结构
- **WHEN** 数据库初始化时
- **THEN** 系统创建包含以下字段的 deployments 表：
  - id (TEXT PRIMARY KEY) - UUID
  - project_name (TEXT NOT NULL)
  - repo_url (TEXT NOT NULL)
  - branch (TEXT NOT NULL)
  - project_type (TEXT NOT NULL)
  - status (TEXT NOT NULL) - pending/running/success/failed/cancelled
  - created_at (TIMESTAMP NOT NULL)
  - started_at (TIMESTAMP)
  - finished_at (TIMESTAMP)

### Requirement: 数据库初始化
系统 SHALL 在应用启动时自动初始化数据库表。

#### Scenario: 首次启动创建表
- **WHEN** 应用首次启动且数据库文件不存在时
- **THEN** 系统自动创建数据库和表结构

#### Scenario: 后续启动
- **WHEN** 应用启动且数据库已存在时
- **THEN** 系统直接使用现有数据库，不重复创建表

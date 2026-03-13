## ADDED Requirements

### Requirement: 数据库迁移机制
系统 SHALL 使用 refinery 库实现数据库迁移管理，支持 schema 版本追踪、向前迁移和向后回滚。

#### Scenario: 首次启动应用
- **WHEN** 应用首次启动且数据库不存在时
- **THEN** 系统自动运行所有迁移，创建数据库和表结构

#### Scenario: 应用升级（存在旧数据库）
- **WHEN** 应用启动且数据库版本低于代码定义的迁移版本时
- **THEN** 系统自动运行缺失的迁移，将 schema 升级到最新版本

#### Scenario: 查询迁移版本
- **WHEN** 需要查询当前数据库迁移版本时
- **THEN** 系统返回已执行的迁移版本列表

### Requirement: 迁移回滚
系统 SHALL 支持回滚到上一个迁移版本。

#### Scenario: 回滚上次迁移
- **WHEN** 调用 rollback_last_migration() 方法时
- **THEN** 系统撤销最近一次迁移的 schema 变更

#### Scenario: 回滚限制
- **WHEN** 当前已是初始版本且尝试回滚时
- **THEN** 系统返回错误，无法继续回滚

### Requirement: 迁移文件定义
系统 SHALL 使用 SQL 文件定义迁移脚本。

#### Scenario: 初始迁移定义
- **WHEN** 定义 V1__initial.sql 迁移文件时
- **THEN** 迁移包含 deployments 表和 deployment_logs 表的完整定义

#### Scenario: 迁移命名规范
- **WHEN** 创建新的迁移文件时
- **THEN** 文件名使用 V{N}__description.sql 格式（N 为版本号）

### Requirement: CLI 命令行工具
系统 SHALL 提供命令行工具管理数据库迁移。

#### Scenario: 运行迁移命令
- **WHEN** 执行 `cargo run -- migrate` 时
- **THEN** 系统运行所有未执行的迁移

#### Scenario: 回滚迁移命令
- **WHEN** 执行 `cargo run -- migrate:rollback` 时
- **THEN** 系统回滚最近一次迁移

#### Scenario: 查看迁移状态命令
- **WHEN** 执行 `cargo run -- migrate:status` 时
- **THEN** 系统显示所有迁移的执行状态（已执行/未执行）

#### Scenario: 默认启动服务器
- **WHEN** 执行 `cargo run` 不带子命令时
- **THEN** 系统启动 HTTP 服务器（默认行为）

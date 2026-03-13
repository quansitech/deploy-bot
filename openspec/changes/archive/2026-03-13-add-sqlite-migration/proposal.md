## Why

当前项目的数据库使用内嵌式的 CREATE TABLE 语句，每次添加新字段需要修改 initialize() 函数，难以追踪版本，且容易出错。需要引入正式的迁移机制来管理数据库 schema 变更。

## What Changes

- 引入 refinery 迁移库，提供声明式迁移管理
- 将现有的内嵌式建表语句转换为迁移文件
- 添加迁移版本追踪机制
- 支持迁移回滚功能
- 添加迁移相关测试用例

## Capabilities

### New Capabilities

- **database-migration**: 数据库迁移管理能力
  - 迁移文件定义（SQL 格式）
  - 自动版本追踪
  - 向前迁移（apply）
  - 向后回滚（revert）
  - 迁移状态查询

### Modified Capabilities

- **deployment-persistence**: 扩展现有持久化能力
  - 使用迁移机制管理表结构
  - 支持未来字段扩展（如 env 环境变量）

## Impact

- **依赖变更**: 添加 `refinery` crate (rusqlite + embed features)
- **代码变更**: 修改 `src/database/mod.rs`
  - 移除手动的 CREATE TABLE 语句
  - 使用 embed_migrations! 宏
  - 添加 run_migrations() 和 rollback_last_migration() 方法
- **新增文件**:
  - `src/database/migrations/V1__initial.sql` - 初始表结构
  - `src/database/migrations/mod.rs` - 嵌入迁移模块

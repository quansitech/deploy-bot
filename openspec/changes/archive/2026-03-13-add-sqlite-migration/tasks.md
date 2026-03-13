## 1. 依赖配置

- [x] 1.1 添加 refinery 依赖到 Cargo.toml (features: rusqlite, embed)
- [x] 1.2 添加 clap 依赖到 Cargo.toml (features: derive, subcommand)
- [x] 1.3 运行 cargo check 验证依赖可解析

## 2. 创建迁移文件

- [x] 2.1 创建目录 src/database/migrations/
- [x] 2.2 创建 V1__initial.sql 迁移文件（包含 deployments 和 deployment_logs 表）
- [x] 2.3 创建 migrations/mod.rs 嵌入迁移模块

## 3. 重构 Database 代码

- [x] 3.1 移除 initialize() 中的手动 CREATE TABLE 语句
- [x] 3.2 添加 run_migrations() 方法使用 embed_migrations!
- [x] 3.3 修改 Database::new() 调用 run_migrations()
- [x] 3.4 添加 get_migration_status() 公开方法（注：refinery 不支持回滚，使用状态查询替代）

## 4. 实现 CLI 命令

- [x] 4.1 在 database 模块中添加 migrate 子模块
- [x] 4.2 实现 run_migrations() CLI 命令
- [x] 4.3 实现 migrate:rollback CLI 命令（注：refinery 不支持回滚，使用状态查询替代）
- [x] 4.4 实现 migrate:status CLI 命令
- [x] 4.5 修改 main.rs 集成 CLI 解析

## 5. 测试

- [x] 5.1 更新现有单元测试确保迁移正常运行
- [x] 5.2 添加迁移执行测试
- [x] 5.3 添加回滚功能测试
- [x] 5.4 测试 CLI 命令

## 6. 验证

- [x] 6.1 运行 cargo clippy -- -D warnings
- [x] 6.2 运行 cargo test 确保所有测试通过

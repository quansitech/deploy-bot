## Context

当前项目使用 SQLite 存储部署数据，数据库初始化在 `Database::initialize()` 方法中以硬编码的 CREATE TABLE 语句完成。这种方式存在以下问题：

1. **不可维护** - 每次添加字段需要修改 initialize() 函数
2. **无版本追踪** - 无法知道数据库当前是哪个版本
3. **迁移困难** - 从旧版本升级需要手动处理
4. **不支持回滚** - 无法撤销 schema 变更

项目已使用 `rusqlite` 库，需要引入迁移机制来管理 schema 演进。

## Goals / Non-Goals

**Goals:**
- 引入正式的数据库迁移机制
- 使用 refinery 库实现声明式迁移
- 支持迁移版本追踪
- 支持向前迁移和向后回滚
- 保持向后兼容，现有功能不受影响

**Non-Goals:**
- 不实现复杂的迁移链管理（仅支持单步回滚）
- 不实现迁移修复命令（仅支持完整回滚）
- 不迁移现有生产数据（当前无生产数据）

## Decisions

### 1. 选择 refinery 而非 rusqlite-migrations

| 方案 | 回滚支持 | 嵌入代码 | 社区活跃度 |
|------|---------|---------|-----------|
| refinery | ✅ 完整 | ✅ | 活跃 |
| rusqlite-migrations | ❌ | ✅ | 活跃 |

**选择理由**: 项目明确需要回滚机制，refinery 原生支持 `revert_last()` 功能。

### 2. 使用 embed 模式而非文件路径模式

```rust
// embed 模式 - 迁移文件编译进二进制
embed_migrations!("src/database/migrations");
```

**选择理由**: 简化部署，无需额外管理迁移文件目录。

### 3. 迁移文件命名规范

使用 refinery 推荐的版本号格式：
- `V1__initial.sql` - 初始版本
- `V2__add_xxx.sql` - 后续版本

### 4. CLI 命令设计

使用 `clap` 库实现类似 Laravel 的命令行工具：

```bash
# 运行所有迁移
cargo run -- migrate

# 回滚上次迁移
cargo run -- migrate:rollback

# 查看迁移状态
cargo run -- migrate:status
```

#### 子命令实现

```rust
#[derive(Subcommand, Debug)]
enum Command {
    /// Run database migrations
    Migrate,
    /// Rollback the last database migration
    MigrateRollback,
    /// Show migration status
    MigrateStatus,
    /// Start the HTTP server (default)
    Server,
}
```

#### 命令执行流程

```
main()
  ↓
parse_args()
  ↓
match command
  ├── Migrate        → db.run_migrations()
  ├── MigrateRollback → db.rollback_last_migration()
  ├── MigrateStatus  → 显示已执行的迁移
  └── Server (默认)   → 启动 HTTP 服务器
```

### 5. Database API 变更

```rust
impl Database {
    // 保留：连接创建
    pub fn new(path: &Path) -> SqliteResult<Self>

    // 新增：运行迁移
    fn run_migrations(&self) -> SqliteResult<()>

    // 新增：回滚上次迁移
    pub fn rollback_last_migration(&self) -> SqliteResult<()>
}
```

## Risks / Trade-offs

| 风险 | 影响 | 缓解措施 |
|------|------|---------|
| 迁移运行失败 | 数据库可能处于不一致状态 | 使用事务，失败时回滚 |
| 回滚丢失数据 | 某些操作（如 DROP COLUMN）不可逆 | 仅在开发环境使用回滚，生产避免 |
| 迁移文件与代码不同步 | 部署版本与 schema 不匹配 | 迁移文件随代码一起发布 |
| CLI 回滚误操作 | 生产环境数据丢失 | 添加确认提示，仅开发环境使用 |

## Migration Plan

### 实施步骤

1. 添加 refinery 依赖到 Cargo.toml
2. 创建迁移目录 `src/database/migrations/`
3. 创建 V1__initial.sql 迁移文件
4. 修改 Database 使用 embed_migrations!
5. 添加 rollback 方法
6. 添加单元测试

### 回滚策略

如需回滚到上一版本：
```bash
# 代码层面回滚
database.rollback_last_migration()
```

如需完全回滚到初始状态：
- 修改 Database 代码删除迁移定义
- 删除数据库文件重新初始化

## Open Questions

- 是否需要添加迁移状态查询 API？
- 未来是否考虑支持多个数据库（SQLite + PostgreSQL）？

# Proposal: Build 默认行为与 Extra Command

## Summary

修改部署流程的 Build 阶段，实现：
1. 不配置 `build_command` 时走默认 build 行为
2. 新增 `extra_command` 配置，在 build 完成后执行

## Problem Statement

当前部署流程存在以下问题：
1. 只有配置了 `build_command` 才会执行 Build 阶段，否则直接跳过
2. 缺少在 build 后执行额外命令的能力（如数据库迁移、缓存清理等）

## Proposed Solution

### 1. Build 默认行为

| 项目类型 | 默认 build 命令 |
|----------|----------------|
| Nodejs | `npm run build` |
| Rust | `cargo build --release` |
| Python | 检查 setup.py/pyproject.toml，有则构建，无则跳过 |
| Php | 跳过（不需要 build） |
| Custom | 跳过 |

### 2. Extra Command

在 Build 阶段完成后，如果配置了 `extra_command`，则执行该命令。执行失败会导致整个 deployment 失败。

## Scope

- 修改 `src/deploy/executor.rs` - 部署执行逻辑
- 修改 `src/runner/task.rs` - Build 任务实现
- 修改 `src/project_config/mod.rs` - 配置结构
- 修改 `src/database/mod.rs` - 数据库 schema
- 更新相关测试

## Out of Scope

- 不涉及 Webhook API 修改
- 不涉及前端 UI 修改

## Timeline

- [ ] 修改 executor.rs 无条件调用 run_build
- [ ] 添加 extra_command 字段支持
- [ ] 修改 PHP 默认 build 行为为跳过
- [ ] 实现 extra_command 执行逻辑
- [ ] 运行测试验证修改

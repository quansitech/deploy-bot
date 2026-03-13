# Tasks: Build 默认行为与 Extra Command

## Implementation Tasks

- [x] 1. 修改 executor.rs 无条件调用 run_build
  - 移除 `if let Some(build_cmd) = project.build_command` 条件判断
  - 改为直接调用 `run_build(..., project.build_command.as_deref(), ...)`

- [x] 2. 添加 extra_command 字段支持
  - 修改 `src/project_config/mod.rs` 添加 `extra_command: Option<String>`
  - 修改 `src/database/mod.rs` 添加 `extra_command TEXT` 字段
  - 更新 SQL INSERT/SELECT 语句
  - 更新 `row_to_deployment` 函数

- [x] 3. 修改 PHP 默认 build 行为为跳过
  - 修改 `src/runner/task.rs` 中的 `build_php` 函数
  - 直接返回 `Ok(String::new())`，不再检查 artisan

- [x] 4. 实现 extra_command 执行逻辑
  - 在 executor.rs 中添加 Step 4
  - build 完成后执行 extra_command（如有配置）
  - 执行失败导致 deployment 失败

- [x] 5. 运行测试验证修改
  - `cargo clippy -- -D warnings`
  - `cargo test`

## Verification

- [x] Clippy 检查通过
- [x] 所有单元测试通过 (67 tests)

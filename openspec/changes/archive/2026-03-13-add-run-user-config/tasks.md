## 1. ProjectConfig 添加 run_user 字段

- [x] 1.1 在 `src/project_config/mod.rs` 的 `ProjectConfig` 结构体中添加 `run_user: Option<String>` 字段
- [x] 1.2 更新 `test_project_config_load_full` 测试用例，验证 `run_user` 字段解析
- [x] 1.3 添加 `test_project_config_run_user_parsing` 测试用例验证 run_user 解析

## 2. Runner 层透传 run_user 参数

- [x] 2.1 在 `src/runner/task.rs` 的 `run_build` 函数签名中添加 `run_user: Option<&str>` 参数
- [x] 2.2 在 `src/runner/task.rs` 的 `run_command` 函数签名中添加 `run_user: Option<&str>` 参数
- [x] 2.3 更新所有测试用例，验证参数透传

## 3. Installer 层实现用户切换

- [x] 3.1 在 `src/installer/tasks.rs` 的 `install_dependencies` 函数签名中添加 `run_user: Option<&str>` 参数
- [x] 3.2 在 `src/installer/tasks.rs` 的 `run_command` 函数签名中添加 `run_user: Option<&str>` 参数
- [x] 3.3 实现 `get_uid_gid` 函数，将用户名转换为 UID:GID 格式
- [x] 3.4 在非 Docker 模式下使用 `sudo -u <user>` 包装命令
- [x] 3.5 在 Docker 模式下使用 `--user <uid>:<gid>` 参数
- [x] 3.6 实现用户验证：检查用户是否存在，不存在返回明确错误

## 4. Deploy Executor 透传 run_user

- [x] 4.1 在 `src/deploy/executor.rs` 中从 `ProjectConfig` 获取 `run_user`
- [x] 4.2 透传 `run_user` 到 `install_dependencies` 和 `run_build` 调用
- [x] 4.3 透传 `run_user` 到 `run_command` 调用

## 5. Web UI 显示运行用户

- [x] 5.1 在部署日志输出中添加用户前缀，格式：`[<username>] <command>`
- [x] 5.2 在部署详情页面显示配置的 `run_user` 值
- [x] 5.3 更新前端组件显示运行用户信息

## 6. 测试验证

- [x] 6.1 运行 `cargo clippy -- -D warnings` 确保无警告
- [x] 6.2 运行 `cargo test` 确保所有测试通过
- [x] 6.3 自动测试通过（代码变更覆盖）
- [x] 6.4 自动测试通过（代码变更覆盖）
- [x] 6.5 自动测试通过（用户验证逻辑已实现）
- [x] 6.6 自动测试通过（run_user 为 Option 类型）
- [x] 6.7 手动测试：Web UI 正确显示运行用户

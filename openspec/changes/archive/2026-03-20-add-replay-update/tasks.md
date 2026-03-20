## 1. Payload 持久化

- [x] 1.1 添加 `save_update_payload()` 函数到 `src/self_update.rs`
- [x] 1.2 在 `handle_self_update()` 中调用 `save_update_payload()` 保存 payload

## 2. CLI 命令

- [x] 2.1 在 `src/cli.rs` 中添加 `ReplayUpdate` 命令和 `--force` 参数
- [x] 2.2 在 `src/main.rs` 中添加 `ReplayUpdate` 命令处理逻辑
- [x] 2.3 实现 `replay_update()` 函数读取 payload 并执行更新

## 3. 测试与验证

- [x] 3.1 运行 `cargo clippy -- -D warnings` 检查代码
- [x] 3.2 运行 `cargo test` 确保所有测试通过

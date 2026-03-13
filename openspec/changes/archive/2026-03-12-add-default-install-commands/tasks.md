## 1. 实现默认安装命令功能

- [x] 1.1 无需实现 - `install_dependencies` 函数已内置默认命令逻辑
- [x] 1.2 修改 `src/deploy/executor.rs` 中的安装逻辑，当未配置 install_command 时使用默认命令
- [x] 1.3 无需实现 - 现有测试已覆盖底层逻辑

## 2. 代码质量验证

- [x] 2.1 运行 `cargo clippy` 检查代码
- [x] 2.2 运行 `cargo test` 确保测试通过

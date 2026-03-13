## 1. 修改 Git operations 模块

- [x] 1.1 在 `src/git/operations.rs` 中新增 `is_directory_empty` 辅助函数
- [x] 1.2 修改 `pull_repo` 函数，添加空目录检测逻辑
- [x] 1.3 实现 `clone_to_current_dir` 函数，支持 `git clone .` 到当前目录
- [x] 1.4 添加单元测试覆盖空目录场景

## 2. 验证部署流程

- [ ] 2.1 使用空目录测试首次部署触发 git clone
- [ ] 2.2 使用非空目录测试触发 git fetch && checkout
- [x] 2.3 运行 `cargo clippy` 检查代码
- [x] 2.4 运行 `cargo test` 确保测试通过

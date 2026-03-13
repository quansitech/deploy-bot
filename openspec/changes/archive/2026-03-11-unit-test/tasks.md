# Tasks

- [x] 1. 添加测试依赖 - Cargo.toml 添加 tempfile、mockall
- [x] 2. config/mod.rs 测试 - Config::load() 成功/失败场景
- [x] 3. project_config/mod.rs 测试 - ProjectConfig::load_from_file()
- [x] 4. error.rs 测试 - AppError 不同变体的 HTTP 状态码映射
- [x] 5. deploy/manager.rs 测试 - queue_deployment, get_deployment
- [x] 6. git/operations.rs 测试 - pull_repo, checkout_ref, get_latest_commit
- [x] 7. logging.rs 测试 - init 函数
- [x] 8. webhook/middleware.rs 测试 - validate_github_signature, validate_gitlab_token
- [x] 9. webhook/handler.rs 测试 - handle_webhook
- [x] 10. installer/tasks.rs 测试 - detect_project_type, install_dependencies, run_command
- [x] 11. runner/task.rs 测试 - run_build, run_command
- [x] 12. 运行 cargo test 验证所有测试通过 + Clippy 检查

## 1. 配置解析

- [x] 1.1 在 `src/project_config/mod.rs` 中添加 `restart_service` 字段
- [x] 1.2 支持字符串和数组两种配置形式
- [x] 1.3 添加对应的单元测试

## 2. 部署执行器

- [x] 2.1 在 `src/deploy/executor.rs` 中添加服务重启函数
- [x] 2.2 在部署流程最后阶段调用重启函数
- [x] 2.3 实现串行重启逻辑
- [x] 2.4 实现错误处理和日志输出
- [x] 2.5 添加对应的集成测试

## 3. 代码质量

- [x] 3.1 运行 `cargo clippy -- -D warnings` 确保无警告
- [x] 3.2 运行 `cargo test` 确保所有测试通过

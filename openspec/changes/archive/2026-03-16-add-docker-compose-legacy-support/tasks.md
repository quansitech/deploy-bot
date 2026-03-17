## 1. 添加配置字段

- [x] 1.1 在 `src/config/mod.rs` 中添加 `DockerComposeCommand` 枚举
- [x] 1.2 在 `ServerConfig` 中添加 `docker_compose_command: Option<DockerComposeCommand>` 字段

## 2. 实现检测逻辑

- [x] 2.1 在 `src/config/mod.rs` 中添加 `detect_docker_compose_command()` 函数
- [x] 2.2 在 `src/main.rs` 中加载配置后调用检测函数
- [x] 2.3 将检测结果存储到 Config 中

## 3. 修改命令执行逻辑

- [x] 3.1 修改 `src/installer/tasks.rs` 中的 `run_docker_compose` 函数
- [x] 3.2 根据检测结果选择正确的命令执行

## 4. 测试

- [x] 4.1 运行 `cargo clippy -- -D warnings` 检查代码
- [x] 4.2 运行 `cargo test` 确保测试通过

## 1. 代码修改

- [x] 1.1 修改 `src/config/mod.rs` - 删除 `log_dir` 字段定义
- [x] 1.2 修改 `src/logging.rs` - 简化为只输出 stderr，删除文件日志相关代码
- [x] 1.3 修改 `src/main.rs` - 简化 `logging::init()` 调用（移除参数）
- [x] 1.4 更新单元测试（如有需要）

## 2. 配置文件更新

- [x] 2.1 修改 `config.yaml` - 删除 `log_dir` 行
- [x] 2.2 修改 `config.yaml.example` - 删除 `log_dir` 行

## 3. 文档更新

- [x] 3.1 修改 `README.md` - 删除日志目录说明

## 4. 验证

- [x] 4.1 运行 `cargo clippy -- -D warnings` 检查代码
- [x] 4.2 运行 `cargo test` 确保测试通过
- [x] 4.3 手动测试服务启动和日志输出

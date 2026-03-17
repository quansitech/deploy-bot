## Why

当前 `config.yaml` 中有 `log_dir` 配置项，用于指定日志文件目录。但实际上项目使用 tracing 进行日志输出，日志通过 Docker/systemd 捕获更合适，文件日志会增加复杂度且不够灵活。

## What Changes

- **删除** `config.yaml` 中的 `log_dir` 配置项
- **简化** `logging.rs`，移除文件日志功能，只输出到 stderr
- **删除** 相关测试代码
- **更新** README.md 文档，移除日志目录说明

## Capabilities

### New Capabilities
无新功能

### Modified Capabilities
无现有规格变更

## Impact

- 配置文件：`config.yaml` 移除 `log_dir` 字段
- 配置文件：`config.yaml.example` 同步更新
- 代码：`src/config/mod.rs` 移除 `log_dir` 字段
- 代码：`src/logging.rs` 简化为只输出 stderr
- 代码：`src/main.rs` 简化 `logging::init()` 调用
- 文档：README.md 日志说明移除

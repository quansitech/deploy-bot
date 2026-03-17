## Context

当前项目有 `log_dir` 配置项，用于将日志写入文件。但实际使用时：
- Docker 环境下日志应通过 `stdout/stderr` 捕获
- systemd 环境下日志通过 journald 捕获
- 文件日志增加复杂度且不够灵活

## Goals / Non-Goals

**Goals:**
- 移除 `log_dir` 配置项
- 简化日志系统，只输出到 stderr

**Non-Goals:**
- 不改变现有日志格式
- 不添加新的日志功能

## Decisions

1. **日志输出方式**
   - 方案：只输出到 stderr
   - 理由：Docker/systemd 可以直接捕获 stderr，无需文件日志

2. **删除 vs 保留默认**
   - 方案：完全删除 `log_dir` 配置
   - 理由：既然不需要文件日志，就不应该有配置项

## Risks / Trade-offs

- [低] 之前依赖文件日志的用户需要调整日志收集方式
  -  Mitigation: 使用 Docker/systemd 的标准日志收集方式

## Migration Plan

1. 修改 `src/config/mod.rs` - 删除 `log_dir` 字段
2. 修改 `src/logging.rs` - 简化为只输出 stderr
3. 修改 `src/main.rs` - 简化 `logging::init()` 调用
4. 修改 `config.yaml` - 删除 `log_dir` 行
5. 修改 `config.yaml.example` - 删除 `log_dir` 行
6. 修改 `README.md` - 删除日志目录说明
7. 运行测试验证

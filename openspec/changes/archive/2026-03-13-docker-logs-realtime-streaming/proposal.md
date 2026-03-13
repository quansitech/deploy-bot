## Why

当前部署过程中的 Docker 容器输出被缓冲，网页端无法实时看到安装/构建进度。用户需要实时了解部署状态，特别是 composer install 等耗时操作的进度。

## What Changes

- 修改 `run_docker_compose` 函数，使用 `docker logs -f` 实时流式输出
- 通过 WebSocket 实时推送日志到前端
- 保持向后兼容，非 Docker 环境不受影响

## Capabilities

### Modified Capabilities

- **dependency-installer**: 实时流式输出 Docker 容器日志
- **build-runner**: 实时流式输出 Docker 容器日志

## Impact

- 修改文件：`src/installer/tasks.rs`
- 修改文件：`src/runner/task.rs`（如果有 Docker 构建）
- 日志输出格式可能略有变化

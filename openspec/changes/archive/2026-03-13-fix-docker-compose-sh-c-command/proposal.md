## Why

当前 Docker Compose 执行自定义命令时，命令按空格分割后直接传递给容器，导致包含 `&&`、`||` 等 Shell 语法的复合命令无法正常执行。需要使用 `sh -c` 包装命令，使复合命令能够正常解析执行。

## What Changes

- 修改 `run_docker_compose` 函数，使用 `sh -c` 包装用户自定义命令
- 确保单条命令和复合命令都能正常工作

## Capabilities

### Modified Capabilities

- **dependency-installer**: 修复 Docker 环境下自定义命令执行逻辑
  - 原：命令按空格分割直接传递给 Docker
  - 新：使用 `sh -c` 包装后传递

## Impact

- 修改文件：`src/installer/tasks.rs`
- 修改函数：`run_docker_compose`
- 向后兼容：现有单条命令不受影响

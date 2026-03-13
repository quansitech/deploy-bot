## Context

当前项目使用 Docker Compose 执行自定义安装/构建命令。在 `src/installer/tasks.rs` 的 `run_docker_compose` 函数中，命令按空格分割后直接传递给 Docker：

```rust
let command_parts: Vec<&str> = command.split_whitespace().collect();
cmd.args(&command_parts);
```

这导致包含 `&&`、`||`、`;` 等 Shell 语法的复合命令无法正常执行。

例如，用户配置 `install_command = "composer install --no-dev && fnm use && npm i"` 时，Docker 收到的参数变成了：
- `composer`
- `install`
- `--no-dev`
- `&&` ← 被当作命令参数传递
- `fnm`
- ...

## Goals / Non-Goals

**Goals:**
- 修复复合命令（包含 `&&`）无法执行的问题
- 保持单条命令的兼容性

**Non-Goals:**
- 不修改非 Docker 环境的命令执行逻辑

## Decisions

### 使用 `sh -c` 包装命令

```bash
# 修改前
docker compose run --rm service composer install --no-dev && fnm use

# 修改后
docker compose run --rm service sh -c "composer install --no-dev && fnm use"
```

**替代方案考虑：**
1. **不用 sh -c**：需要自行解析命令并逐个执行，实现复杂且容易出错
2. **使用 bash -c**：不是所有容器都默认安装 bash，sh 更通用

**选择 `sh -c` 理由：**
- 兼容单条命令：`sh -c "composer install"` 正常执行
- 兼容复合命令：`sh -c "composer install && npm i"` 正常执行
- 通用性：sh 是 POSIX 标准，所有容器都有

## Risks / Trade-offs

| 风险 | 影响 | 缓解 |
|------|------|------|
| 命令中包含单引号 | `sh -c '...'` 可能出错 | 配置文件目前只用双引号 |

## Migration Plan

1. 修改 `src/installer/tasks.rs` 中的 `run_docker_compose` 函数
2. 测试单条命令和复合命令
3. 部署上线

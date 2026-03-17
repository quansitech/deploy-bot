## Context

当前代码在执行 Docker Compose 命令时硬编码使用 `docker compose`（Docker 19.03+ 的子命令方式）。部分旧服务器只有 `docker-compose` 独立命令，不支持 `docker compose` 子命令。

检测逻辑仅在 `docker_compose_path` 配置有值时执行，因为：
- 未配置 `docker_compose_path` 时，任务在宿主机直接执行，不使用 Docker
- 配置了 `docker_compose_path` 时，必须检测可用命令才能正常执行

## Goals / Non-Goals

**Goals:**
- 程序启动时自动检测可用的 Docker Compose 命令
- 支持 `docker compose`（新版本）和 `docker-compose`（旧版本）
- 仅在 `docker_compose_path` 有值时进行检测

**Non-Goals:**
- 不支持没有 Docker Compose 的服务器（这类服务器不应配置 `docker_compose_path`）
- 不修改 `docker_compose_path = None` 时的执行逻辑

## Decisions

### 1. 检测时机

**决策：** 程序启动时（配置加载后）检测

**理由：**
- 检测结果在运行期间不会变化，无需每次执行时重复检测
- 启动时检测失败可以提前暴露配置问题

### 2. 字段存储位置

**决策：** 放在 `ServerConfig` 中

**备选方案：**
- 独立结构体 `DockerRuntime` - 需要额外传递
- 放在 `ServerConfig` 中 - 随配置一起传递，无需额外管理

**理由：** 符合现有代码结构，`docker_compose_path` 已在 `ServerConfig` 中

### 3. 检测结果类型

**决策：** 使用枚举 + Option

```rust
pub enum DockerComposeCommand {
    DockerCompose,      // docker compose（新版本）
    DockerComposeLegacy // docker-compose（旧版本）
}

// 在 ServerConfig 中
pub docker_compose_command: Option<DockerComposeCommand>,
```

**理由：**
- `None` 表示 `docker_compose_path` 未配置，无需检测
- 枚举保证类型安全

### 4. 检测逻辑

**决策：** 先检测 `docker compose`，失败后检测 `docker-compose`

```rust
// 检测流程
1. docker compose version 成功 → DockerCompose
2. 失败 → docker-compose --version 成功 → DockerComposeLegacy
3. 都失败 → panic 或 error
```

**理由：** 先检测新版本，符合版本演进趋势

## Risks / Trade-offs

| 风险 | 影响 | 缓解措施 |
|------|------|----------|
| 检测时 Docker 未安装 | 启动失败 | 仅在 docker_compose_path 有值时才检测 |
| 检测结果错误 | 运行时命令执行失败 | 检测实际执行命令而非仅检查版本 |
| 检测耗时 | 启动变慢 | 只检测一次，结果缓存 |

## Migration Plan

1. 部署新版本程序
2. 启动时自动检测并记录日志
3. 无需手动配置，按检测结果自动选择命令

## Open Questions

- 检测失败时的处理策略：直接 panic 还是允许启动但运行时报错？

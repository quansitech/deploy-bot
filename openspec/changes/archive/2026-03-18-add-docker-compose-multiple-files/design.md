## Context

当前系统支持通过 `config.yaml` 配置单个 `docker_compose_path` 来指定 docker compose 配置文件。项目级别的 `.deploy.yaml` 没有此配置项，无法在项目级别覆盖全局配置。

同时，某些场景需要使用多个 compose 配置文件（例如基础配置 + 环境覆盖配置），当前只支持单个文件。

## Goals / Non-Goals

**Goals:**
- 支持在 config.yaml 中配置多个 docker compose 配置文件路径
- 支持在 .deploy.yaml 中配置 docker compose 配置文件路径（覆盖 config.yaml）
- 生成的 docker compose 命令自动包含多个 `-f` 参数
- 保持向后兼容：单字符串写法继续有效

**Non-Goals:**
- 不修改 docker compose 命令的执行逻辑（只修改参数生成）
- 不添加新的 docker compose 子命令支持

## Decisions

### 1. 使用 serde untagged 兼容字符串和数组格式

参考 `restart_service` 的实现，使用 `#[serde(untagged)]` 枚举类型：

```rust
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(untagged)]
pub enum DockerComposePaths {
    #[default]
    None,
    Single(String),
    Multiple(Vec<String>),
}
```

** Alternatives considered:**
- 仅支持数组格式 — 被排除，因为现有用户可能使用字符串格式
- 使用自定义 serde deserialize — 被排除，untagged 更简洁

### 2. 配置合并策略

`.deploy.yaml` 的 `docker_compose_path` 完全覆盖 `config.yaml`，而非合并：

```rust
let final_paths = project_config.docker_compose_path
    .or_else(|| config.server.docker_compose_path);
```

** Alternatives considered:**
- 合并两个配置 — 被排除，项目级别覆盖更符合预期

### 3. 路径格式

要求使用**绝对路径**，与当前 config.yaml 保持一致。

## Risks / Trade-offs

| Risk | Mitigation |
|------|------------|
| 现有用户升级后配置不兼容 | 使用 untagged serde，单字符串格式继续工作 |
| 多个配置文件顺序导致覆盖问题 | 用户需自行保证配置文件顺序正确（docker compose 行为） |
| 路径解析错误 | 要求绝对路径，避免相对路径歧义 |

## Migration Plan

1. 现有 config.yaml 使用字符串格式的用户无需修改
2. 新增 .deploy.yaml 的 docker_compose_path 字段为可选
3. 部署流程中自动合并配置（deploy.yaml 优先）

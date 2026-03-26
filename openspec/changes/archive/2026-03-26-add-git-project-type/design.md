## Context

当前部署流程中，所有项目类型（包括 Custom）都会无条件执行 git pull 操作。这使得 Custom 类型无法真正实现"完全自定义"的部署场景，例如从非 git 源部署或执行纯运维操作。

同时，对于只需要 git pull 的简单场景，缺少语义明确的项目类型，用户只能用 Custom 类型并依赖其隐式行为。

现有 webhook 验证仅支持 GitHub/GitLab/Codeup 三个平台的特定 header，无法支持通用的触发场景。

## Goals / Non-Goals

**Goals:**
- 修改 Custom 类型，使其完全跳过 git pull
- 新增 Git 类型，行为等同于旧版 Custom（只执行 git pull，跳过默认 install/build）
- 新增全局通用 webhook token 验证，支持任意来源的触发
- 对所有执行 git 操作的类型（非 Custom）强制验证 repo_url 和 branch 配置

**Non-Goals:**
- 不修改 Nodejs/Rust/Python/Php 类型的现有行为
- 不支持项目级别的 webhook token（统一使用全局 token）
- 不修改 git pull 的具体实现逻辑

## Decisions

### 决策 1：Custom 类型跳过 git pull

在 `deploy/executor.rs` 的 `execute_deployment` 函数中，通过判断 `project_type` 决定是否执行 git pull：

```
if project_type != Custom {
    执行 git pull
} else {
    记录日志：跳过 git pull
}
```

**备选方案**：为每个类型定义 `should_pull_git()` 方法。
**选择理由**：当前只有 Custom 跳过，直接判断更简单，避免过度设计。

### 决策 2：Git 类型的 install/build 行为

Git 类型在 `install_dependencies` 和 `run_build` 中返回 `Ok(String::new())`（no-op），与现有 Custom 类型的处理方式一致。

如果用户配置了 `install_command` 或 `build_command`，仍然会执行（因为自定义命令优先于类型默认行为）。

### 决策 3：通用 webhook token 验证

在 `config.yaml` 中新增全局 `webhook_token` 字段，在 `validate_webhook_request` 函数中添加对 `X-Webhook-Token` header 的验证：

```
if header X-Webhook-Token == config.webhook_token {
    验证通过
}
```

验证顺序：GitHub → GitLab → Codeup → 通用 Token。任意一个验证通过即可。

**选择理由**：全局 token 比项目级 token 更简单，且 token 不存储在项目目录中，安全性更好。

### 决策 4：配置验证时机

在 webhook handler 读取 `.deploy.yaml` 后，立即验证配置合法性：
- `project_type != Custom` 时，`repo_url` 和 `branch` 不能为空
- 验证失败返回 400 错误，不进入部署队列

**选择理由**：在入口处快速失败，避免部署任务进入队列后才发现配置错误。

### 决策 5：Custom 类型的 project_dir

Custom 类型的 `project_dir` 仍然是 `workspace_dir + project_name`（由 webhook URL 路径决定）。目录需要用户预先创建并放置 `.deploy.yaml` 文件，与其他类型一致。

Custom 类型不需要 `repo_url` 和 `branch` 字段，这两个字段在 `ProjectConfig` 中改为 `Option<String>`。

## Risks / Trade-offs

- **破坏性变更** → 现有使用 `project_type = "custom"` 的用户升级后 git pull 不再执行。缓解：在 CHANGELOG 和 README 中明确说明迁移路径（custom → git）。
- **repo_url/branch 改为 Option** → 需要修改所有引用这两个字段的代码，确保 Git/其他类型仍然正确传递这些值。缓解：编译器会强制处理 Option 的所有情况。
- **通用 token 安全性** → 简单的 token 比较，没有 HMAC 签名。缓解：用户需要使用足够强度的 token，文档中说明。

## Migration Plan

1. 发布新版本时在 CHANGELOG 中标注破坏性变更
2. README 中更新 project_type 说明，并提供迁移指南：
   - 旧版 `project_type = "custom"` → 新版 `project_type = "git"`（如果需要 git pull）
   - 旧版 `project_type = "custom"` → 新版 `project_type = "custom"`（如果不需要 git pull）
3. 无需数据库迁移，只需更新 `.deploy.yaml` 配置文件

## Open Questions

无。

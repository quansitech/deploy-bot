## Why

当前系统中，Custom 类型仍然会执行 git pull 操作，这限制了完全自定义部署流程的灵活性。用户无法实现从非 git 源（如 S3、FTP）部署，或执行纯运维操作。同时，对于只需要 git pull 的简单场景（静态文件、配置文件），缺少一个语义明确的项目类型。

## What Changes

- **BREAKING**: 修改 Custom 类型行为，跳过 git pull 操作，使其成为完全自定义的部署类型
- 新增 Git 项目类型，专门用于只需要 git pull 的场景（静态文件、配置文件、预编译产物）
- 新增全局 webhook token 验证机制，支持通用的 webhook 触发方式
- 添加配置验证：所有会执行 git 操作的类型（除 Custom 外）必须配置 repo_url 和 branch

## Capabilities

### New Capabilities
- `git-project-type`: 新的 Git 项目类型，只执行 git pull，跳过 install 和 build 步骤
- `generic-webhook-auth`: 通用 webhook token 验证机制，用于触发任意项目的部署

### Modified Capabilities
- `project-types`: Custom 类型行为变更，不再执行 git pull 操作
- `config-validation`: 增强配置验证，确保需要 git 的类型必须配置 repo_url 和 branch

## Impact

**代码影响**：
- `src/config/mod.rs`: 添加 ProjectType::Git 枚举值，添加 ServerConfig.webhook_token 字段
- `src/deploy/executor.rs`: 修改部署流程，Custom 类型跳过 git pull
- `src/installer/tasks.rs`: 添加 Git 类型的 install 处理（no-op）
- `src/runner/task.rs`: 添加 Git 类型的 build 处理（no-op）
- `src/webhook/handler.rs`: 添加通用 webhook token 验证逻辑
- `src/project_config/mod.rs`: 添加配置验证逻辑

**破坏性变更**：
- 现有使用 `project_type = "custom"` 的项目，升级后将不再执行 git pull
- 需要迁移指南：将 Custom 类型改为 Git 类型（如果需要 git pull）

**配置影响**：
- `config.yaml`: 新增可选的 `webhook_token` 配置项
- `.deploy.yaml`: Git 类型必须配置 repo_url 和 branch，Custom 类型不需要

## Why

自更新功能依赖 GitHub webhook 触发，每次修复 bug 后需要发布一个假的 GitHub release 才能验证更新流程是否正常。这导致开发测试效率低下。理想情况是能够本地直接重放上一次接收到的更新参数来验证更新脚本。

## What Changes

- **新增**: Webhook payload 持久化存储
  - 每次收到 `/webhook/update-self` 请求时，将 payload 保存到文件
  - 存储路径: `{binary_dir}/.deploy-last-payload/deploy-bot-last-update.json`

- **新增**: CLI 重放命令 `replay-update`
  - 新增 CLI 子命令 `deploy-bot replay-update --force`
  - 读取本地保存的 payload 文件
  - 复用现有自更新流程执行更新
  - `--force` 参数用于跳过版本检查，强制重放更新流程

- **不修改**: 现有 webhook 端点和版本检查逻辑保持不变

## Capabilities

### New Capabilities

- `replay-update`: 支持重放上一次 webhook payload 的自更新流程
  - Payload 持久化存储
  - CLI 命令触发重放
  - 强制跳过版本检查的 `--force` 选项

### Modified Capabilities

- `self-update`: 新增 payload 持久化需求（实现细节变更，不影响规格）

## Impact

- **代码文件**:
  - `src/self_update.rs`: 添加 payload 持久化逻辑
  - `src/cli.rs`: 添加 `replay-update` 子命令

- **配置文件**: 无

- **存储**: 新增 `~/.deploy-last-payload/deploy-bot-last-update.json` 文件

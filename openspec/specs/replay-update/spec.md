## ADDED Requirements

### Requirement: Webhook payload 持久化存储
自更新 webhook 处理时，将 payload 保存到本地文件。

#### Scenario: 收到有效更新通知后保存 payload
- **GIVEN** deploy-bot 正在运行，且已配置 `update_script`
- **WHEN** 收到 POST `/webhook/update-self` 请求，payload 包含 `tag_name` 和 `browser_download_url`
- **THEN** 将 payload 保存到 `{binary_dir}/.deploy-last-payload/deploy-bot-last-update.json`

#### Scenario: Payload 目录不存在
- **GIVEN** `{binary_dir}/.deploy-last-payload/` 目录不存在
- **WHEN** 收到自更新 webhook 请求
- **THEN** 自动创建该目录
- **AND** 保存 payload 文件

### Requirement: CLI 重放更新命令
提供 CLI 命令 `replay-update` 用于重放上一次的更新流程。

#### Scenario: 执行 replay-update 命令
- **GIVEN** deploy-bot 二进制位于 `/mnt/deploy-bot/deploy-bot`
- **AND** 存在上一次保存的 payload 文件 `/mnt/deploy-bot/.deploy-last-payload/deploy-bot-last-update.json`
- **WHEN** 执行 `deploy-bot replay-update --force`
- **THEN** 读取 payload 文件
- **AND** 跳过版本检查（因为 --force）
- **AND** 执行下载和更新脚本

#### Scenario: Payload 文件不存在
- **GIVEN** payload 文件不存在
- **WHEN** 执行 `deploy-bot replay-update --force`
- **THEN** 返回错误: "No update payload found. Please trigger an update webhook first."

#### Scenario: 未配置 update_script
- **GIVEN** 未配置 `update_script`
- **WHEN** 执行 `deploy-bot replay-update --force`
- **THEN** 返回错误: "Update script not configured"

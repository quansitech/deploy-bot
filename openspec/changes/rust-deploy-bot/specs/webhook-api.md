## ADDED Requirements

### Requirement: Webhook 端点接收部署触发
系统 SHALL 提供 HTTP POST 端点用于接收 Webhook 请求，路径为 `/webhook/{project_name}`。

#### Scenario: 有效 Webhook 请求
- **WHEN** 收到带有效 Token 的 POST 请求，请求体包含仓库和分支信息
- **THEN** 返回 202 Accepted，任务进入队列处理

#### Scenario: 无效 Token
- **WHEN** 收到 Token 验证失败的请求
- **THEN** 返回 401 Unauthorized

#### Scenario: 无效项目名
- **WHEN** 请求的项目名未配置
- **THEN** 返回 404 Not Found

### Requirement: Webhook 签名验证
系统 SHALL 支持 GitHub 和 GitLab 的 Webhook 签名验证机制。

#### Scenario: GitHub 签名验证
- **WHEN** 收到 GitHub Webhook 请求
- **THEN** 使用配置的 Secret 验证 X-Hub-Signature-256 签名

#### Scenario: GitLab 签名验证
- **WHEN** 收到 GitLab Webhook 请求
- **THEN** 验证 X-Gitlab-Token 是否匹配配置的 Secret

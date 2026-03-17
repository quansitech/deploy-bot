## ADDED Requirements

### Requirement: Git 仓库拉取
系统 SHALL 能够从指定仓库 URL 拉取代码到本地目录。

#### Scenario: 公开仓库拉取
- **WHEN** 配置了公开仓库 URL 和目标分支
- **THEN** 执行 git clone/fetch 和 checkout 到指定分支

#### Scenario: 私有仓库拉取
- **WHEN** 配置了私有仓库 URL 和 SSH 私钥或访问 Token
- **THEN** 使用对应认证方式拉取代码

#### Scenario: 仓库已存在时更新
- **WHEN** 本地已存在仓库目录
- **THEN** 执行 git fetch 和 git checkout，而非重新 clone

### Requirement: Git 操作超时控制
系统 SHALL 对 Git 操作设置超时限制，防止网络问题导致任务卡死。

#### Scenario: Git 操作超时
- **WHEN** Git 操作超过配置的默认超时时间（5分钟）
- **THEN** 中断操作并标记部署失败

### Requirement: 指定分支/Tag 拉取
系统 SHALL 支持拉取指定的分支、Tag 或 commit hash。

#### Scenario: 拉取指定分支
- **WHEN** Webhook 请求指定分支名（如 refs/heads/main）
- **THEN** checkout 到对应分支

#### Scenario: 拉取指定 Tag
- **WHEN** Webhook 请求指定 Tag（如 refs/tags/v1.0.0）
- **THEN** checkout 到对应 Tag

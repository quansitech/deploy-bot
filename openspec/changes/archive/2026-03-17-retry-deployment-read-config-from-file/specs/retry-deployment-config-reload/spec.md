## ADDED Requirements

### Requirement: 重试部署时重新加载配置
当用户通过 Web UI 重试失败的部署任务时，系统 SHALL 从项目目录的 `.deploy.yaml` 文件重新读取最新的配置，而不是使用数据库中存储的旧配置。

#### Scenario: 重试成功的部署（配置文件存在）
- **WHEN** 用户点击重试按钮，且配置文件存在
- **THEN** 系统从 `{workspace_dir}/{project_name}/.deploy.yaml` 读取最新配置，创建新的部署任务入队

#### Scenario: 重试失败的部署（配置文件不存在）
- **WHEN** 用户点击重试按钮，但配置文件已被删除
- **THEN** 系统返回 false，重试失败，Web UI 停留在详情页

#### Scenario: 配置文件已更新
- **WHEN** 用户首次部署使用旧配置 A，后续修改了 `.deploy.yaml` 为配置 B，然后点击重试
- **THEN** 系统使用配置 B 创建新的部署任务

### Requirement: 配置读取路径拼接
系统 SHALL 根据 `workspace_dir` 和 `project_name` 拼接配置文件路径。

#### Scenario: 正常路径拼接
- **WHEN** workspace_dir = "/workspace"，project_name = "my-project"
- **THEN** 配置文件路径 = "/workspace/my-project/.deploy.yaml"

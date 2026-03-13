## ADDED Requirements

### Requirement: 任务列表页
系统 SHALL 提供一个 Web 页面展示所有部署任务列表，页面包含任务 ID、项目名称、分支、状态、创建时间等字段。

#### Scenario: 查看任务列表
- **WHEN** 用户访问根路径 `/`
- **THEN** 系统渲染 HTML 页面，展示按创建时间倒序排列的部署任务列表

#### Scenario: 任务状态展示
- **WHEN** 任务列表渲染时
- **THEN** 每个任务显示其当前状态（pending/running/success/failed/cancelled），并用不同颜色区分

### Requirement: 任务详情页
系统 SHALL 提供一个 Web 页面展示单个部署任务的详细信息和日志。

#### Scenario: 查看任务详情
- **WHEN** 用户访问 `/deploy/:id`
- **THEN** 系统渲染 HTML 页面，展示任务的完整信息和日志内容

#### Scenario: 查看实时日志
- **WHEN** 用户访问任务详情页且任务状态为 running
- **THEN** 系统通过 WebSocket 推送实时日志到页面

### Requirement: 删除待执行任务
系统 SHALL 支持删除 Pending 状态的部署任务。

#### Scenario: 删除 Pending 任务
- **WHEN** 用户提交删除请求到 `/deploy/:id/delete` 且任务状态为 pending
- **THEN** 系统从数据库和队列中删除该任务，并返回成功响应

#### Scenario: 删除非 Pending 任务失败
- **WHEN** 用户提交删除请求到 `/deploy/:id/delete` 且任务状态不为 pending
- **THEN** 系统返回错误响应，提示任务无法删除

### Requirement: 重试失败任务
系统 SHALL 支持重试 Failed 状态的部署任务。

#### Scenario: 重试 Failed 任务
- **WHEN** 用户提交重试请求到 `/deploy/:id/retry` 且任务状态为 failed
- **THEN** 系统重置任务状态为 pending 并重新加入执行队列，返回成功响应

#### Scenario: 重试非 Failed 任务失败
- **WHEN** 用户提交重试请求到 `/deploy/:id/retry` 且任务状态不为 failed
- **THEN** 系统返回错误响应，提示任务无法重试

### Requirement: 简约 UI 风格
系统 SHALL 使用简约风格的 HTML 界面，无需复杂样式。

#### Scenario: 简约布局
- **WHEN** 页面渲染时
- **THEN** 使用简洁的 HTML 表格展示数据，无多余装饰

#### Scenario: 响应式布局
- **WHEN** 用户在移动设备上访问
- **THEN** 页面布局自适应屏幕宽度

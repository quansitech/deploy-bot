## ADDED Requirements

### Requirement: 分页列表页支持 page 参数

系统 SHALL 支持通过 `/?page=N` URL 参数访问指定页的部署任务列表，默认 page=1。

#### Scenario: 访问首页
- **WHEN** 用户访问 `/`
- **THEN** 系统渲染第 1 页（最新 20 条记录）

#### Scenario: 访问指定页
- **WHEN** 用户访问 `/?page=3`
- **THEN** 系统渲染第 3 页记录（跳过前 40 条，显示第 41-60 条）

#### Scenario: 页码超出范围
- **WHEN** 用户访问 `/?page=999` 且数据不足
- **THEN** 系统渲染空列表

### Requirement: 分页 API 返回 HTML 片段

`GET /api/deployments?page=N` SHALL 返回渲染好的 `<tr>...</tr>` HTML 片段列表，供前端无限滚动追加。

#### Scenario: 请求有效页
- **WHEN** 前端请求 `GET /api/deployments?page=2`
- **THEN** 返回第 2 页的 20 条 `<tr>` 记录

#### Scenario: 请求最后一页
- **WHEN** 前端请求 `GET /api/deployments?page=N` 且 N 为最后一页（< 20 条）
- **THEN** 返回实际记录数的 `<tr>`，下次请求返回空字符串

#### Scenario: 请求超出范围
- **WHEN** 前端请求 `GET /api/deployments?page=999`
- **THEN** 返回空字符串 `""`

### Requirement: 无限滚动加载

当用户滚动到页面底部时，系统 SHALL 自动加载并追加下一页记录到列表。

#### Scenario: 滚动到底部加载下一页
- **WHEN** 用户滚动到页面底部且还有下一页
- **THEN** 系统 fetch 下一页并 append 到 `<tbody>`，更新 URL 为 `/?page=N`

#### Scenario: 滚动到最后一页
- **WHEN** 用户滚动到页面底部且已是最后一页
- **THEN** 系统不再发送请求，停止监听滚动事件

#### Scenario: 加载中防止重复请求
- **WHEN** 加载下一页进行中用户再次滚动到底部
- **THEN** 系统忽略此次滚动事件

### Requirement: URL 状态同步

页面切换时系统 SHALL 使用 `pushState` 更新浏览器 URL，支持分享和前进/后退。

#### Scenario: 加载新页面更新 URL
- **WHEN** 前端加载第 2 页完成
- **THEN** URL 更新为 `/?page=2`

#### Scenario: 浏览器前进/后退
- **WHEN** 用户点击浏览器后退按钮
- **THEN** 页面回退到上一页状态（由浏览器默认行为处理）

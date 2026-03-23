## Why

Web UI 的任务列表当前会一次性加载所有部署任务。当部署记录增多时，页面加载变慢，DOM 节点过多导致浏览器性能下降。需要添加分页功能改善用户体验。

## What Changes

- 新增 `GET /?page=N` URL 参数，支持服务器端渲染指定页面
- 新增 `GET /api/deployments?page=N` JSON API，返回渲染好的 HTML 片段
- 列表页增加无限滚动：滚动到底部时自动加载下一页
- URL 使用 `pushState` 更新，支持浏览器前进/后退
- 每页固定 20 条记录

## Capabilities

### New Capabilities

- `webui-list-pagination`: 分页功能，包括服务器端渲染和前端无限滚动加载

### Modified Capabilities

- (无)

## Impact

- 修改 `src/database/mod.rs` - 添加分页查询方法
- 修改 `src/deploy/manager.rs` - 添加分页调用
- 修改 `src/web_ui/mod.rs` - 添加 page 参数解析和 JSON API
- 修改 `templates/list.html` - 添加无限滚动 JS 和分页控件

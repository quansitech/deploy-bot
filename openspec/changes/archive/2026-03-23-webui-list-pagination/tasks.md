## 1. Database Layer

- [x] 1.1 在 `src/database/mod.rs` 添加 `get_deployments_paginated(page, page_size)` 方法
- [x] 1.2 在 `src/database/mod.rs` 添加 `get_deployments_count()` 方法（用于判断是否有更多数据）
- [x] 1.3 添加单元测试验证分页逻辑

## 2. Manager Layer

- [x] 2.1 在 `src/deploy/manager.rs` 添加 `get_deployments_paginated(page, page_size)` 方法
- [x] 2.2 合并 queue + db 结果后按 created_at DESC 排序并做分页切片

## 3. Web UI Handler

- [x] 3.1 在 `src/web_ui/mod.rs` 修改 `list_deployments()` 支持 `page` query 参数
- [x] 3.2 添加 `deployments_api()` 处理 `GET /api/deployments?page=N`，返回 HTML 片段
- [x] 3.3 注册新路由 `/api/deployments` 到 router

## 4. Template & Frontend

- [x] 4.1 修改 `templates/list.html` 添加 JS 无限滚动逻辑
- [x] 4.2 实现 `nearBottom()` 检测函数
- [x] 4.3 实现 `loadNextPage()` fetch 并 append
- [x] 4.4 添加 `loading` 锁和 `exhausted` 标记防止重复请求
- [x] 4.5 实现 `history.pushState` / `replaceState` 更新 URL

## 5. Integration

- [x] 5.1 运行 `cargo clippy -- -D warnings` 确保无警告
- [x] 5.2 运行 `cargo test` 确保所有测试通过
- [ ] 5.3 手动测试：访问 `/?page=1`、`/?page=2`、滚动加载

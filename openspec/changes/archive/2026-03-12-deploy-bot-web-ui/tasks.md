## 1. 依赖配置

- [x] 1.1 添加依赖：rusqlite (SQLite 驱动)
- [x] 1.2 添加依赖：tokio-tungstenite (WebSocket)
- [x] 1.3 添加依赖：askama (模板引擎)
- [x] 1.4 添加依赖：askama-axum (Axum 集成)

## 2. 数据库模块

- [x] 2.1 创建 database 模块 (src/database/mod.rs)
- [x] 2.2 实现数据库初始化函数
- [x] 2.3 创建 deployments 表的 CRUD 操作
- [x] 2.4 创建 deployment_logs 表的 CRUD 操作
- [x] 2.5 添加单元测试

## 3. 重构 DeploymentManager

- [x] 3.1 修改 DeploymentManager 使用 SQLite 持久化
- [x] 3.2 实现队列任务和数据库的同步
- [x] 3.3 添加日志写入方法
- [x] 3.4 添加删除任务方法
- [x] 3.5 添加重试任务方法
- [x] 3.6 添加单元测试

## 4. WebSocket 支持

- [x] 4.1 创建 WebSocket 处理模块
- [x] 4.2 实现 /ws/deploy/:id 路由
- [x] 4.3 实现日志实时推送
- [x] 4.4 处理连接关闭

## 5. HTML 模板

- [x] 5.1 创建 templates 目录
- [x] 5.2 实现任务列表模板 (list.html)
- [x] 5.3 实现任务详情模板 (detail.html)
- [x] 5.4 添加简约 CSS 样式
- [x] 5.5 实现 JavaScript WebSocket 客户端

## 6. Web 路由

- [x] 6.1 创建 web_ui 模块
- [x] 6.2 实现 GET / 路由（任务列表）
- [x] 6.3 实现 GET /deploy/:id 路由（任务详情）
- [x] 6.4 实现 POST /deploy/:id/delete 路由
- [x] 6.5 实现 POST /deploy/:id/retry 路由

## 7. 集成与测试

- [x] 7.1 在 main.rs 中注册新模块和路由
- [x] 7.2 配置 askama 模板路径
- [x] 7.3 运行 cargo clippy 检查
- [x] 7.4 运行 cargo test 验证
- [ ] 7.5 手动测试各功能

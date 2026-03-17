## 1. 项目基础设置

- [x] 1.1 初始化 Rust 项目结构，添加 Cargo.toml 依赖（axum, tokio, serde, git2, tracing 等）
- [x] 1.2 配置日志系统（tracing）和错误处理
- [x] 1.3 创建配置文件格式（YAML）和配置加载模块
- [x] 1.4 搭建基础 HTTP 服务器骨架

## 2. Webhook API 实现

- [x] 2.1 实现 Webhook 端点 `/webhook/{project_name}`
- [x] 2.2 添加 Token 验证中间件
- [x] 2.3 实现 GitHub Webhook 签名验证
- [x] 2.4 实现 GitLab Webhook 签名验证

## 3. Git 操作模块

- [x] 3.1 实现 Git 仓库拉取功能（使用 git2 库）
- [x] 3.2 添加私有仓库 SSH/Token 认证支持
- [x] 3.3 实现分支/Tag 检出功能
- [x] 3.4 添加 Git 操作超时控制

## 4. 依赖安装模块

- [x] 4.1 实现项目类型自动识别（package.json, Cargo.toml, requirements.txt, composer.json 等）
- [x] 4.2 实现 Node.js 依赖安装（npm/yarn）
- [x] 4.3 实现 Rust 依赖获取（cargo fetch）
- [x] 4.4 实现 Python 依赖安装（pip/poetry）
- [x] 4.5 实现 PHP 依赖安装（composer install）
- [x] 4.6 支持自定义安装命令配置

## 5. 构建运行模块

- [x] 5.1 实现项目构建执行器
- [x] 5.2 支持自定义构建命令配置
- [x] 5.3 添加构建超时控制

## 6. 部署管理模块

- [x] 6.1 实现部署任务队列管理
- [x] 6.2 添加任务状态跟踪（pending/running/success/failed）
- [x] 6.3 实现部署日志记录（输出到文件）
- [x] 6.4 添加任务取消功能

## 7. 配置管理

- [x] 7.1 创建配置文件模板（config.yaml）
- [x] 7.2 实现项目配置管理
- [x] 7.3 添加项目配置 API（可选）

## 8. 测试与部署

- [x] 8.1 编写单元测试
- [x] 8.2 编写集成测试
- [x] 8.3 构建 release 版本
- [x] 8.4 部署文档和运行说明

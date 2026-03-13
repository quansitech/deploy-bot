## Why

当前项目部署依赖人工操作，流程繁琐且容易出错。需要一个基于 Rust 的自动部署机器人，通过 Webhook 接口触发自动化部署流程，提升部署效率和可靠性。

## What Changes

- 创建基于 Rust 的自动部署服务
- 提供 Webhook HTTP 接口接收部署触发请求
- 实现代码仓库拉取功能
- 实现依赖自动安装功能
- 实现项目构建功能

## Capabilities

### New Capabilities

- **webhook-api**: 提供 Webhook 接口接收外部触发请求，支持 GitHub/GitLab 等平台的 Webhook 格式
- **git-operations**: 实现 Git 代码仓库的自动拉取，支持私有仓库认证
- **dependency-installer**: 自动识别项目类型并安装依赖（npm/cargo/pip/composer 等）
- **build-runner**: 执行项目构建命令，支持自定义构建脚本
- **deployment-manager**: 管理部署任务队列，执行部署流程

### Modified Capabilities

- （无）

## Impact

- 新增 Rust HTTP 服务
- 新增 Webhook API 端点
- 新增部署任务执行模块
- 需配置 Git 凭证存储
- 需配置各类型项目的构建环境

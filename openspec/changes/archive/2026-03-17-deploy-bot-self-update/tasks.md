## 1. 配置项修改

- [x] 1.1 在 config.rs 中添加 `update_script` 配置字段
- [x] 1.2 修改 config.yaml 添加 `update_script` 配置示例

## 2. GitHub Actions Workflow

- [x] 2.1 创建 `.github/workflows/release.yml`
- [x] 2.2 实现 tag 触发构建逻辑
- [x] 2.3 实现二进制上传到 GitHub Releases
- [x] 2.4 实现 webhook 通知 deploy-bot

## 3. 自更新 API

- [x] 3.1 创建 `src/self_update.rs` 模块
- [x] 3.2 实现 `/webhook/update-self` API 端点
- [x] 3.3 实现 payload 解析（tag_name, browser_download_url）
- [x] 3.4 实现版本号比对逻辑（从 Cargo.toml 读取当前版本）

## 4. 下载和执行逻辑

- [x] 4.1 实现从 GitHub Releases 下载二进制
- [x] 4.2 实现 fork 子进程执行更新脚本
- [x] 4.3 实现更新脚本路径传递参数

## 5. 单元测试

- [x] 5.1 为自更新 API 编写单元测试
- [x] 5.2 为下载逻辑编写单元测试
- [x] 5.3 为版本号比对编写单元测试

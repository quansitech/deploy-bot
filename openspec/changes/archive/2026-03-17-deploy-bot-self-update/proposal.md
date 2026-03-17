## Why

deploy-bot 部署在局域网服务器上，无法从 GitHub Actions 直接部署。需要实现自更新机制：GitHub Actions 构建并发布二进制到 GitHub Releases，然后通知 deploy-bot 下载更新并重启服务。

## What Changes

1. 新增 GitHub Actions workflow：每次 push 到 main 时构建 release 并发布到 GitHub Releases
2. 新增 `/webhook/update-self` API：接收自更新通知，携带版本信息和下载链接
3. 新增配置项 `update_script`：指定更新脚本路径
4. 实现自更新逻辑：下载新二进制 → 执行更新脚本 → 重启服务

## Capabilities

### New Capabilities
- `self-update`: 实现 deploy-bot 自更新功能，包括 webhook API、下载逻辑和脚本执行

### Modified Capabilities
- (无)

## Impact

- 新增 `src/self_update.rs` 模块
- 新增 `.github/workflows/release.yml` workflow 文件
- 修改 `config.yaml` 添加 `update_script` 配置项

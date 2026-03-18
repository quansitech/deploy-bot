## Why

GitHub 在中国大陆地区网络不稳定，导致 self-update 功能经常无法正常下载二进制文件，影响服务部署和更新。

## What Changes

- 在 `ServerConfig` 中添加 `github_mirror` 可选配置项
- 修改 `download_binary` 函数，支持根据配置添加镜像前缀
- 仅对包含 `github.com` 的 URL 应用镜像转换
- 前缀方式：直接在原 URL 前拼接镜像地址

## Capabilities

### New Capabilities

- `github-mirror`: 支持配置 GitHub 镜像地址，在 self-update 下载二进制文件时自动添加镜像前缀

### Modified Capabilities

无

## Impact

- 配置文件：`config.yaml` 新增 `github_mirror` 配置项
- 代码修改：`src/config/mod.rs`、`src/self_update.rs`

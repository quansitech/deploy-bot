## Context

当前 self-update 功能直接从 GitHub releases 下载二进制文件。由于 GitHub 在中国大陆地区网络不稳定，下载经常失败，影响服务的正常更新。

## Goals / Non-Goals

**Goals:**
- 支持在配置文件中设置 GitHub 镜像地址
- 当配置镜像后，下载 GitHub 二进制文件时自动添加镜像前缀

**Non-Goals:**
- 不考虑其他 GitHub 访问场景（如 git clone）
- 不实现重试逻辑或备用镜像切换

## Decisions

### 1. 镜像应用方式：路径前缀拼接

**决定:** 使用路径前缀方式拼接镜像地址

```
原 URL:  https://github.com/owner/repo/releases/download/v1.0.0/deploy-bot
镜像:    https://ghproxy.com/
结果:    https://ghproxy.com/https://github.com/owner/repo/releases/download/v1.0.0/deploy-bot
```

**备选方案:**
- 域名替换: `github.com` → `mirror.ghproxy.com` - 需要更复杂的 URL 解析和重构

**理由:** 路径前缀方式实现简单，只需字符串拼接即可，无需解析 URL 结构。

### 2. 镜像应用条件：仅对 github.com 域名生效

**决定:** 仅当 URL 包含 `github.com` 时才应用镜像

**理由:**
- 避免误转换非 GitHub 来源的 URL
- 保持配置的简单性

### 3. 配置结构

在 `ServerConfig` 中添加可选字段：

```toml
[server]
github_mirror = "https://ghproxy.com/"  # 可选，不填则不使用镜像
```

**理由:** 与现有 `update_script`、`update_webhook_secret` 等可选配置保持一致的风格。

## Risks / Trade-offs

- **风险:** 用户配置的镜像地址格式不正确
  **缓解:** 文档说明镜像地址格式要求（以 `/` 结尾）

- **风险:** 镜像服务本身不可用
  **缓解:** 用户可选择不配置镜像，回退到直连 GitHub

## Migration Plan

1. 部署新版本后，在 `config.yaml` 中添加 `github_mirror` 配置（如需要）
2. 重启服务生效
3. 回滚：移除配置或恢复旧版本即可

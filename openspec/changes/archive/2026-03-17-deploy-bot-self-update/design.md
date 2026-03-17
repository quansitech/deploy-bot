## Context

deploy-bot 部署在局域网服务器上，无法从 GitHub Actions 直接部署。需要实现自更新机制：GitHub Actions 构建发布二进制到 GitHub Releases，然后通知 deploy-bot 下载更新并重启服务。

当前架构：
- deploy-bot 通过 Webhook 接收 GitHub/GitLab/Codeup 推送事件触发部署
- 配置文件为 `config.yaml`，项目配置为 `.deploy.yaml`

## Goals / Non-Goals

**Goals:**
- 实现 GitHub Actions 自动构建并发布二进制到 Releases
- 实现 deploy-bot 自更新 webhook API
- 实现下载新版本、执行更新脚本、重启服务的完整流程

**Non-Goals:**
- 不实现回滚机制（更新失败需手动恢复）
- 不实现增量更新（每次全量替换二进制）
- 不支持 Windows 平台

## Decisions

### D1: GitHub Actions 触发方式
**决定**: 使用 `release` 事件触发工作流

```yaml
on:
  push:
    tags:
      - 'v*'
```
每次 push 匹配 `v*` 的 tag 时触发构建。

**备选方案**:
- `workflow_dispatch`: 手动触发（需要手动操作）
- push 到 main 分支自动创建 tag（复杂度高）

### D2: Webhook Payload 传递方式
**决定**: GitHub Actions 在创建 Release 后发送 webhook 请求，payload 包含 `tag_name` 和 `browser_download_url`

理由：deploy-bot 无需自行构造下载 URL，直接使用 GitHub 提供的下载链接。

### D3: 更新脚本执行方式
**决定**: deploy-bot fork 子进程执行更新脚本

```
deploy-bot (父进程)
  |
  ├── 下载新 binary 到 /tmp/
  └── fork() -> 子进程执行 update.sh
        |
        ├── systemctl stop deploy-bot
        ├── cp new_binary -> old_binary
        ├── chmod +x
        └── systemctl start deploy-bot
```

理由：子进程可以独立停止父进程，不会有"自己杀死自己"的问题。

### D4: 配置项设计
**决定**: 在 `config.yaml` 新增 `update_script` 字段

```yaml
[server]
host = "0.0.0.0"
port = 8080
update_script = "/opt/deploy-bot/update.sh"  # 新增
```

理由：灵活配置更新脚本位置，适应不同部署环境。

### D5: 下载临时文件管理
**决定**: 下载到 `/tmp/deploy-bot-{version}`，更新完成后删除

理由：避免污染工作目录，更新失败可重试。

### D6: 版本号比对机制
**决定**: deploy-bot 从 Cargo.toml 读取当前版本号，与 webhook 收到的版本号比对，只有新版本号更大时才执行更新

版本号格式：`v{major}.{minor}.{patch}`，使用语义版本比对逻辑：
- 提取 major、minor、patch 数字
- 逐级比较：major > minor > patch
- 如果新版本不大于当前版本，返回成功但不执行更新

理由：避免重复下载和更新已运行的相同或更低版本。

## Risks / Trade-offs

| 风险 |  Mitigation |
|------|-------------|
| 更新过程中服务中断 | 短暂中断（1-3秒），可接受 |
| 更新失败无法启动 | 保留旧版本 backup，更新前 cp 备份 |
| 网络下载失败 | 重试机制（可配置重试次数） |
| 二进制被篡改 | 可选：校验 SHA256（后续迭代） |

## Migration Plan

1. **部署前准备**
   - 在服务器创建 `update.sh` 脚本
   - 配置 `config.yaml` 添加 `update_script` 路径

2. **发布流程**
   - 开发完成后，push 代码并打 tag: `git tag v0.2.0 && git push --tags`
   - GitHub Actions 自动构建并发布到 Releases
   - Actions 发送 webhook 通知 deploy-bot

3. **deploy-bot 接收通知**
   - 解析 payload 获取版本号和下载链接
   - **比对版本号**：从 Cargo.toml 读取当前版本，只有新版本 > 当前版本才继续
   - 下载新 binary 到临时目录
   - fork 子进程执行 update.sh
   - update.sh 停止旧进程、替换二进制、启动新进程

## Open Questions

1. **是否需要校验 SHA256？** 当前版本不实现，后续可迭代
2. **回滚机制？** 暂时手动处理，更新失败后 ssh 登录恢复

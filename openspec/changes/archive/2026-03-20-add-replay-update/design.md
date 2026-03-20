## Context

自更新功能通过 GitHub webhook 触发，收到 payload 后执行版本检查、下载二进制、执行更新脚本的完整流程。

当前问题：开发者修复 bug 后，无法方便地验证自更新脚本是否正常工作。每次测试都需要发布一个假的 GitHub release，效率低下。

约束：
- 更新脚本路径从 config.yaml 的 `update_script` 读取
- Webhook secret 验证机制保持不变
- 二进制路径通过 `std::env::current_exe()` 获取

## Goals / Non-Goals

**Goals:**
- 支持在本地重放上一次的 webhook payload
- Payload 自动持久化，无需手动构造
- 提供独立的 CLI 命令，不依赖 webhook

**Non-Goals:**
- 不修改现有的 webhook 处理逻辑
- 不添加自动版本检查/轮询机制
- 不支持任意的 payload 重放（只重放最后一次）

## Decisions

### 1. Payload 持久化路径

**决定**: `{binary_dir}/.deploy-last-payload/deploy-bot-last-update.json`

**理由**:
- 与二进制文件同目录，便于管理
- 使用 `.deploy-last-payload` 子目录避免污染主目录
- 隐藏目录 (`_` 前缀) 在 Unix 系统中表示隐藏

**替代方案考虑**:
- `~/.config/deploy-bot/last-payload.json`: 用户目录，跨版本共享，但路径不直观
- `/tmp/deploy-bot-last-payload.json`: 临时文件，可能丢失

### 2. Payload 保存时机

**决定**: 在版本检查之后、保存/下载之前保存 payload

**理由**:
- 即使版本检查失败（已是最新版本），payload 也会被保存
- 开发者可以在任何时候重放，包括跳过版本检查

### 3. CLI 命令 `--force` 参数

**决定**: 使用 `--force` 参数跳过版本检查

**理由**:
- `replay-update` 命令本身就表明意图是重放
- `--force` 是标准约定，表达"强制执行"
- 命令全称: `deploy-bot replay-update --force`

### 4. 复用现有逻辑

**决定**: 重放时复用 `handle_self_update` 的下载和执行逻辑

**理由**:
- 减少代码重复
- 确保测试场景与真实更新完全一致
- 唯一区别：跳过版本检查

## Risks / Trade-offs

| 风险 |  Mitigation |
|------|-------------|
| Payload 文件损坏或不存在 | CLI 命令返回明确错误信息 |
| replay 时网络下载失败 | 复用现有错误处理，日志记录 |
| Payload 保存失败 | 继续执行更新流程，只记录警告 |

## Migration Plan

1. **开发阶段**: 添加 payload 持久化和 replay 命令
2. **测试**: 使用 `replay-update --force` 验证更新脚本
3. **无需数据迁移**: 全新功能，无历史数据

## Open Questions

- 无

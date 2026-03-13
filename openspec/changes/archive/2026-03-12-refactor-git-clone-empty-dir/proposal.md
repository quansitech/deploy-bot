## Why

当前部署流程存在逻辑矛盾：项目目录必须存在才能存放 `.deploy.yaml`，但如果目录不存在就无法通过 `git clone` 拉取代码。这导致首次部署时无法自动化完成。

## What Changes

1. 修改 Git 仓库拉取逻辑，支持空目录场景
2. 新增目录状态判断：检查项目目录是否为空（仅包含 `.deploy.yaml`）
3. 空目录时使用 `git clone <repo> .` 克隆到当前目录
4. 非空目录时保持现有的 `git fetch && checkout` 逻辑

## Capabilities

### New Capabilities

- `git-clone-empty-dir`: 支持空目录场景的 Git 克隆能力

## Impact

- 修改 `src/git/operations.rs` 中的 `pull_repo` 函数
- 影响 `src/deploy/executor.rs` 的部署执行流程

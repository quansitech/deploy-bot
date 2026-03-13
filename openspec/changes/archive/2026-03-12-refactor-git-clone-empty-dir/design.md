## Context

当前部署流程中，Git 仓库拉取逻辑 (`src/git/operations.rs`) 假设项目目录要么不存在（执行 clone），要么已存在且为有效的 Git 仓库（执行 fetch）。但由于项目目录必须存在以存放 `.deploy.yaml`，导致无法触发 clone 逻辑。

## Goals / Non-Goals

**Goals:**
- 支持空目录场景的 Git 克隆
- 保持现有非空目录的 fetch && checkout 逻辑不变
- 不改变 `.deploy.yaml` 配置文件的位置要求

**Non-Goals:**
- 不修改 webhook 验证逻辑
- 不修改部署任务队列逻辑

## Decisions

### 1. 目录状态判断方式

**方案 A**: 读取目录文件列表，排除 `.deploy.yaml` 后判断是否为空
- 优点：实现简单
- 缺点：需要读取文件系统

**方案 B**: 检查是否存在 `.git` 目录
- 优点：判断更准确（空目录一定没有 .git）
- 缺点：逻辑与原方案差异较大

**选择**: 方案 A - 通过读取目录文件列表判断，更直观明确

### 2. clone 到当前目录的实现

使用 `git clone --branch <branch> --depth 1 <repo_url> .` 命令：
- `--branch`: 指定分支
- `--depth 1`: 浅克隆，只拉取最新提交，加快速度
- `.`: 克隆到当前目录

### 3. 函数签名变更

修改 `pull_repo` 函数，增加 `is_empty_dir` 参数或内部自行判断：
- 方案 A: 新增参数 `force_clone: bool`
- 方案 B: 内部自动判断

**选择**: 方案 B - 内部自动判断，减少调用方复杂度

## Risks / Trade-offs

- [风险] 目录中存在隐藏文件导致判断不准确 → 忽略以 `.` 开头的隐藏文件
- [风险] clone 失败后目录状态不确定 → clone 失败时不修改目录，保持原状

## Context

当前系统接收到 webhook 请求后会立即将部署任务加入队列，没有任何重复检测机制。如果用户在短时间内多次 push 代码，会导致同一项目创建多个部署任务。

## Goals / Non-Goals

**Goals:**
- 在任务入队前检测重复任务
- 避免同一项目重复部署

**Non-Goals:**
- 不处理跨分支的部署冲突
- 不提供任务优先级机制

## Decisions

### 去重判断标准
**决定**: 使用 `project_name + branch` 作为去重判断条件

**理由**:
- 同一项目的不同分支通常可以独立部署
- 与 GitHub/GitLab webhook 的事件粒度一致

### 重复任务判定状态
**决定**: 仅当队列中存在 `Pending` 或 `Running` 状态的任务时才跳过

**理由**:
- `Success`/`Failed`/`Cancelled` 状态的任务已完成，不影响新部署
- 正在执行的任务如果被跳过，可能导致部署中断

### 返回值设计
**决定**: `queue_deployment` 返回 `Option<String>`

**理由**:
- `Some(id)`: 成功入队
- `None`: 跳过重复任务

## Risks / Trade-offs

| 风险 | 解决方案 |
|------|---------|
| 长时间 Running 状态的任务阻塞新部署 | 用户可通过取消超时的任务来解决 |
| 不同分支部署需求被误判为重复 | 使用 project_name + branch 组合避免 |

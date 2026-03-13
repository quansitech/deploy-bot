## Context

当前部署流程中，只有在项目配置明确提供 `install_command` 时才会执行依赖安装。如果用户未配置，则跳过安装步骤，导致项目无法正常使用。

## Goals / Non-Goals

**Goals:**
- 为每种项目类型提供默认安装命令
- 当未配置 `install_command` 时自动使用默认命令
- 保持向后兼容，自定义命令优先

**Non-Goals:**
- 不修改 `build_command` 的逻辑（仅处理 install）
- 不添加默认的 build 命令

## Decisions

### 1. 默认安装命令映射

| 项目类型 | 默认安装命令 |
|---------|-------------|
| nodejs | `npm install` |
| php | `composer install` |
| python | `pip install -r requirements.txt` |
| rust | 无（跳过安装，cargo build 已包含依赖安装） |
| custom | 无（保持原逻辑） |

### 2. 实现位置

在 `src/installer/tasks.rs` 中新增 `get_default_install_command` 函数，根据项目类型返回默认命令。

在 `execute_deployment` 中，当 `install_command` 为 `None` 时，调用该函数获取默认命令。

### 3. 逻辑变更

修改 `executor.rs` 中的安装步骤逻辑：
- 原逻辑：`if let Some(install_cmd) = ...`
- 新逻辑：获取项目类型的默认命令（如果存在），然后执行安装

## Risks / Trade-offs

- [风险] requirements.txt 不存在时 pip install 失败 → 失败时报错
- [风险] composer.json 不存在时 composer install 失败 → 失败时报错

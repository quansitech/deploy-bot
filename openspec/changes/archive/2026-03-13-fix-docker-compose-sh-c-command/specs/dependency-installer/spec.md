## ADDED Requirements

### Requirement: Docker 环境下的命令执行
系统 SHALL 使用 sh -c 包装自定义命令，确保包含 Shell 语法的复合命令能正常执行。

#### Scenario: 单条命令执行
- **WHEN** 用户配置单条命令（如 `composer install`）并通过 Docker 执行时
- **THEN** 命令正常执行

#### Scenario: 复合命令执行
- **WHEN** 用户配置包含 && 的复合命令（如 `composer install && npm i`）并通过 Docker 执行时
- **THEN** 命令正常执行，所有操作按顺序执行

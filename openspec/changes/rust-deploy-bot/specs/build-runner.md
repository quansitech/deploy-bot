## ADDED Requirements

### Requirement: 项目构建执行
系统 SHALL 执行项目的构建命令。

#### Scenario: Node.js 项目构建
- **WHEN** 项目为 Node.js 类型且配置了 build_script
- **THEN** 执行 npm run build 或配置的构建命令

#### Scenario: Rust 项目构建
- **WHEN** 项目为 Rust 类型
- **THEN** 执行 cargo build --release

#### Scenario: Python 项目打包
- **WHEN** 项目为 Python 类型且配置了打包命令
- **THEN** 执行对应的打包命令（如 python setup.py bdist_wheel）

#### Scenario: PHP 项目构建
- **WHEN** 项目为 PHP 类型且配置了 build_command
- **THEN** 执行 composer install 和配置的构建命令（如 php artisan migrate）

#### Scenario: 无需构建
- **WHEN** 项目配置指定无需构建（如静态网站）
- **THEN** 跳过构建步骤

### Requirement: 自定义构建命令
系统 SHALL 支持在项目配置中指定自定义的构建命令。

#### Scenario: 自定义构建命令执行
- **WHEN** 项目配置中指定了 build_command
- **THEN** 使用配置的命令执行构建

### Requirement: 构建超时控制
系统 SHALL 对构建操作设置超时限制。

#### Scenario: 构建超时
- **WHEN** 构建操作超过配置的超时时间（默认60分钟）
- **THEN** 中断构建并标记部署失败

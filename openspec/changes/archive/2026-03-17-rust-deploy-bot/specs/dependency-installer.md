## ADDED Requirements

### Requirement: 项目类型自动识别
系统 SHALL 自动识别项目类型（Node.js/Rust/Python/PHP等）并选择合适的依赖安装命令。

#### Scenario: Node.js 项目
- **WHEN** 项目根目录存在 package.json
- **THEN** 执行 npm install 或 yarn install

#### Scenario: Rust 项目
- **WHEN** 项目根目录存在 Cargo.toml
- **THEN** 执行 cargo fetch

#### Scenario: Python 项目
- **WHEN** 项目根目录存在 requirements.txt 或 pyproject.toml
- **THEN** 执行 pip install -r requirements.txt 或 poetry install

#### Scenario: PHP 项目
- **WHEN** 项目根目录存在 composer.json
- **THEN** 执行 composer install

#### Scenario: PHP 项目（使用 artisan）
- **WHEN** 项目根目录存在 artisan 文件
- **THEN** 执行 composer install

### Requirement: 自定义安装命令
系统 SHALL 支持在项目配置中指定自定义的依赖安装命令。

#### Scenario: 自定义安装命令执行
- **WHEN** 项目配置中指定了 install_command
- **THEN** 使用配置的命令替代默认行为

### Requirement: 依赖安装超时
系统 SHALL 对依赖安装操作设置超时限制。

#### Scenario: 依赖安装超时
- **WHEN** 依赖安装超过配置的超时时间（默认30分钟）
- **THEN** 中断操作并标记部署失败

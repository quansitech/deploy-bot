## Why

当用户配置项目时，如果没有指定 `install_command`，部署流程会跳过依赖安装步骤。这导致首次部署或更新依赖时需要手动执行安装命令，增加了使用复杂度。

## What Changes

1. 为每种项目类型添加默认的安装命令
2. 当 `install_command` 未配置时，自动使用项目类型对应的默认命令
3. Node.js: `npm install`
4. PHP: `composer install`
5. Python: `pip install -r requirements.txt`
6. Rust: 无（跳过安装，cargo build 已包含依赖安装）
7. 自定义项目类型不执行默认安装

## Capabilities

### New Capabilities

- `default-install-commands`: 根据项目类型自动执行默认安装命令

## Impact

- 修改 `src/installer/tasks.rs` 中的安装逻辑
- 影响 `src/deploy/executor.rs` 的部署执行流程

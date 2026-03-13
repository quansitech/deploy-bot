## Why

当前所有项目配置集中在 deploy-bot 的 config.yaml 中，由运维统一管理。当需要修改项目的构建配置（如 PHP 项目需要修改 composer install 命令）时，必须修改 deploy-bot 的配置文件并重启服务，不够灵活。

将配置下沉到各项目目录，让项目成员可以自行管理 `.deploy.yaml` 配置文件，实现自管理。

## What Changes

- 修改 config.yaml 结构，只保留 server 全局配置
- 新增项目级 `.deploy.yaml` 配置文件支持
- 修改部署流程：从项目目录读取 .deploy.yaml 获取配置

## Impact

- 修改 config.yaml 结构（移除 projects 配置）
- 修改 webhook 处理逻辑
- 新增 .deploy.yaml 解析逻辑
- 修改部署管理器加载配置的逻辑

## ADDED Requirements

### Requirement: 配置更新脚本路径
config.yaml 中可配置 `update_script` 字段，指定自更新时执行的脚本路径。该字段为可选配置，未配置时自更新功能不可用。

#### Scenario: 配置有效的更新脚本
- **GIVEN** config.yaml 中 `update_script` 设置为 `/opt/deploy-bot/update.sh`
- **WHEN** deploy-bot 启动
- **THEN** 自更新功能可用

#### Scenario: 未配置更新脚本
- **GIVEN** config.yaml 中未设置 `update_script`
- **WHEN** deploy-bot 启动
- **THEN** 自更新功能不可用，收到更新 webhook 时返回错误

### Requirement: 接收自更新 webhook 通知
deploy-bot 监听 `/webhook/update-self` 端点，接收 GitHub Actions 的自更新通知。

#### Scenario: 收到有效更新通知
- **GIVEN** deploy-bot 正在运行，且已配置 `update_script`
- **WHEN** 收到 POST `/webhook/update-self` 请求，payload 包含 `tag_name` 和 `browser_download_url`
- **AND** 新版本号大于当前版本号
- **THEN** 开始下载新版本二进制
- **AND** 下载完成后 fork 子进程执行更新脚本

#### Scenario: 收到无效 payload
- **GIVEN** deploy-bot 正在运行
- **WHEN** 收到 POST `/webhook/update-self` 请求，但 payload 缺少 `tag_name` 或 `browser_download_url`
- **AND** 返回 400 错误，错误信息说明缺少必要字段

#### Scenario: 未配置 update_script 时收到通知
- **GIVEN** config.yaml 中未配置 `update_script`
- **WHEN** 收到 POST `/webhook/update-self` 请求
- **AND** 返回 400 错误，提示未配置更新脚本

#### Scenario: 版本号不小于当前版本
- **GIVEN** deploy-bot 当前运行版本为 v0.2.0
- **WHEN** 收到更新通知，tag_name 为 v0.2.0 或 v0.1.0
- **THEN** 返回成功响应，但不执行下载和更新
- **AND** 日志记录跳过更新的原因

### Requirement: 版本号比对
deploy-bot 从 Cargo.toml 读取当前版本号，与 webhook 收到的版本号进行语义版本比对。只有新版本号大于当前版本号时才执行更新。

#### Scenario: 新版本大于当前版本
- **GIVEN** deploy-bot 当前版本为 v0.2.0
- **WHEN** 收到更新通知，tag_name 为 v0.3.0
- **THEN** 版本比对通过，执行更新流程

#### Scenario: 当前版本等于新版本
- **GIVEN** deploy-bot 当前版本为 v0.2.0
- **WHEN** 收到更新通知，tag_name 为 v0.2.0
- **THEN** 跳过更新，返回成功响应

#### Scenario: 新版本小于当前版本
- **GIVEN** deploy-bot 当前版本为 v0.2.0
- **WHEN** 收到更新通知，tag_name 为 v0.1.0
- **THEN** 跳过更新，返回成功响应

### Requirement: 下载新版本二进制
deploy-bot 从 GitHub Releases 下载新版本二进制文件到临时目录。

#### Scenario: 下载成功
- **WHEN** 开始下载新版本二进制
- **AND** 下载过程正常完成
- **THEN** 二进制保存在 `/tmp/deploy-bot-{version}`

#### Scenario: 下载失败
- **WHEN** 下载过程中网络错误
- **AND** 返回错误信息，记录日志
- **AND** 不执行更新脚本

### Requirement: 执行更新脚本
下载完成后，deploy-bot fork 子进程执行配置好的更新脚本，传递新版本路径作为参数。

#### Scenario: 更新脚本执行成功
- **GIVEN** 新版本二进制已下载到 `/tmp/deploy-bot-v0.2.0`
- **WHEN** fork 子进程执行 `/opt/deploy-bot/update.sh /tmp/deploy-bot-v0.2.0`
- **AND** 脚本执行返回码为 0
- **THEN** 更新成功，日志记录完成

#### Scenario: 更新脚本执行失败
- **GIVEN** 新版本二进制已下载到 `/tmp/deploy-bot-v0.2.0`
- **WHEN** fork 子进程执行更新脚本
- **AND** 脚本执行返回码非 0
- **THEN** 更新失败，日志记录错误
- **AND** 保留旧版本服务运行

### Requirement: GitHub Actions 自动发布
每次 push 匹配 `v*` 的 tag 时，GitHub Actions 自动构建并发布二进制到 GitHub Releases，然后发送 webhook 通知 deploy-bot。

#### Scenario: push tag 触发发布
- **WHEN** push tag `v0.2.0` 到仓库
- **AND** GitHub Actions 构建成功
- **THEN** 创建 GitHub Release `v0.2.0`
- **AND** 上传构建的二进制文件
- **AND** 发送 POST 请求到 deploy-bot 的 `/webhook/update-self`

#### Scenario: 构建失败
- **WHEN** push tag 后 GitHub Actions 构建失败
- **THEN** 不创建 Release
- **AND** 不发送 webhook 通知

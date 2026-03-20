# Deploy Bot

基于 Rust 的自动部署服务，通过 Webhook 接口触发自动化部署流程。

## 功能特性

- **Webhook 接口**：支持 GitHub、GitLab、阿里云 Codeup Webhook 签名验证
- **自动部署**：代码拉取、依赖安装、项目构建自动化
- **任务队列**：部署任务排队机制，支持去重
- **多语言支持**：Node.js、Rust、Python、PHP
- **Docker 支持**：支持 Docker Compose 部署
- **自定义命令**：支持自定义安装和构建命令
- **自更新**：支持 GitHub Releases 自动更新

## 快速开始

### 1. 配置服务

编辑 `config.yaml`：

```yaml
[server]
host = "0.0.0.0"
port = 8080
workspace_dir = "./workspace"
docker_compose_path = "./docker-compose.yaml"

# 可选：Webhook 平台验证
# github_secret = "your-github-secret"
# gitlab_token = "your-gitlab-token"
# codeup_token = "your-codeup-token"
```

## 服务配置说明 (config.yaml)

| 字段 | 必填 | 说明 |
|------|------|------|
| host | 是 | 服务监听地址 |
| port | 是 | 服务监听端口 |
| workspace_dir | 是 | 工作空间目录（存放项目代码） |
| docker_compose_path | 否 | Docker Compose 文件路径 |
| github_secret | 否 | GitHub Webhook 签名密钥 |
| gitlab_token | 否 | GitLab Webhook Token |
| codeup_token | 否 | 阿里云 Codeup Webhook Token |
| update_script | 否 | 自更新脚本路径 |
| update_webhook_secret | 否 | 自更新 Webhook 验证密钥 |
| github_mirror | 否 | GitHub 镜像地址（用于自更新下载加速） |

### 字段详细说明

#### docker_compose_path
Docker Compose 文件路径。设置后，支持使用 Docker 容器执行安装和构建命令。

**支持两种格式：**

```yaml
# 单个文件（字符串）
docker_compose_path = "./docker-compose.yaml"

# 多个文件（数组），按顺序覆盖配置
docker_compose_path = ["./docker-compose.yaml", "./docker-compose.override.yaml"]

# 不使用 Docker，在宿主机执行
# docker_compose_path = null
```

**多文件使用场景：**
- 基础配置文件 + 环境覆盖配置
- 例如：`["./docker-compose.yaml", "./docker-compose.prod.yaml"]`

**生成的 Docker 命令示例：**
```bash
# 单文件
docker compose -f ./docker-compose.yaml run --rm php sh -c "命令"

# 多文件
docker compose -f ./docker-compose.yaml -f ./docker-compose.override.yaml run --rm php sh -c "命令"
```

**使用条件：**
1. 服务器已安装 Docker
2. Docker Compose 文件存在（通常为 `docker-compose.yaml` 或 `docker-compose.yml`）
3. 项目配置中指定了 `docker_service`

**自动检测：**
程序启动时会自动检测可用的 Docker Compose 命令：
- 优先使用 `docker compose`（Docker 19.03+ 子命令）
- 若不可用，则使用 `docker-compose`（旧版本独立命令）

这确保了与不同版本 Docker 的兼容性。

#### github_secret
GitHub Webhook 签名验证密钥。当仓库配置 Webhook 时，需要设置此值来验证请求来源。

#### gitlab_token
GitLab Webhook Token。用于验证 GitLab Webhook 请求。

#### codeup_token
阿里云 Codeup Webhook Token。用于验证阿里云 Codeup Webhook 请求。

### 2. 配置项目

在项目工作目录（如 `./workspace/my-project/`）下创建 `.deploy.yaml`：

```yaml
repo_url = "https://github.com/username/repo.git"
branch = "main"
project_type = "nodejs"
# docker_service = "php"        # 可选：使用 Docker 容器执行命令
# working_dir = "/app"          # 可选：命令执行目录
# run_user = "www-data"         # 可选：运行命令的用户
# install_command = "npm install"
# build_command = "npm run build"
# env = { NODE_ENV = "production" }
# restart_service = "web"       # 可选：部署完成后重启 Docker 服务
# docker_compose_path = ["./docker-compose.yaml", "./docker-compose.override.yaml"]  # 可选：覆盖 config.yaml 中的 Docker Compose 配置
```

### 3. 启动服务

```bash
# 开发模式
cargo run

# 生产模式
cargo build --release
./target/release/deploy-bot
```

### 3.2 安装为系统服务

将 deploy-bot 安装为系统守护进程，实现开机自启和进程管理。

#### 标准目录布局

```
/opt/deploy-bot/
├── deploy-bot       # 二进制文件
├── config.yaml       # 配置文件
└── logs/             # 日志目录（SysV init 使用）
```

**首次安装时需创建目录：**
```bash
sudo mkdir -p /opt/deploy-bot/{logs}
```

#### systemd (Ubuntu 15.04+ / Debian 8+)

1. 复制 unit 文件并重载：
   ```bash
   sudo cp scripts/deploy-bot.service /etc/systemd/system/
   sudo systemctl daemon-reload
   ```

2. 启动服务：
   ```bash
   sudo systemctl start deploy-bot
   ```

3. 开机自启（可选）：
   ```bash
   sudo systemctl enable deploy-bot
   ```

**管理命令：**
```bash
sudo systemctl start deploy-bot   # 启动
sudo systemctl stop deploy-bot    # 停止
sudo systemctl restart deploy-bot  # 重启
sudo systemctl status deploy-bot   # 状态
journalctl -u deploy-bot           # 查看日志
```

#### SysV init (Ubuntu 14.04- / Debian 7-)

1. 复制 init 脚本：
   ```bash
   sudo cp scripts/deploy-bot.init /etc/init.d/deploy-bot
   sudo chmod +x /etc/init.d/deploy-bot
   ```

2. 启动服务：
   ```bash
   sudo service deploy-bot start
   ```

3. 开机自启（可选）：
   ```bash
   sudo update-rc.d deploy-bot defaults
   ```

**管理命令：**
```bash
sudo service deploy-bot start   # 启动
sudo service deploy-bot stop    # 停止
sudo service deploy-bot restart  # 重启
sudo service deploy-bot status   # 状态
cat /opt/deploy-bot/logs/deploy-bot.log  # 查看日志
```

#### 二进制部署

将构建好的二进制文件复制到目标位置：
```bash
sudo cp target/release/deploy-bot /opt/deploy-bot/deploy-bot
sudo chmod +x /opt/deploy-bot/deploy-bot
```

### 4. 配置 Webhook

在 GitHub/GitLab/Codeup 仓库设置 Webhook：

- URL: `http://your-server:8080/webhook/your-project-name`
- Content-Type: `application/json`
- Secret/Token: 对应平台的 webhook secret 或 token

### 5. 触发部署

配置好 Webhook 后，当有代码推送到指定分支时，会自动触发部署。

## 项目配置说明 (.deploy.yaml)

| 字段 | 必填 | 说明 |
|------|------|------|
| repo_url | 是 | Git 仓库地址 |
| branch | 是 | 部署分支 |
| project_type | 是 | 项目类型：nodejs/rust/python/php/custom |
| docker_service | 否 | Docker 服务名称，配合 docker-compose 使用 |
| working_dir | 否 | 命令执行的工作目录 |
| run_user | 否 | 运行命令的用户（如 www-data、nginx） |
| install_command | 否 | 自定义安装命令 |
| build_command | 否 | 自定义构建命令 |
| extra_command | 否 | 部署完成后执行的额外命令 |
| env | 否 | 环境变量（键值对） |
| restart_service | 否 | 部署完成后需要重启的 Docker 服务 |
| docker_compose_path | 否 | Docker Compose 文件路径（会覆盖 config.yaml 的配置） |

### 字段详细说明

#### docker_service
指定 Docker Compose 服务名称。当设置此字段时，安装和构建命令会在对应的容器内执行。
```yaml
docker_service = "php"  # 使用 docker-compose.yml 中定义的 php 服务
```

#### working_dir
命令执行的工作目录。默认为仓库根目录。
```yaml
working_dir = "/app"  # 在仓库的 app 子目录中执行命令
```

#### run_user
指定运行命令的用户。未指定时使用当前进程用户。

- **非 Docker 环境**：使用 `sudo -u <user>` 切换用户执行命令
- **Docker 环境**：使用 `docker run --user <uid>:<gid>` 参数在容器内切换用户

```yaml
run_user = "www-data"  # 以 www-data 用户身份执行命令
```

使用此功能需要：
1. 部署服务器上存在指定的用户
2. 部署进程用户有 sudo 权限（无密码 sudo 更佳）

#### install_command / build_command
自定义安装和构建命令。如果指定这两个字段，会覆盖项目类型的默认行为。

```yaml
# 覆盖默认的 npm install
install_command = "npm install --legacy-peer-deps"

# 覆盖默认的 npm run build
build_command = "npm run build:prod"
```

**注意：** 设置 `install_command` 或 `build_command` 后，将完全使用自定义命令，不再执行默认命令。

#### extra_command
在构建完成后执行的额外命令。常用于部署后操作，如重启服务、清理缓存等。

```yaml
extra_command = "php artisan optimize:clear && systemctl restart nginx"
```

**执行时机：** 在 build_command 执行完成后运行（如果设置了 build_command），或在 install_command 后运行。

#### env
环境变量，在执行安装和构建命令时注入到当前环境。
```yaml
env = {
    NODE_ENV = "production",
    DATABASE_URL = "postgres://localhost/mydb",
    API_KEY = "your-api-key"
}
```

#### restart_service
部署完成后需要重启的 Docker 服务。配置后，deploy-bot 会自动执行 `docker compose restart <service>` 来重启服务。

**典型使用场景：** 在 Docker 环境中部署 Python 等语言项目时，依赖安装通常在临时容器中执行，需要重启实际运行的服务容器才能让新依赖生效。

```yaml
# 单服务
restart_service = "web"

# 多服务（按顺序串行重启）
restart_service = ["web", "worker"]
```

**执行时机：** 在所有部署步骤（git pull、安装依赖、构建、extra_command）完成后执行。

**注意事项：**
- 需要在 `config.yaml` 中配置 `docker_compose_path`
- 服务名称必须存在于 docker-compose.yml 中
- 重启失败会导致部署失败

#### docker_compose_path (.deploy.yaml)
在项目级别覆盖 `config.yaml` 中的 Docker Compose 配置。

```yaml
# 单个文件覆盖
docker_compose_path = "/path/to/docker-compose.yaml"

# 多个文件覆盖（按顺序）
docker_compose_path = ["/path/to/base.yaml", "/path/to/override.yaml"]
```

**优先级规则：**
1. `.deploy.yaml` 中的 `docker_compose_path` **优先**于 `config.yaml` 中的配置
2. 如果 `.deploy.yaml` 未设置此字段，则使用 `config.yaml` 的配置
3. 支持单个文件（字符串）或多个文件（数组）格式

**使用场景：**
- 不同环境使用不同的 Docker Compose 配置
- 基础配置 + 环境特定覆盖

#### project_type 默认行为
不同项目类型的默认安装和构建命令：

| 项目类型 | 默认安装命令 | 默认构建命令 | 说明 |
|----------|--------------|--------------|------|
| **nodejs** | 自动检测：pnpm > yarn > npm | `npm run build` | 根据 lock 文件自动选择包管理器 |
| **python** | poetry > venv + pip | 无（Python 无需构建） | poetry.lock 优先；否则使用虚拟环境 .venv 安装 |
| **php** | `composer install --no-dev` | 无（PHP 无需构建） | - |
| **rust** | 无（cargo build 自动处理依赖） | `cargo build --release` | 构建阶段自动处理依赖 |
| **custom** | 无 | 无 | 需要手动指定 install_command 和 build_command |

**自动检测优先级：**
- Node.js: `pnpm-lock.yaml` → `yarn.lock` → `npm`
- Python: `poetry.lock` → `requirements.txt`

**示例：**
```yaml
# Node.js 项目（自动检测）
project_type = "nodejs"
# 安装: pnpm install / yarn install / npm install
# 构建: npm run build

# Python 项目
project_type = "python"
# 安装: poetry install (如果有 poetry.lock)
#     或使用虚拟环境安装 requirements.txt (deploy-bot 自动处理)
#
# 虚拟环境说明：Python 项目默认使用 .venv 虚拟环境安装依赖，
# 这样不需要系统 site-packages 写权限，也避免了容器内 HOME 环境变量问题。

# PHP 项目
project_type = "php"
# 安装: composer install --no-dev

# Rust 项目
project_type = "rust"
# 构建: cargo build --release (无需单独安装)

# 自定义项目
project_type = "custom"
# 需要手动指定 install_command 和 build_command
```

## API 端点

### POST /webhook/:project_name

触发部署

Headers (至少配置一项):
- `X-Hub-Signature-256`: GitHub HMAC-SHA256 签名
- `X-Gitlab-Token`: GitLab Token
- `X-Codeup-Token`: 阿里云 Codeup Token

Response:
```json
{
  "message": "Deployment queued",
  "deployment_id": "uuid"
}
```

## Webhook 验证

Deploy Bot 强制要求 Webhook 验证，必须配置平台 Token 并在请求中带上对应的 Header。

### 各平台 Header 对照表

| 平台 | 请求 Header | 配置项 | 验证方式 |
|------|-------------|--------|----------|
| GitHub | `X-Hub-Signature-256` | `github_secret` | HMAC-SHA256 签名 |
| GitLab | `X-Gitlab-Token` | `gitlab_token` | Token 字符串匹配 |
| 阿里云 Codeup | `X-Codeup-Token` | `codeup_token` | Token 字符串匹配 |

### 配置示例

```yaml
[server]
# 选择至少一个平台配置
github_secret = "your-github-webhook-secret"
# gitlab_token = "your-gitlab-token"
# codeup_token = "your-codeup-token"
```

### 注意事项

- Header 名称大小写不敏感（`X-Codeup-Token` 或 `x-codeup-token` 均可）
- GitHub 使用签名验证（`X-Hub-Signature-256: sha256=...`）
- GitLab/Codeup 使用 Token 验证（Header 值与配置值完全匹配）

## 自更新 (Self-Update)

deploy-bot 支持从 GitHub Releases 自动下载并更新自身。

### 1. 配置更新脚本

在 `config.yaml` 中配置更新脚本路径：

```yaml
[server]
# ... 其他配置 ...

# 自更新配置
update_script = "/opt/deploy-bot/update.sh"
update_webhook_secret = "your-webhook-secret"
```

#### update_script
自更新脚本路径。该脚本负责停止旧进程、替换二进制、启动新进程。

示例脚本：
```bash
#!/bin/bash
# /opt/deploy-bot/update.sh
NEW_BINARY="$1"
BINARY_PATH="/usr/local/bin/deploy-bot"

# 停止服务
systemctl stop deploy-bot

# 替换二进制
cp "$NEW_BINARY" "$BINARY_PATH"
chmod +x "$BINARY_PATH"

# 启动服务
systemctl start deploy-bot
```

#### update_webhook_secret
自更新 Webhook 验证密钥。用于验证更新请求的来源合法性。

#### github_mirror
GitHub 镜像地址。当配置后，自更新下载 GitHub Releases 二进制文件时会添加镜像前缀，解决 GitHub 在中国大陆地区访问不稳定的问题。

```yaml
github_mirror = "https://ghproxy.com/"  # 镜像地址，以斜杠结尾
```

**转换规则：**
- 仅对包含 `github.com` 的 URL 应用镜像
- 原 URL: `https://github.com/owner/repo/releases/download/v1.0.0/deploy-bot`
- 转换后: `https://ghproxy.com/https://github.com/owner/repo/releases/download/v1.0.0/deploy-bot`

**常见镜像服务：**
- `https://ghproxy.com/`
- `https://mirror.ghproxy.com/`

### 2. GitHub Actions 配置

在 GitHub 仓库的 **Settings** → **Secrets and variables** → **Actions** 中添加 Secrets：

| Secret Name | 说明 | 示例 |
|-------------|------|------|
| `DEPLOY_BOT_WEBHOOK_URLS` | 逗号分隔的 webhook 地址 | `http://192.168.1.100:8088/webhook/update-self` |
| `DEPLOY_BOT_WEBHOOK_SECRET` | 验证密钥（可选） | `your-webhook-secret` |

### 3. 发布流程

```bash
# 打标签
git tag v0.2.0
git push --tags
```

GitHub Actions 会自动：
1. 构建 release 二进制
2. 上传到 GitHub Releases
3. 发送 webhook 通知 deploy-bot

### 4. 版本比对

deploy-bot 会自动比对版本号：
- 只有新版本号大于当前版本时才执行更新
- 版本号格式：`v{major}.{minor}.{patch}`
- 如果当前版本更新或相同，则跳过更新

### 5. API 端点

#### POST /webhook/update-self

自更新 webhook 端点。

Headers:
- `X-Update-Secret`: Webhook 验证密钥（如果配置了 `update_webhook_secret`）

Request Body:
```json
{
  "tag_name": "v0.2.0",
  "browser_download_url": "https://github.com/.../deploy-bot-v0.2.0-x86_64.tar.gz"
}
```

Response:
```json
{
  "message": "Update to v0.2.0 initiated",
  "updated": true,
  "version": "v0.2.0"
}
```

### 6. 本地重放更新

当收到自更新 webhook 时，deploy-bot 会自动保存 payload 到本地文件。可以通过 CLI 命令重放更新流程，用于测试自更新功能。

#### 保存位置

Payload 保存在 `{binary_dir}/.deploy-last-payload/deploy-bot-last-update.json`

例如：二进制文件在 `/opt/deploy-bot/deploy-bot`，则 payload 保存在 `/opt/deploy-bot/.deploy-last-payload/deploy-bot-last-update.json`

#### 重放命令

```bash
# 强制重放（跳过版本检查）
deploy-bot replay-update --force

# 非强制重放（会检查版本号）
deploy-bot replay-update
```

#### 使用场景

- 测试自更新流程时，无需发布假 release
- 验证 update_script 是否正常工作
- 下载链接是否有效

## 构建

```bash
cargo build --release
```

构建产物位于 `target/x86_64-unknow-linux-gnu/release/deploy-bot`

# Deploy Bot

基于 Rust 的自动部署服务，通过 Webhook 接口触发自动化部署流程。

## 功能特性

- **Webhook 接口**：支持 GitHub、GitLab、阿里云 Codeup Webhook 签名验证
- **自动部署**：代码拉取、依赖安装、项目构建自动化
- **任务队列**：部署任务排队机制，支持去重
- **多语言支持**：Node.js、Rust、Python、PHP
- **Docker 支持**：支持 Docker Compose 部署
- **自定义命令**：支持自定义安装和构建命令

## 快速开始

### 1. 配置服务

编辑 `config.yaml`：

```yaml
[server]
host = "0.0.0.0"
port = 8080
log_dir = "./logs"
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

### 字段详细说明

#### docker_compose_path
Docker Compose 文件路径。设置后，支持使用 Docker 容器执行安装和构建命令。

```yaml
docker_compose_path = "./docker-compose.yaml"  # 使用 Docker Compose
# docker_compose_path = null                  # 不使用 Docker，在宿主机执行
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
# run_user = "www-data"        # 可选：运行命令的用户
# install_command = "npm install"
# build_command = "npm run build"
# env = { NODE_ENV = "production" }
```

### 3. 启动服务

```bash
# 开发模式
cargo run

# 生产模式
cargo build --release
./target/release/deploy-bot
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

#### project_type 默认行为
不同项目类型的默认安装和构建命令：

| 项目类型 | 默认安装命令 | 默认构建命令 | 说明 |
|----------|--------------|--------------|------|
| **nodejs** | 自动检测：pnpm > yarn > npm | `npm run build` | 根据 lock 文件自动选择包管理器 |
| **python** | 自动检测：poetry > pip | 无（Python 无需构建） | poetry.lock 优先，否则安装 requirements.txt |
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
#     或 pip install -r requirements.txt (如果有 requirements.txt)

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

## 构建

```bash
cargo build --release
```

构建产物位于 `target/x86_64-unknow-linux-gnu/release/deploy-bot`

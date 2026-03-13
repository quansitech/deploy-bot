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
webhook_token = "your-secure-token"
log_dir = "./logs"
workspace_dir = "./workspace"
docker_compose_path = "./docker-compose.yaml"

# 可选：Webhook 平台验证
# github_secret = "your-github-secret"
# gitlab_token = "your-gitlab-token"
# codeup_token = "your-codeup-token"
```

### 2. 配置项目

在项目工作目录（如 `./workspace/my-project/`）下创建 `.deploy.yaml`：

```yaml
repo_url = "https://github.com/username/repo.git"
branch = "main"
project_type = "nodejs"
# docker_service = "php"        # 可选：使用 Docker 容器执行命令
# working_dir = "/app"           # 可选：命令执行目录
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

```bash
curl -X POST http://localhost:8080/webhook/my-project \
  -H "Authorization: Bearer your-secure-token"
```

## 项目配置说明 (.deploy.yaml)

| 字段 | 必填 | 说明 |
|------|------|------|
| repo_url | 是 | Git 仓库地址 |
| branch | 是 | 部署分支 |
| project_type | 是 | 项目类型：nodejs/rust/python/php/custom |
| docker_service | 否 | Docker 服务名称，配合 docker-compose 使用 |
| working_dir | 否 | 命令执行的工作目录 |
| install_command | 否 | 自定义安装命令 |
| build_command | 否 | 自定义构建命令 |
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

#### env
环境变量，在执行安装和构建命令时注入到当前环境。
```yaml
env = {
    NODE_ENV = "production",
    DATABASE_URL = "postgres://localhost/mydb",
    API_KEY = "your-api-key"
}
```

## API 端点

### POST /webhook/:project_name

触发部署

Headers:
- `Authorization: Bearer <token>`

Response:
```json
{
  "message": "Deployment queued",
  "deployment_id": "uuid"
}
```

## Webhook 验证

支持多种平台的 Webhook 签名验证：

- **GitHub**: 使用 `X-Hub-Signature-256` Header 和 HMAC-SHA256 签名
- **GitLab**: 使用 `X-Gitlab-Token` Header 和 Token 验证
- **阿里云 Codeup**: 使用 `X-Codeup-Token` Header 和 Token 验证

## 日志

日志保存在 `./logs` 目录下，按日期滚动。

## 构建

```bash
cargo build --release
```

构建产物位于 `target/release/deploy-bot`

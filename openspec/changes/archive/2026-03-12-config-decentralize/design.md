# 配置下沉设计

## 当前架构

```
deploy-bot/config.yaml  ← 包含 server + projects 所有配置
```

## 新架构

```
deploy-bot/config.yaml          ← 只有 server 配置
workspace/{project_name}/
├── .deploy.yaml                 ← 项目级配置
└── (项目代码文件)
```

## config.yaml 结构

```yaml
[server]
host = "0.0.0.0"
port = 8080
webhook_token = "your-token"
log_dir = "./logs"
workspace_dir = "./workspace"
```

## .deploy.yaml 结构

```yaml
repo_url = "https://github.com/xxx/yyy.git"
branch = "main"
project_type = "php"
install_command = "composer install"
build_command = "php artisan migrate --force"
env = { APP_ENV = "production" }
```

## 部署流程

```
1. 接收 Webhook 请求 /webhook/:project_name
2. 查找 workspace/{project_name} 目录
   - 不存在 → 返回错误 "Project not found"
3. 查找 workspace/{project_name}/.deploy.yaml
   - 不存在 → 返回错误 "Project not configured"
4. 读取 .deploy.yaml 配置
5. 执行 git clone/pull (使用 repo_url + branch)
6. 执行 install (使用 install_command)
7. 执行 build (使用 build_command)
```

## 需要修改的代码

| 文件 | 修改内容 |
|------|---------|
| config/mod.rs | 移除 ProjectConfig，简化 Config 结构 |
| config.yaml | 移除 projects 配置段 |
| webhook 处理逻辑 | 改为从项目目录读取配置 |
| deploy/manager.rs | 修改部署任务创建逻辑 |

## Context

当前部署系统在执行命令时使用部署进程的用户（通常是 root 或部署进程的用户）。项目需要以特定用户身份运行命令（如 www-data 运行 PHP 项目，nginx 运行前端项目），但无法在 `.deploy.yaml` 中配置。

## Goals / Non-Goals

**Goals:**
- 在 `.deploy.yaml` 中支持 `run_user` 配置
- Git 操作（clone/fetch/pull）使用 `run_user` 指定用户执行
- 非 Docker 环境使用 `sudo -u <user>` 执行命令
- Docker 环境使用 `docker compose run --user <uid>:<gid>` 参数
- 用户不存在或无权限时返回明确错误信息
- 默认使用当前进程用户（deploy-bot 运行用户）

**Non-Goals:**
- 不支持切换到 root 用户（安全考虑）
- 不修改现有 Docker 镜像内的用户配置

## Decisions

### 1. 用户名到 UID 的转换

在 Docker 环境中需要将用户名转换为 UID:GID 格式：

| 方案 | 实现 | 优点 | 缺点 |
|------|------|------|------|
| 方案 A | 在宿主机执行 `id -u` 和 `id -g` 获取 UID/GID | 提前验证用户存在，不依赖容器内工具 | 需要额外系统调用 |
| 方案 B | 在容器内执行 `id -u` 和 `id -g` | 简单直接 | 容器内可能没有该用户，无法提前验证 |

**决策**: 采用方案 A，在宿主机上解析用户名对应的 UID:GID。

### 2. 用户验证时机

| 方案 | 实现 | 优点 | 优点 |
|------|------|------|------|
| 方案 A | 在执行命令前验证用户存在 | 提前报错 | 需要额外系统调用 |
| 方案 B | 在命令执行失败后解析错误 | 无需额外调用 | 错误信息不够明确 |

**决策**: 采用方案 A，在命令执行前验证用户存在。

### 3. 命令包装方式

#### 3.1 Git 操作

Git 的 clone/fetch/checkout/pull 操作也需要使用 `run_user` 指定用户执行：

- **目的**：避免文件权限问题和 git dubious ownership 错误
- **实现**：使用 `sudo -u <user> git <command>` 格式

```bash
# clone
sudo -u www git clone --branch main --depth 1 repo_url /path/to/dir

# fetch
sudo -u www git fetch origin main

# checkout
sudo -u www git checkout -f origin/main

# pull
sudo -u www git pull origin
```

#### 3.2 安装和构建命令

非 Docker 环境使用 `sudo -u <user>` 包装命令：

```rust
// 原始命令: npm install
// 包装后: sudo -u www-data npm install
let cmd = format!("sudo -u {} {}", user, command);
```

Docker 环境使用 `--user` 参数：

```bash
docker compose run --rm -u 1000:1000 php sh -c "composer install"
```

## Risks / Trade-offs

- [风险] sudo 命令需要部署用户有 sudo 权限
  - [缓解] 在文档中说明需要配置 sudoers 或使用 NOPASSWD
- [风险] Docker 容器内不存在对应的 UID 用户
  - [缓解] Docker 的 `--user` 参数接受数字 UID，即使用户名不存在也会尝试以该 UID 运行（权限允许的情况下）
- [风险] 命令参数包含特殊字符
  - [缓解] 使用 shell 包装执行 `sh -c`

## Migration Plan

1. 部署新版本
2. 更新 `.deploy.yaml.example` 添加 `run_user` 字段说明
3. 项目配置中可选添加 `run_user` 字段

## Open Questions

（无）

## Additional Implementation Notes

### Web UI 显示运行用户

在部署日志和任务详情中显示当前执行命令的用户：

```
[部署日志]
2026-03-13 10:30:00 [www-data] Running: composer install
2026-03-13 10:30:05 [www-data] Running: php artisan migrate
```

在部署任务详情页面显示：
- 配置的 `run_user` 值
- 实际使用的用户（解析后的用户名或 UID:GID）

# Spec: Extra Command

## Overview

Extra Command 是部署流程中 Build 阶段之后的一个可选步骤，用于执行用户自定义的额外命令。

## Use Cases

1. **数据库迁移**: `php artisan migrate --force`
2. **缓存操作**: `php artisan cache:clear`
3. **静态资源**: `python manage.py collectstatic`
4. **部署脚本**: `npm run deploy`

## Design

### 配置字段

```rust
pub struct ProjectConfig {
    // ... 其他字段
    pub extra_command: Option<String>,
}
```

### 执行时机

```
Step 1: Clone
Step 2: Install
Step 3: Build
Step 4: Extra Command ← 在此处执行
```

### 行为

1. **可选**: `extra_command` 为 `None` 时跳过
2. **同步**: 等待命令执行完成后再继续
3. **失败处理**: 命令返回非零退出码 → deployment 失败

### 配置示例

```yaml
# .deploy.yaml
repo_url = "https://github.com/example/laravel-app.git"
branch = "main"
project_type = "php"
docker_service = "php"
extra_command = "php artisan migrate --force"
```

## Error Handling

| 退出码 | 行为 |
|--------|------|
| 0 | 成功，继续部署 |
| 非 0 | 失败，deployment 状态变为 Failed |

## 注意事项

- Extra Command 在 Build 之后执行，无论 Build 是否实际执行了命令
- Extra Command 使用与 Build 相同的 docker 配置（docker_service、working_dir）
- Extra Command 继承项目的环境变量配置

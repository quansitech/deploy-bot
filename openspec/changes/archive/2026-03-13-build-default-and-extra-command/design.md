# Design: Build 默认行为与 Extra Command

## Overview

本文档详细描述 Build 阶段默认行为和 Extra Command 的实现方案。

## Architecture

### 配置结构变更

```rust
// src/project_config/mod.rs
pub struct ProjectConfig {
    // ... 现有字段
    pub build_command: Option<String>,
    pub extra_command: Option<String>,  // 新增
}
```

### 数据库 Schema 变更

```sql
ALTER TABLE deployments ADD COLUMN extra_command TEXT;
```

### 部署流程

```
Step 1: Clone repository
Step 2: Install dependencies (install_command 或默认)
Step 3: Build
    ├── if build_command.is_some() → 执行自定义命令
    └── else → 执行默认 build (见下表)
Step 4: Extra command (if extra_command.is_some())
    └── 失败 → deployment 失败
```

## Default Build Behavior

| ProjectType | 行为 |
|-------------|------|
| Nodejs | 执行 `npm run build` |
| Rust | 执行 `cargo build --release` |
| Python | 检查 setup.py 存在则执行 `python setup.py bdist_wheel`<br>检查 pyproject.toml 存在则执行 `python -m build`<br>否则跳过 |
| Php | 跳过（不需要 build） |
| Custom | 跳过 |

### 理由

- **Nodejs**: 大多数 Node.js 项目需要 build 步骤（React、Vue 等前端项目）
- **Rust**: 编译型语言，需要 build 生成可执行文件
- **Python**: 纯 Python 项目（只需依赖安装）不需要 build，但需要打包的项目（发布到 PyPI）需要 build
- **Php**: 解释型语言，不需要编译步骤
- **Custom**: 无法推断默认行为，跳过

## Extra Command

### 设计决策

1. **执行时机**: Build 阶段完成后，无论 build 是否实际执行了命令
2. **失败处理**: extra_command 执行失败会导致整个 deployment 失败
3. **用途示例**:
   - `php artisan migrate --force` - Laravel 数据库迁移
   - `php artisan cache:clear` - 清除缓存
   - `npm run deploy` - 自定义部署脚本

## Implementation Details

### executor.rs 修改

```rust
// Step 3: Build (无条件执行)
match runner::task::run_build(
    &project_dir,
    &project.project_type,
    project.build_command.as_deref(),  // 传入自定义命令或 None
    &project.env,
    docker_compose_path,
    project.docker_service.as_deref(),
    project.working_dir.as_deref(),
).await { ... }

// Step 4: Extra command
if let Some(ref extra_cmd) = project.extra_command {
    match runner::task::run_command(...).await {
        Ok(_) => success,
        Err(e) => {
            // 失败导致 deployment 失败
            deployment_manager.update_status(&deployment_id, DeploymentStatus::Failed);
            return;
        }
    }
}
```

### task.rs 修改

```rust
// build_php - 跳过，不再检查 artisan
async fn build_php(...) -> Result<String, AppError> {
    Ok(String::new())  // 直接返回，不执行任何命令
}
```

## Error Handling

| 场景 | 处理 |
|------|------|
| build_command 执行失败 | deployment 失败，终止 |
| extra_command 执行失败 | deployment 失败，终止 |
| 默认 build 不适用（Php/Custom） | 跳过，继续下一步 |

## Backward Compatibility

不需要向后兼容。现有配置中已配置 `build_command` 的项目会继续使用自定义命令。

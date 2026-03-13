# Spec: 默认 Build 命令

## Overview

定义各项目类型的默认 build 命令。

## ProjectType -> Default Build Command

| ProjectType | 默认命令 | 条件 | 说明 |
|-------------|---------|------|------|
| Nodejs | `npm run build` | 始终 | 执行 package.json 中的 build 脚本 |
| Rust | `cargo build --release` | 始终 | 编译 Release 版本 |
| Python | `python setup.py bdist_wheel` | 存在 setup.py | 构建 wheel 包 |
| Python | `python -m build` | 存在 pyproject.toml | 使用 build 工具打包 |
| Python | (跳过) | 其他 | 纯依赖项目不需要 build |
| Php | (跳过) | 始终 | 解释型语言，不需要编译 |
| Custom | (跳过) | 始终 | 无法推断默认行为 |

## Implementation

默认命令在 `src/runner/task.rs` 中实现：

```rust
pub async fn run_build(
    project_dir: &Path,
    project_type: &ProjectType,
    custom_command: Option<&str>,  // 用户自定义命令
    ...
) -> Result<String, AppError> {
    // 如果提供了自定义命令，直接使用
    if let Some(cmd) = custom_command {
        return run_command(...).await;
    }

    // 否则使用项目类型的默认命令
    match project_type {
        ProjectType::Nodejs => build_nodejs(...).await,
        ProjectType::Rust => build_rust(...).await,
        ProjectType::Python => build_python(...).await,
        ProjectType::Php => build_php(...).await,
        ProjectType::Custom => Ok(String::new()),
    }
}
```

## Configuration Example

```yaml
# .deploy.yaml
repo_url = "https://github.com/example/myapp.git"
branch = "main"
project_type = "nodejs"
# build_command 可选，不配置则使用默认
# build_command = "npm run build"
extra_command = "npm run deploy:prod"
```

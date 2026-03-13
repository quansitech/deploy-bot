## Context

当前 Docker 执行命令使用 `docker compose run` 等待完成后返回输出。这种方式有以下问题：

1. **无实时输出** - 用户无法看到安装进度
2. **超时风险** - 大型依赖下载可能超时
3. **体验差** - 用户以为部署卡住

## Goals / Non-Goals

**Goals:**
- 实现 Docker 容器日志实时流式输出
- 通过 WebSocket 推送日志到前端
- 兼容现有的非 Docker 执行路径

**Non-Goals:**
- 不修改日志存储逻辑（仍保存到数据库）
- 不修改前端 UI（现有 WebSocket 接口已支持）

## Decisions

### 1. 使用 docker logs -f 实现实时流式

```rust
// 修改前：等待完成返回
let output = Command::new("docker")
    .args(["compose", "run", "--rm", "service", "sh", "-c", "command"])
    .output()?;

let stdout = String::from_utf8_lossy(&output.stdout);

// 修改后：实时流式
// 1. 后台启动容器
let mut child = Command::new("docker")
    .args(["compose", "run", "-d", "--rm", "service", "sh", "-c", "command"])
    .spawn()?;

// 2. 获取容器 ID
let container_id = ...;

// 3. 实时读取 logs
let mut logs = Command::new("docker")
    .args(["logs", "-f", &container_id])
    .spawn()?;

loop {
    // 读取日志行
    // 推送到 WebSocket
    // 推送到数据库
}
```

### 2. 传递日志回调函数

修改函数签名，添加日志回调参数：

```rust
pub async fn install_dependencies(
    // ...existing params
    log_callback: Option<Box<dyn Fn(&str) + Send>>,
) -> Result<String, AppError>
```

## Migration Plan

1. 修改 `run_docker_compose` 函数支持流式输出
2. 修改 `run_command` 函数传递回调
3. 在 `executor.rs` 中传入日志回调
4. 测试验证

## Open Questions

- 是否需要处理容器异常退出？
- 如何处理 docker logs 的 ANSI 颜色码？

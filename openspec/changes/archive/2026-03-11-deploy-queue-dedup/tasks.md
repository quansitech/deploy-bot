## 1. Modify DeploymentManager

- [x] 1.1 修改 `queue_deployment` 返回类型为 `Option<String>`
- [x] 1.2 添加去重逻辑：检查 Pending 或 Running 状态的任务
- [x] 1.3 返回 `None` 如果存在重复任务

## 2. Update Webhook Handler

- [x] 2.1 修改 handler 处理 `Option<String>` 返回值
- [x] 2.2 返回适当的提示信息当任务被跳过时

## 3. Add Unit Tests

- [x] 3.1 测试相同 project + branch 时返回 None
- [x] 3.2 测试不同 branch 时正常入队
- [x] 3.3 测试 Running 状态也算重复

## 4. Verify

- [x] 4.1 运行 `cargo clippy -- -D warnings`
- [x] 4.2 运行 `cargo test`

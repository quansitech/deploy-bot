## 1. 数据模型变更

- [x] 1.1 在 `src/config/mod.rs` 的 `ProjectType` 枚举中添加 `Git` 变体
- [x] 1.2 在 `src/config/mod.rs` 的 `ProjectType` 的 `Display` 实现中添加 `Git => "git"` 分支
- [x] 1.3 在 `src/config/mod.rs` 的 `ServerConfig` 中添加可选字段 `webhook_token: Option<String>`
- [x] 1.4 将 `src/project_config/mod.rs` 的 `ProjectConfig` 中 `repo_url` 和 `branch` 改为 `Option<String>`

## 2. 配置验证

- [x] 2.1 在 `src/project_config/mod.rs` 中为 `ProjectConfig` 添加 `validate()` 方法，对非 Custom 类型验证 `repo_url` 和 `branch` 不为空
- [x] 2.2 在 `src/webhook/handler.rs` 的 `handle_webhook` 中，加载 `.deploy.yaml` 后调用 `validate()` 并返回错误响应

## 3. 部署流程修改

- [x] 3.1 在 `src/deploy/executor.rs` 的 `execute_deployment` 中，将 git pull 步骤包裹在 `project_type != Custom` 的条件判断中
- [x] 3.2 在 `src/deploy/executor.rs` 中，Custom 类型跳过 git pull 时记录日志
- [x] 3.3 修复 `execute_deployment` 中所有使用 `project.repo_url` 和 `project.branch` 的地方，处理 `Option<String>`

## 4. Install/Build 步骤

- [x] 4.1 在 `src/installer/tasks.rs` 的 `install_dependencies` match 中添加 `ProjectType::Git => Ok(String::new())`
- [x] 4.2 在 `src/runner/task.rs` 的 `run_build` match 中添加 `ProjectType::Git => Ok(String::new())`

## 5. Webhook 通用 Token 验证

- [x] 5.1 在 `src/webhook/handler.rs` 的 `validate_webhook_request` 中添加 `X-Webhook-Token` header 验证逻辑
- [x] 5.2 验证顺序：GitHub → GitLab → Codeup → 通用 Token，任意一个通过即返回 Ok

## 6. 测试

- [x] 6.1 为 `ProjectConfig::validate()` 添加单元测试（各类型的合法/非法配置）
- [x] 6.2 为 `validate_webhook_request` 添加通用 token 验证的单元测试
- [x] 6.3 在 `src/project_config/mod.rs` 的测试中添加 `project_type = "git"` 的解析测试
- [x] 6.4 运行 `cargo clippy -- -D warnings` 确保无警告
- [x] 6.5 运行 `cargo test` 确保所有测试通过

## 7. 文档更新

- [x] 7.1 更新 `README.md`，说明 `git` 和 `custom` 类型的用途及区别
- [x] 7.2 在 `README.md` 中添加 `webhook_token` 配置项说明
- [x] 7.3 在 `README.md` 中添加从旧版 `custom` 迁移到 `git` 类型的说明

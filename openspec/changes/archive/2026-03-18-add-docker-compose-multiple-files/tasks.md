## 1. 数据结构变更

- [x] 1.1 在 src/config/mod.rs 中添加 DockerComposePaths 枚举类型（使用 serde untagged 兼容字符串和数组）
- [x] 1.2 修改 ServerConfig.docker_compose_path 类型为 Option<DockerComposePaths>
- [x] 1.3 更新 DockerComposeCommand::detect 函数，使用 is_empty() 替代 is_none()
- [x] 1.4 在 src/project_config/mod.rs 中添加 DockerComposePaths 字段到 ProjectConfig

## 2. 配置合并逻辑

- [x] 2.1 在部署执行流程中添加配置合并逻辑（.deploy.yaml 覆盖 config.yaml）

## 3. run_docker_compose 函数修改

- [x] 3.1 修改 src/installer/tasks.rs 中 run_docker_compose 函数签名，接受 Vec<String> 而非单个路径
- [x] 3.2 修改 -f 参数生成逻辑，支持循环生成多个 -f 参数

## 4. restart_docker_services 函数修改

- [x] 4.1 修改 src/deploy/executor.rs 中 restart_docker_services 函数签名
- [x] 4.2 修改 -f 参数生成逻辑，支持多个配置文件

## 5. 函数签名调整

- [x] 5.1 更新 src/installer/tasks.rs 中 install_dependencies 和 run_command 函数的 docker_compose_path 参数类型
- [x] 5.2 更新 src/runner/task.rs 中相关函数的参数类型

## 6. 测试

- [x] 6.1 为 DockerComposePaths 添加单元测试
- [x] 6.2 运行 cargo clippy 检查
- [x] 6.3 运行 cargo test 确保所有测试通过

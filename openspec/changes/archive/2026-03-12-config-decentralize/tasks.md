# Tasks

- [x] 1. 修改 config/mod.rs - 移除 ProjectConfig，简化 Config 结构
- [x] 2. 修改 config.yaml - 移除 projects 配置段
- [x] 3. 创建 project_config/mod.rs - 新增项目级配置解析模块
- [x] 4. 创建 project_config/ProjectConfig - 定义 .deploy.yaml 结构
- [x] 5. 修改 webhook 处理逻辑 - 从项目目录读取 .deploy.yaml
- [x] 6. 修改 deploy/manager.rs - 修改部署任务创建逻辑
- [x] 7. 运行 Clippy 检查代码
- [x] 8. 测试部署流程

## 1. 修改 run_docker_compose 函数

- [x] 1.1 修改 run_docker_compose 使用 -d 后台启动容器
- [x] 1.2 添加 docker logs -f 实时读取日志
- [x] 1.3 添加日志回调参数

## 2. 修改日志调用链

- [x] 2.1 修改 run_command 函数支持日志回调
- [x] 2.2 修改 install_dependencies 函数支持日志回调
- [x] 2.3 在 executor 中传入日志回调

## 3. 测试验证

- [x] 3.1 运行 clippy 检查代码
- [x] 3.2 运行单元测试
- [x] 3.3 手动测试 Docker 日志实时输出

# 项目开发规范

## 代码质量检查

**每次实现任务完成后，必须运行 Clippy 检查代码：**

```bash
cargo clippy -- -D warnings
```

在以下情况下必须执行：
1. 完成任何功能实现后
2. 完成任何 bug 修复后
3. 完成代码重构后
4. 提交代码前

如果 Clippy 报告错误或警告，必须修复后再继续。

## 单元测试

**每次实现新功能或修改现有功能后，必须为相关的 public 函数编写单元测试：**

- 测试文件放在各模块的 `#[cfg(test)]` 块中
- 每个 public 函数至少有一个对应的测试用例
- 测试必须通过才能继续

```bash
# 运行所有测试
cargo test

# 运行特定模块测试
cargo test <module>::tests
```

## 开发

```bash
# 代码检查
cargo clippy -- -D warnings

# 运行测试
cargo test

# 构建
cargo build --release
```

## 发布流程

**每次发布新版本前，必须更新 Cargo.toml 中的版本号：**

```toml
[package]
version = "0.4.0"  # 更新为新版本号
```

**发布步骤：**

1. 更新 Cargo.toml 版本号
2. 提交代码并推送（**必须包含 openspec 目录的变更**）
3. 创建 tag 并推送：

```bash
git tag v0.4.0
git push origin v0.4.0
```

GitHub Actions 会自动构建并发布。

**注意：** 提交代码时必须同时提交 `openspec/` 目录下的所有变更，包括：
- `openspec/changes/` - 变更提案和设计文档
- `openspec/specs/` - 功能规格文档

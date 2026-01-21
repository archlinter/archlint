---
title: CLI 参考
description: "archlint CLI 命令的完整参考，包括 scan、diff、snapshot 和 watch。"
---

# CLI 参考

archlint CLI 是与该工具交互的主要方式。

## 通用用法

```bash
archlint [command] [options]
```

## 命令

| 命令                           | 描述                           |
| ------------------------------ | ------------------------------ |
| [`init`](/zh/cli/init)         | 初始化新的配置文件             |
| [`scan`](/zh/cli/scan)         | 运行一次性架构分析             |
| [`diff`](/zh/cli/diff)         | 将当前状态与基线进行比较       |
| [`snapshot`](/zh/cli/snapshot) | 将当前状态保存到 JSON 文件     |
| [`watch`](/zh/cli/watch)       | 在监听模式下运行以获得实时反馈 |

## 全局选项

| 选项                  | 描述                        |
| --------------------- | --------------------------- |
| `-c, --config <path>` | 配置文件路径                |
| `-v, --verbose`       | 启用详细日志                |
| `-q, --quiet`         | CI 友好模式（不显示进度条） |
| `-V, --version`       | 显示版本信息                |
| `-h, --help`          | 显示命令的帮助信息          |

---
title: oclif 支持
description: "内置支持 oclif CLI 框架，将命令文件识别为入口点并提供架构预设。"
---

# oclif 支持

archlint 为 [oclif](https://oclif.io/) (Open CLI Framework) 提供内置支持。

## 特性

- **CLI 入口点**：自动识别命令文件为入口点。
- **钩子检测**：识别 oclif 钩子以防止死代码分析中的误报。
- **架构规则**：提供遵循 oclif 推荐目录结构的预设。

## 配置

要启用 oclif 支持，请将其添加到 `extends` 列表：

```yaml
extends:
  - oclif
```

## 检测逻辑

在以下情况下自动检测 oclif 预设：

1. `package.json` 的依赖项中包含 `@oclif/core`。
2. 项目中存在 `oclif.manifest.json` 文件。

---
title: Next.js 支持
description: "理解 Next.js 基于文件的路由，将 pages/app 目录识别为入口点，并为常见模式放宽 barrel 文件规则。"
---

# Next.js 支持

Next.js 项目具有独特的文件路由和打包模式，archlint 能够理解这些模式。

## 主要特性

- **路由感知**：自动将 `pages/` 和 `app/` 目录中的文件识别为入口点。
- **桶文件（Barrel Files）**：为常见的 Next.js 模式放宽桶文件规则。
- **客户端/服务端组件**：(即将推出) 针对仅限服务端 vs 仅限客户端代码泄漏的专项分析。

## 推荐配置

```yaml
extends:
  - nextjs

entry_points:
  - 'src/pages/**/*.tsx'
  - 'src/app/**/*.tsx'
```

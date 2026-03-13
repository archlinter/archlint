---
layout: home
title: Archlint — TypeScript & JavaScript 架构检查器
description: '快速、基于 AST 的 TypeScript/JavaScript 项目架构问题检测器。通过 28+ 个检测器和极速分析阻止架构恶化。'

hero:
  name: 'archlint'
  text: '我们不修复您的架构。我们阻止它变得更糟。'
  tagline: 快速、基于 AST 的 TypeScript/JavaScript 项目架构问题检测器。
  image:
    src: /logo.svg
    alt: archlint logo
  actions:
    - theme: brand
      text: 开始使用
      link: /zh/getting-started/
    - theme: alt
      text: 在 GitHub 上查看
      link: https://github.com/archlinter/archlint

features:
  - title: 28+ 个检测器
    details: 从循环依赖到上帝模块和图层违规。使用 Rust 和 oxc 构建，实现极致性能。
  - title: 差异模式
    details: “棘轮原则（持续改进）”的理念。锁定当前状态，仅在出现新的架构退化时发出警告。
  - title: 框架感知
    details: 内置 NestJS、Next.js、React 和 oclif 预设。了解您框架的架构模式。
  - title: 极速分析
    details: 在 5 秒内分析 200+ 个文件。并行处理和智能的基于内容的缓存。
  - title: 详尽见解
    details: 每份报告都包含严重程度评分、清晰的解释和重构建议。
  - title: 集成就绪
    details: ESLint 插件、GitHub Actions、GitLab CI，甚至还有为您 AI 编程助手准备的 MCP 服务器。
---

## 为什么选择 archlint？

现代代码库的复杂度增长很快。archlint 帮助您在架构问题演变成技术债之前尽早发现它们。

```bash
# 在您的 PR 中捕获退化
npx -y @archlinter/cli diff HEAD~1 --explain
```

```
🔴 REGRESSION: New cycle detected

  src/orders/service.ts → src/payments/processor.ts → src/orders/service.ts

  Why this is bad:
    Circular dependencies create tight coupling between modules.
    Changes in one module can cause unexpected failures in the other.

  How to fix:
    Extract shared logic into a separate module, or use dependency injection.
```

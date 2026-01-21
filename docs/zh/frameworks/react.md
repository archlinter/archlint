---
title: React 支持
description: "针对 React 组件的专业分析，识别命名模式，禁用组件的 LCOM，并将自定义 hooks 理解为入口点。"
---

# React 支持

React 组件与传统的类或模块具有不同的架构特征。

## 主要特性

- **组件识别**：通过命名模式和 JSX 使用情况识别 React 组件。
- **禁用 LCOM**：自动禁用组件的低内聚（LCOM）探测器，因为它们本质上专注于 UI 状态和渲染。
- **Hook 分析**：理解自定义 hook 是共享 UI 逻辑的入口点。

## 推荐配置

```yaml
extends:
  - react
```

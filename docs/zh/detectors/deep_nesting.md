---
title: 过深嵌套
description: "识别嵌套过深的代码块，使代码的阅读难度呈指数级增长，并表明函数承担了过多职责。"
---

# 过深嵌套

**ID:** `deep_nesting` | **严重程度:** 低 (默认)

识别嵌套过深的代码块（if、for、while 等）。

## 为什么这是一种坏味道

过深嵌套的代码阅读和理解难度会呈指数级增加。这通常意味着一个函数承担了过多的职责，或者逻辑可以被简化。

## 如何修复

- **Guard Clauses（卫语句）**：尽早返回以避免 `else` 块并减少嵌套。
- **Extract Function（提取函数）**：将内部嵌套块移动到新函数中。
- **Flatten Logic（打平逻辑）**：重新评估逻辑，看是否可以更简单地表达。

## 配置

```yaml
rules:
  deep_nesting:
    severity: low
    max_depth: 4
```

## ESLint 规则

此检测器可作为 ESLint 规则使用，以便在编辑器中获得实时反馈。

```javascript
// eslint.config.js
export default [
  {
    rules: {
      '@archlinter/no-deep-nesting': 'warn',
    },
  },
];
```

详见 [ESLint 集成](/zh/integrations/eslint) 了解设置说明。

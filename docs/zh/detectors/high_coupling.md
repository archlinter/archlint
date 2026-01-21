---
title: 高耦合
description: "识别依赖过多其他模块的模块，这会在代码库中造成刚性和脆弱性。"
---

# 高耦合

**ID:** `high_coupling` | **严重程度:** 中 (默认)

高耦合（High coupling）发生在一个模块依赖于过多的其他模块时（高 Fan-out）。

## 为什么这是一种坏味道

- **僵化性**：任何依赖项的更改都可能需要更改此模块。
- **脆弱性**：当其任何依赖项发生更改时，该模块更容易损坏。
- **难以测试**：在单元测试中需要大量的 Mock（模拟对象）来进行隔离。

## 如何修复

1. **提取职责**：如果一个模块有太多的依赖项，它可能承担了过多的职责。
2. **使用抽象**：依赖于接口或外观，而不是许多具体的实现。

## 配置

```yaml
rules:
  high_coupling:
    severity: medium
    max_cbo: 20
```

## ESLint 规则

此检测器可作为 ESLint 规则使用，以便在编辑器中获得实时反馈。

```javascript
// eslint.config.js
export default [
  {
    rules: {
      '@archlinter/no-high-coupling': 'warn',
    },
  },
];
```

详见 [ESLint 集成](/zh/integrations/eslint) 了解设置说明。

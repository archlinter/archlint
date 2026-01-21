---
title: 认知复杂度
description: "检测由于深度嵌套和复杂逻辑而难以理解的函数，有助于减少心理负担和维护风险。"
---

# 认知复杂度

**ID:** `cognitive_complexity` | **严重程度:** 中 (默认)

该检测器识别具有高认知复杂度 (Cognitive Complexity) 的函数。认知复杂度衡量的是理解代码的难度，而不仅仅是代码有多少条路径。

## 为什么这是一个代码异味

- **高心理负荷**: 深层嵌套的逻辑和复杂的布尔表达式使开发人员难以在脑中保持状态。
- **维护风险**: 难以理解的代码在修改时容易产生错误。
- **隐藏的 Bug**: 逻辑错误经常隐藏在深层嵌套的结构中。

## 如何计算

认知复杂度根据以下各项计算：

1.  **结构性增量**: `if`、`else`、`switch`、`for`、`while`、`do-while`、`catch`、三元运算符和逻辑序列。
2.  **嵌套惩罚**: 控制结构的增量根据其嵌套级别而增加。
3.  **特殊情况**: `switch` 无论有多少个 case，整个块只计算一次。

## 如何修复

1.  **扁平化逻辑**: 使用卫语句 (提前返回) 来减少嵌套。
2.  **提取方法**: 将嵌套块或复杂条件移动到小而集中的函数中。
3.  **简化表达式**: 将复杂的布尔条件分解为中间变量或函数。
4.  **替换嵌套 If**: 考虑使用查找表或策略模式。

## 配置

```yaml
rules:
  cognitive_complexity:
    severity: medium
    max_complexity: 15
```

## ESLint 规则

此检测器可作为 ESLint 规则使用，以便在编辑器中获得实时反馈。

```javascript
// eslint.config.js
export default [
  {
    rules: {
      '@archlinter/no-high-cognitive-complexity': 'warn',
    },
  },
];
```

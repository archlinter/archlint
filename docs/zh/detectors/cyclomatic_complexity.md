# 圈复杂度 (Cyclomatic Complexity)

**ID:** `cyclomatic_complexity` | **严重程度:** 中 (默认)

圈复杂度衡量的是代码执行路径的多样性。你可以把它看作是代码中 `if-else` 和 `switch` 所产生的“面条代码”系数。

## 为什么这是一种坏味道

- **思维迷宫**：每一个 `if`、`else` 或 `case` 都会为这段代码增加一个新的转弯点。如果一个函数有 20 条路径，你可以打赌，开发人员迟早会迷失在其中。
- **测试难题**：要真正测试一个复杂的函数，你需要为每条可能的路径都准备一个测试用例。但在现实世界中，这通常意味着某些分支永远不会被测试到。
- **“蝴蝶效应”**：在极度复杂的函数中，修改一行代码可能会在五个分支之外产生奇怪且不可预测的后果。

## 如何修复

1. **提取方法 (Extract Method)**: 将复杂的逻辑分解为更小的命名函数。
2. **卫语句 (Guard Clauses)**: 使用早期返回来减少嵌套层级。
3. **用多态替换条件语句 (Replace Conditional with Polymorphism)**: 使用对象或策略模式代替大型 `switch` 或 `if/else` 块。

## 配置

```yaml
rules:
  cyclomatic_complexity:
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
      '@archlinter/no-high-cyclomatic-complexity': 'warn',
    },
  },
];
```

详见 [ESLint 集成](/zh/integrations/eslint) 了解设置说明。

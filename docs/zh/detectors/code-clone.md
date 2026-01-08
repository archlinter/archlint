# 代码克隆

**ID:** `code_clone` | **严重程度:** 中（默认）

此检测器识别项目中的重复代码块。它支持 **Type-1**（完全一致）和 **Type-2**（重命名变量/标识符）克隆。

## 为什么这是一个代码异味

- **维护开销**: 在一个地方修复错误或进行更改需要更新所有重复项。
- **违反 DRY 原则**: 重复显然表明缺少抽象或重用。
- **演进不一致**: 随着时间的推移，重复项可能会发生偏差，从而导致微妙的错误并使重构变得更加困难。

## 如何修复

1. **提取方法 (Extract Method)**: 将共享逻辑移动到单个函数中，并从多个地方调用它。
2. **泛型组件**: 对于 UI 代码，创建一个带有 props 的可重用组件。
3. **工具模块**: 将通用的辅助逻辑移动到共享的工具文件中。

## 配置

```yaml
rules:
  code_clone:
    enabled: true
    severity: warn
    min_tokens: 50
    min_lines: 6
```

### 选项

- `min_tokens`: 触发克隆检测所需的最小归一化标记数（默认值：50）。
- `min_lines`: 克隆必须跨越的最小行数（默认值：6）。

## ESLint 规则

此检测器可作为 ESLint 规则使用，以便在编辑器中获得实时反馈。

```javascript
// eslint.config.js
export default [
  {
    rules: {
      '@archlinter/no-code-clone': 'warn',
    },
  },
];
```

有关设置说明，请参阅 [ESLint 集成](/zh/integrations/eslint)。

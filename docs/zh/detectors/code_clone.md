# 代码克隆

**ID:** `code_clone` | **严重程度:** 中（默认）

此检测器找出那些有人走"复制粘贴"捷径的地方。它会寻找在你项目中被复制的相同逻辑。

## 为什么这是一个代码异味

- **Bug 成倍增加**: 如果你在一个副本中发现一个 bug，你必须记得在其他四个副本中也修复它。剧透：你通常会忘记一个。
- **维护开销**: 每次你想改变某个特定逻辑的工作方式时，你都在一遍又一遍地做同样的工作。
- **演进不一致**: 最终，一个副本更新了，而另一个没有，你那"相同"的逻辑突然在应用的不同部分表现得不同了。

## 如何修复

1. **提取方法 (Extract Method)**: 将共享逻辑移动到单个函数中，并从多个地方调用它。
2. **泛型组件**: 对于 UI 代码，创建一个带有 props 的可重用组件。
3. **工具模块**: 将通用的辅助逻辑移动到共享的工具文件中。

## 配置

```yaml
rules:
  code_clone:
    enabled: true
    severity: medium
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

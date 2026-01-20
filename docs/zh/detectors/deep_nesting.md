# 过深嵌套

**ID:** `deep_nesting` | **严重程度:** 低 (默认)

识别嵌套过深的代码块（if、for、while 等），这些代码块往往看起来像是一个倾斜的“金字塔”。

## 为什么这是一种坏味道

阅读嵌套过深的代码就像在读一段带有太多（括号（里面（还有括号）））的句子。这会让人的大脑感到非常疲倦，通常也意味着你的函数试图同时处理太多的边缘情况。更好的做法是“尽早失败”或者将逻辑提取出来。

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

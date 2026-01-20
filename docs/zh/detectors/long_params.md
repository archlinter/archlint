# 过长参数列表

**ID:** `long_params` | **严重程度:** 低 (默认)

识别一次性要求过多信息的函数。

## 为什么这是一种坏味道

有10个参数的函数，调用时令人困惑，阅读时更是混乱。第三个参数是 `userId` 还是 `orderId`？当你有一长串参数时，这说明函数要么做得太多，要么这些参数本应该一起放在一个对象里。

## 如何修复

- **Introduce Parameter Object (引入参数对象)**：将相关的参数组合成一个对象或接口。
- **Decompose Function (分解函数)**：将函数分解为需要更少参数的小函数。

## 配置

```yaml
rules:
  long_params:
    severity: low
    max_params: 5
    ignore_constructors: true
```

## ESLint 规则

此检测器可作为 ESLint 规则使用，以便在编辑器中获得实时反馈。

```javascript
// eslint.config.js
export default [
  {
    rules: {
      '@archlinter/no-long-params': 'warn',
    },
  },
];
```

详见 [ESLint 集成](/zh/integrations/eslint) 了解设置说明。

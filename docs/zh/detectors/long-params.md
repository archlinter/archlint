# 过长参数列表

**ID:** `long_params` | **严重程度:** 低 (默认)

识别具有过多参数的函数或方法。

## 为什么这是一种坏味道

参数过多的函数难以使用且难以阅读。它们通常表明函数承担了过多的职责，或者某些参数应该被组合成一个对象。

## 如何修复

- **Introduce Parameter Object (引入参数对象)**：将相关的参数组合成一个对象或接口。
- **Decompose Function (分解函数)**：将函数分解为需要更少参数的小函数。

## 配置

```yaml
rules:
  long_params:
    severity: info
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

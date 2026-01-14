# 死符号 (Dead Symbols)

**ID:** `dead_symbols` | **严重程度:** 低 (默认)

识别在文件内定义但从未被使用（甚至在本地也未被使用）的函数、变量或类。

## 为什么这是一种坏味道

它只是杂乱无章的堆积。它在不增加任何价值的情况下增加了文件的阅读和维护难度。

## 如何修复

删除未使用的符号。

## 配置

```yaml
rules:
  dead_symbols:
    severity: low
    # 要忽略的方法名称列表（例如框架生命周期方法）
    ignore_methods:
      - 'constructor'
    # 实现时要忽略的接口/类方法映射
    contract_methods:
      MyInterface: ['method1', 'method2']
      ValidatorConstraintInterface: ['validate', 'defaultMessage']
```

## ESLint 规则

此检测器可作为 ESLint 规则使用，以便在编辑器中获得实时反馈。

```javascript
// eslint.config.js
export default [
  {
    rules: {
      '@archlinter/no-dead-symbols': 'warn',
    },
  },
];
```

详见 [ESLint 集成](/zh/integrations/eslint) 了解设置说明。

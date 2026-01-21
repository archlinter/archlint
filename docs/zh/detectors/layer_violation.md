---
title: 分层违规
description: "检测一个架构层中的代码错误地导入另一层代码的情况，这会破坏抽象和单一职责原则。"
---

# 分层违规

**ID:** `layer_violation` | **严重程度:** 高 (默认)

分层违规（Layer violation）发生在当一个架构层中的代码导入了它不应该知道的层中的代码时（例如，Domain 层导入了 Infrastructure 层）。

## 为什么这是一种坏味道

- **打破了抽象**：内部实现细节泄露到了高层业务逻辑中。
- **测试困难**：如果没有对基础设施（数据库、API 等）的模拟，业务逻辑将难以测试。
- **僵化性**：更改数据库或外部库需要更改核心业务逻辑。

## 配置

你必须在 `.archlint.yaml` 中定义你的层：

```yaml
rules:
  layer_violation:
    layers:
      - name: domain
        path: ['**/domain/**']
        allowed_imports: [] # Domain 层不导入任何内容

      - name: application
        path: ['**/application/**']
        allowed_imports: ['domain']

      - name: infrastructure
        path: ['**/infrastructure/**']
        allowed_imports: ['domain', 'application']
```

## ESLint 规则

此检测器可作为 ESLint 规则使用，以便在编辑器中获得实时反馈。

```javascript
// eslint.config.js
export default [
  {
    rules: {
      '@archlinter/no-layer-violations': 'error',
    },
  },
];
```

详见 [ESLint 集成](/zh/integrations/eslint) 了解设置说明。

## 如何修复

1. **依赖倒置（Dependency Inversion）**：在较高级别层（Domain）定义接口，并在较低级别层（Infrastructure）实现它。
2. **重构**：将放错地方的代码移动到适当的层中。

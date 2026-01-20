# 分层违规

**ID:** `layer_violation` | **严重程度:** 高 (默认)

分层违规发生在你的“整洁架构”开始出现泄露时。也就是说，你的高层业务逻辑（Domain）开始询问关于数据库表或 API 端点（Infrastructure）的细节。

## 为什么这是一种坏味道

- **抽象泄露**：你的业务逻辑不应该关心你使用的是 Postgres 还是一个 JSON 文件。当分层发生泄露时，你就失去了这种架构上的自由。
- **脆弱的测试**：你不应该为了测试一个简单的业务规则，而去启动一个复杂的数据库模拟。
- **修改困难**：想要更换日志库？太糟糕了，因为你已经把它直接导入到了领域层的核心中，现在你不得不重构整个项目。

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

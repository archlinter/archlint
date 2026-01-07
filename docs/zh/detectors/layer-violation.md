# 分层违规

**ID:** `layer_violation` | **Severity:** High (default)

分层违规（Layer violation）发生在当一个架构层中的代码导入了它不应该知道的层中的代码时（例如，Domain 层导入了 Infrastructure 层）。

## 为什么这是一种坏味道

- **打破了抽象**：内部实现细节泄露到了高层业务逻辑中。
- **测试困难**：如果没有对基础设施（数据库、API 等）的模拟，业务逻辑将难以测试。
- **僵化性**：更改数据库或外部库需要更改核心业务逻辑。

## 配置

你必须在 `.archlint.yaml` 中定义你的层：

```yaml
layers:
  - name: domain
    paths: ['**/domain/**']
    can_import: [] # Domain 层不导入任何内容

  - name: application
    paths: ['**/application/**']
    can_import: ['domain']

  - name: infrastructure
    paths: ['**/infrastructure/**']
    can_import: ['domain', 'application']
```

## 如何修复

1. **依赖倒置（Dependency Inversion）**：在较高级别层（Domain）定义接口，并在较低级别层（Infrastructure）实现它。
2. **重构**：将放错地方的代码移动到适当的层中。

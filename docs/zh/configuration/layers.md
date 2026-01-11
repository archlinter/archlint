# 分层

分层配置允许您定义项目的架构级别，并强制执行它们之间的依赖规则。

## 定义分层

分层在 `layer_violation` 规则内进行配置。每个分层定义包含：

- `name`: 分层的唯一名称。
- `path`（或 `paths`）: 识别该分层中文件的 glob 模式。
- `allowed_imports`（或 `can_import`）: 允许该分层导入的分层名称列表。

## 示例：整洁架构 (Clean Architecture)

```yaml
rules:
  layer_violation:
    severity: high
    layers:
      - name: domain
        path: '**/domain/**'
        allowed_imports: [] # Domain 层不应依赖于任何东西

      - name: application
        path: '**/application/**'
        allowed_imports:
          - domain

      - name: infrastructure
        path: '**/infrastructure/**'
        allowed_imports:
          - domain
          - application

      - name: presentation
        path: '**/presentation/**'
        allowed_imports:
          - domain
          - application
```

## 工作原理

当启用 `layer_violation` 检测器时：

1. 它根据 `path` 模式将项目中的每个文件映射到特定的分层。
2. 如果一个文件匹配多个模式，将选择最具体的模式（最长模式）。
3. 具检查每个导入。如果分层 `A` 中的文件导入了分层 `B` 中的文件，但 `B` 不在分层 `A` 的 `allowed_imports` 列表中，则会报告违规。

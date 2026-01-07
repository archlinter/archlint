# 架构层

`layers` 配置允许您定义项目的架构层，并强制执行它们之间的依赖规则。

## 定义架构层

每个层定义包含：

- `name`：层的唯一标识符。
- `paths`：标识该层中文件的 Glob 模式数组。
- `can_import`：该层允许依赖的层名称数组。

## 示例：整洁架构 (Clean Architecture)

```yaml
layers:
  - name: domain
    paths: ['**/domain/**']
    can_import: [] # 领域层必须是独立的

  - name: application
    paths: ['**/application/**', '**/use-cases/**']
    can_import:
      - domain

  - name: infrastructure
    paths: ['**/infrastructure/**', '**/adapters/**']
    can_import:
      - domain
      - application

  - name: presentation
    paths: ['**/controllers/**', '**/api/**', '**/ui/**']
    can_import:
      - domain
      - application
```

## 工作原理

当 `layer_violation` 探测器启用时：

1. 它根据 `paths` 模式将项目中的每个文件分配到一个层。
2. 它检查这些文件中的每一个导入（import）。
3. 如果层 `A` 中的文件导入了层 `B` 中的文件，但 `B` 不在 `A` 的 `can_import` 列表中，则会报告违规。

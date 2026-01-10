# 框架预设

archlint 使用基于 YAML 的预设来理解特定于框架的模式，并减少误报。

## 工作原理

archlint 通过分析 `package.json` 中的依赖项和配置文件自动检测框架。您还可以在 `.archlint.yaml` 中显式扩展预设：

```yaml
extends:
  - nestjs
  - ./my-company-preset.yaml
```

## 内置预设

- **nestjs**：用于 NestJS 应用程序。
- **nextjs**：用于 Next.js 项目。
- **react**：用于 React 库和应用程序。
- **oclif**：用于使用 oclif 构建的 CLI 工具。

## 自定义预设

预设文件是标准的 archlint 配置文件，带有额外的 `detect` 部分用于自动发现。

### 结构

```yaml
name: my-framework
version: 1

# 自动检测规则
detect:
  packages:
    any_of: ['my-core-pkg']
  files:
    any_of: ['my-framework.config.js']

# 全局规则
rules:
  layer_violation: error
  dead_symbols:
    ignore_methods: ['onInit', 'onDestroy']
  vendor_coupling:
    ignore_packages: ['my-framework/*']

# 特定路径的覆盖
overrides:
  - files: ['**/*.controller.ts']
    rules:
      lcom: off

# 死代码分析的模式
entry_points:
  - '**/*.controller.ts'
```

### 加载自定义预设

您可以从本地文件或 URL 加载预设：

```yaml
extends:
  - ./presets/shared.yaml
  - https://raw.githubusercontent.com/org/archlint-presets/main/standard.yaml
```

## 合并逻辑

预设按照指定的顺序合并。优先级为：

1. `.archlint.yaml` 中的用户配置（最高）
2. `extends` 列表中的预设
3. 自动检测的预设
4. archlint 默认设置（最低）

对于基于列表的设置（如 `entry_points` 或规则内的 `ignore_packages`），archlint 执行所有值的并集。规则和覆盖递归合并。

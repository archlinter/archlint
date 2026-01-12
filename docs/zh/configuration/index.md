---
title: 配置
description: 了解如何使用 .archlint.yaml 配置 archlint，定义架构层，并为检测器设置规则。
---

# 配置

archlint 可以通过项目根目录下的 `.archlint.yaml` 文件进行配置。如果未找到配置文件，该工具将对所有检测器使用合理的默认值。

## 配置文件结构

```yaml
# 要忽略的文件和目录（全局）
ignore:
  - '**/dist/**'
  - '**/node_modules/**'

# 路径别名（类似于 tsconfig.json 或 webpack）
# 默认情况下，archlint 会自动从 tsconfig.json 加载别名。
# 在此显式定义的别名优先级高于 tsconfig.json 中的别名。
aliases:
  '@/*': 'src/*'

# TypeScript 集成设置（true、false 或文件路径）
tsconfig: true

# 从内置或自定义预设扩展
extends:
  - nestjs
  - ./my-company-preset.yaml

# 分析的入口点（用于死代码检测）
entry_points:
  - 'src/main.ts'

# 每个检测器的规则配置
rules:
  # 短格式：严重程度级别或 "off"
  cycles: high
  dead_code: medium

  # 完整格式：带有额外选项
  god_module:
    severity: high
    enabled: true
    exclude: ['**/generated/**']
    # 检测器特定的选项
    fan_in: 15
    fan_out: 15
    churn: 20

  dead_symbols:
    severity: high
    # 匹配接口方法以避免未使用的实现产生误报
    contract_methods:
      MyInterface: ['method1', 'method2']
      ValidatorConstraintInterface: ['validate', 'defaultMessage']

  vendor_coupling:
    severity: medium
    ignore_packages: ['lodash', 'rxjs']

# 特定路径的规则覆盖
overrides:
  - files: ['**/legacy/**']
    rules:
      complexity: medium
      god_module: off

# 评分和分级配置
scoring:
  # 要报告的最低严重程度级别 (low, medium, high, critical)
  minimum: low
  # 总分计算的权重
  weights:
    critical: 100
    high: 50
    medium: 20
    low: 5
  # 分级阈值（密度 = 总分 / 文件数）
  grade_rules:
    excellent: 1.0
    good: 3.0
    fair: 7.0
    moderate: 15.0
    poor: 30.0

# 自动检测框架（默认为 true）
auto_detect_framework: true

# 架构 diff 设置
diff:
  # 指标恶化的百分比阈值（例如复杂度增长）
  metric_threshold_percent: 20
  # 模糊匹配中视为相同问题的最大行号偏移
  line_tolerance: 50

# Git 设置
git:
  enabled: true # 启用 Git 分析（默认为 true）
  history_period: '1y'
```

## 扩展 (Extends)

`extends` 字段允许您从不同的来源加载预设：

- **内置预设**：`nestjs`、`nextjs`、`express`、`react`、`angular`、`vue`、`typeorm`、`prisma`、`oclif`、`class-validator`。
- **本地文件**：YAML 文件的相对路径（例如 `./archlint-shared.yaml`）。
- **URL**：YAML 文件的直接 URL（例如 `https://example.com/preset.yaml`）。

预设按照列出的顺序合并。用户配置始终具有最高优先级。

## 规则和严重程度级别

在 `rules` 部分，您可以使用以下级别：

- `critical`: 需要立即关注的严重问题。
- `high`: 高严重性的架构问题。
- `medium`: 中等严重性的问题或警告。
- `low`: 低严重性的问题或信息性消息。
- `off`: 完全禁用检测器。

## CLI 配置

您可以显式指定配置文件路径：

```bash
archlint scan --config custom-config.yaml
```

## TypeScript 集成

archlint 可以自动与您的 `tsconfig.json` 同步。使用 `tsconfig` 字段来控制此功能：

- `tsconfig: true` (默认)：自动在项目根目录中查找 `tsconfig.json`。
- `tsconfig: false` 或 `tsconfig: null`：禁用 TypeScript 集成。
- `tsconfig: "./path/to/tsconfig.json"`：使用特定的配置文件。

启用后，该工具将：

1. **加载别名**：提取 `compilerOptions.paths` 和 `compilerOptions.baseUrl` 以自动配置 `aliases`。
2. **自动忽略**：将 `compilerOptions.outDir` 添加到全局 `ignore` 列表中。
3. **排除项**：将 `exclude` 字段中的模式纳入 `ignore` 列表。

## Diff 配置

`diff` 部分控制在比较两个快照时如何检测架构退化：

- **`metric_threshold_percent`** (默认值：`20`)：定义指标（如循环复杂度或耦合度）在被报告为“恶化”的问题之前必须增加多少。例如，阈值为 20% 时，函数的复杂度必须从 10 增加到至少 12 才会触发警告。
- **`line_tolerance`** (默认值：`50`)：定义代码符号在 archlint 停止将其识别为同一问题之前可以偏移的最大行数（由于文件中其他位置的添加或删除）。这种“模糊匹配”可防止偏移后的代码被报告为新的退化。

该工具在项目根目录中查找 `tsconfig.json`。如果您有自定义设置，请使用 `tsconfig` 字段指向正确的文件。

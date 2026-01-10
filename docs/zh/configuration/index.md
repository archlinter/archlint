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
# 默认情况下，archlint 会自动从 tsconfig.json 加载别名
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

# Git 设置
git:
  enabled: true # 启用 Git 分析（默认为 true）
  history_period: '1y'
```

## 扩展 (Extends)

`extends` 字段允许您从不同的来源加载预设：

- **内置预设**：`nestjs`、`nextjs`、`react`、`oclif`。
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

1. **加载别名**：提取 `compilerOptions.paths` 和 `compilerOptions.baseUrl` 以自动配置 `aliases`。
2. **自动忽略**：将 `compilerOptions.outDir` 添加到全局 `ignore` 列表中。
3. **排除项**：将 `exclude` 字段中的模式纳入 `ignore` 列表。

该工具在项目根目录中查找 `tsconfig.json`。如果您有自定义设置，请使用 `tsconfig` 字段指向正确的文件。

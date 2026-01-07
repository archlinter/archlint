---
title: 配置
description: 了解如何使用 archlint.yaml 配置 archlint，定义架构层，并为探测器设置自定义阈值。
---

# 配置

可以通过项目根目录下的 `archlint.yaml` 文件来配置 archlint。如果未找到配置文件，该工具将对所有探测器使用合理的默认值。

## 配置文件结构

```yaml
# 忽略的文件
ignore:
  - '**/dist/**'

# 路径别名 (例如，来自 tsconfig.json)
aliases:
  '@/*': 'src/*'

# 死代码分析的入口点
entry_points:
  - 'src/index.ts'

# 探测器的自定义阈值
thresholds:
  cycles:
    exclude_patterns: []
  god_module:
    fan_in: 15
    fan_out: 15

# 架构层
layers:
  - name: domain
    paths: ['**/domain/**']
    can_import: []

# 框架预设
frameworks:
  - nestjs

# 严重程度覆盖
severity:
  cycles: critical
```

## CLI 配置

您还可以通过 CLI 指定配置文件路径：

```bash
archlint scan --config custom-config.yaml
```

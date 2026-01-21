---
title: 枢纽依赖
description: "检测被过多文件导入的外部包，这会创建单点故障并使升级变得困难。"
---

# 枢纽依赖 (Hub Dependency)

**ID:** `hub_dependency` | **严重程度:** 中 (默认)

识别被项目中过多文件导入的外部包，从而创建单点故障。

## 为什么这是一种坏味道

当您的项目过度依赖单个外部库时，替换或升级该库将变得困难。这也表明您可能将基础设施细节泄漏到了应用程序逻辑中。

## 配置

```yaml
rules:
  hub_dependency:
    severity: medium
    min_dependents: 20
    ignore_packages:
      - 'react'
      - 'lodash'
      - 'typescript'
```

### 选项

- `min_dependents`（默认：20）：触发此坏味道所需的导入包的文件最小数量。
- `ignore_packages`：要忽略的包名称或 glob 模式列表。

## 如何修复

识别包为何如此广泛使用。如果是像 `lodash` 这样的工具库，请考虑是否真的需要所有这些导入，或者是否可以使用原生语言功能。对于基础设施库，使用**适配器模式 (Adapter Pattern)** 来隔离依赖。

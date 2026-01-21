---
title: 厂商耦合
description: "检测与特定外部库耦合过于紧密的模块，这会使未来的迁移变得困难并增加测试难度。"
---

# 厂商耦合 (Vendor Coupling)

**ID:** `vendor_coupling` | **严重程度:** 中 (默认)

识别与特定外部库或框架耦合过于紧密的模块。

## 为什么这是一种坏味道

如果您决定将来更换该库，您将不得不修改多处的代码。这也会使测试变得更加困难，因为您必须在各处对外部库进行 Mock。

## 如何修复

使用**适配器模式 (Adapter Pattern)**。在您的领域中创建一个接口，并使用外部库实现它。代码的其余部分应该只依赖于您的接口。

## 配置

```yaml
rules:
  vendor_coupling:
    severity: medium
    max_files_per_package: 10
    ignore_packages:
      - 'lodash'
      - 'rxjs'
      - '@nestjs/*'
```

### 选项

- `max_files_per_package`（默认：10）：在报告坏味道之前可以导入特定包的最大文件数。
- `ignore_packages`：要忽略的包名称或 glob 模式列表。

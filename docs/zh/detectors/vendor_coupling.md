# 厂商耦合 (Vendor Coupling)

**ID:** `vendor_coupling` | **严重程度:** 中 (默认)

识别那些与特定外部库或框架"结婚"的模块。

## 为什么这是一种坏味道

- **厂商锁定**: 如果该库变得过时，或者你决定切换到更好的替代品，你将不得不重写你代码库的一半。
- **测试摩擦**: 如果不引入沉重的外部库及其 mock，你无法测试业务逻辑。
- **难以升级**: 你被困在库支持的任何版本上，因为它已经编织到每个文件中。

## 如何修复

使用**适配器模式 (Adapter Pattern)**。在你的领域中创建一个接口，并使用外部库实现它。代码的其余部分应该只依赖于你的接口。

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

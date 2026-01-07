# 大文件

**ID:** `large_file` | **Severity:** Medium (default)

识别超过特定行数的源文件。

## 为什么这是一种坏味道

极大的文件难以浏览、理解和维护。它们通常表明违反了单一职责原则。

## 如何修复

将文件拆分为更小、更集中的模块。

## 配置

```yaml
thresholds:
  large_file:
    max_lines: 1000
```

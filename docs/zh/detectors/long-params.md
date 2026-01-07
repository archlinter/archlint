# 过长参数列表

**ID:** `long_params` | **Severity:** Low (default)

识别具有过多参数的函数或方法。

## 为什么这是一种坏味道

参数过多的函数难以使用且难以阅读。它们通常表明函数承担了过多的职责，或者某些参数应该被组合成一个对象。

## 如何修复

- **Introduce Parameter Object (引入参数对象)**：将相关的参数组合成一个对象或接口。
- **Decompose Function (分解函数)**：将函数分解为需要更少参数的小函数。

## 配置

```yaml
thresholds:
  long_params:
    max_params: 5
```

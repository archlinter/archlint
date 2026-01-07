# 高复杂度 (High Complexity)

**ID:** `complexity` | **严重程度:** 中 (默认)

该检测器识别具有高圈复杂度 (Cyclomatic Complexity) 的函数。

## 为什么这是一种坏味道

- **难以理解**: 过多的分支路径使代码难以追踪。
- **易出错**: 在测试期间漏掉边缘情况的可能性更高。
- **维护噩梦**: 由于复杂的逻辑，微小的更改可能会产生不可预测的影响。

## 如何修复

1. **提取方法 (Extract Method)**: 将复杂的逻辑分解为更小的命名函数。
2. **卫语句 (Guard Clauses)**: 使用早期返回来减少嵌套层级。
3. **用多态替换条件语句 (Replace Conditional with Polymorphism)**: 使用对象或策略模式代替大型 `switch` 或 `if/else` 块。

## 配置

```yaml
thresholds:
  complexity:
    max_complexity: 15
```

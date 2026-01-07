# 过深嵌套

**ID:** `deep_nesting` | **Severity:** Low (default)

识别嵌套过深的代码块（if、for、while 等）。

## 为什么这是一种坏味道

过深嵌套的代码阅读和理解难度会呈指数级增加。这通常意味着一个函数承担了过多的职责，或者逻辑可以被简化。

## 如何修复

- **Guard Clauses（卫语句）**：尽早返回以避免 `else` 块并减少嵌套。
- **Extract Function（提取函数）**：将内部嵌套块移动到新函数中。
- **Flatten Logic（打平逻辑）**：重新评估逻辑，看是否可以更简单地表达。

## 配置

```yaml
thresholds:
  deep_nesting:
    max_depth: 4
```

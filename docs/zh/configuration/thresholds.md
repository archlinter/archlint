# 阈值

阈值允许您微调探测器报告坏味道的时机。

## 常见阈值

| 探测器         | 选项               | 默认值 | 描述                             |
| -------------- | ------------------ | ------ | -------------------------------- |
| `cycles`       | `exclude_patterns` | `[]`   | 循环依赖检测中要忽略的 Glob 模式 |
| `god_module`   | `fan_in`           | `10`   | 最大入向依赖数                   |
| `god_module`   | `fan_out`          | `10`   | 最大出向依赖数                   |
| `god_module`   | `churn`            | `20`   | Git 历史记录中的最大提交数       |
| `god_module`   | `max_lines`        | `500`  | 文件的最大代码行数               |
| `complexity`   | `max_complexity`   | `15`   | 每个函数的最大圈复杂度           |
| `deep_nesting` | `max_depth`        | `4`    | 块的最大嵌套深度                 |
| `long_params`  | `max_params`       | `5`    | 每个函数的最大参数个数           |
| `large_file`   | `max_lines`        | `1000` | 每个文件的最大行数               |
| `lcom`         | `threshold`        | `1`    | 类中允许的最大未连接组件数       |

## 配置示例

```yaml
thresholds:
  god_module:
    fan_in: 20
    max_lines: 800

  complexity:
    max_complexity: 10

  large_file:
    max_lines: 2000
```

# Thresholds

Thresholds allow you to fine-tune when a detector should report a smell.

## Common Thresholds

| Detector       | Option             | Default | Description                                   |
| -------------- | ------------------ | ------- | --------------------------------------------- |
| `cycles`       | `exclude_patterns` | `[]`    | Glob patterns to ignore in cycle detection    |
| `god_module`   | `fan_in`           | `10`    | Max incoming dependencies                     |
| `god_module`   | `fan_out`          | `10`    | Max outgoing dependencies                     |
| `god_module`   | `churn`            | `20`    | Max git commits in history                    |
| `god_module`   | `max_lines`        | `500`   | Max lines of code in file                     |
| `complexity`   | `max_complexity`   | `15`    | Max cyclomatic complexity per function        |
| `deep_nesting` | `max_depth`        | `4`     | Max nesting depth for blocks                  |
| `long_params`  | `max_params`       | `5`     | Max parameters per function                   |
| `large_file`   | `max_lines`        | `1000`  | Max lines per file                            |
| `lcom`         | `threshold`        | `1`     | Max allowed unconnected components in a class |

## Example Configuration

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

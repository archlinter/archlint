# archlint scan

`scan` 命令对项目执行完整的架构分析。

## 用法

```bash
archlint scan [path] [options]
```

## 选项

| 选项                        | 默认值   | 描述                                                |
| --------------------------- | -------- | --------------------------------------------------- |
| `--format <format>`         | `table`  | 输出格式：`table`, `json`, `markdown`               |
| `--report <file>`           | `stdout` | 将报告保存到文件                                    |
| `--min-severity <sev>`      | `low`    | 按严重程度过滤：`low`, `medium`, `high`, `critical` |
| `--detectors <ids>`         | `all`    | 要运行的探测器 ID 列表（以逗号分隔）                |
| `--exclude-detectors <ids>` | `none`   | 要跳过的探测器                                      |
| `--no-cache`                | `false`  | 禁用分析缓存                                        |

## 示例

### 使用 Markdown 报告进行扫描

```bash
archlint scan --format markdown --report report.md
```

### 仅运行循环依赖检测

```bash
archlint scan --detectors cycles,circular_type_deps
```

### 仅限高严重程度

```bash
archlint scan --min-severity high
```

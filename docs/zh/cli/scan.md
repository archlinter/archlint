---
title: scan
description: '使用 archlint scan 命令对项目执行完整的架构分析，支持多种输出格式，如 JSON、Markdown 和 SARIF。'
---

# archlint scan

`scan` 命令对项目执行完整的架构分析。

## 用法

```bash
archlint scan [path] [options]
```

## 选项

| 选项                            | 默认值   | 描述                                                    |
| ------------------------------- | -------- | ------------------------------------------------------- |
| `-f, --format <format>`         | `table`  | 输出格式：`table`, `json`, `markdown`, `sarif`, `junit` |
| `-j, --json`                    | `false`  | `--format json` 的快捷方式                              |
| `-r, --report <file>`           | `stdout` | 将报告保存到文件                                        |
| `-s, --min-severity <sev>`      | `low`    | 按严重程度过滤：`low`, `medium`, `high`, `critical`     |
| `-S, --min-score <score>`       | `none`   | 按最低健康分过滤                                        |
| `-d, --detectors <ids>`         | `all`    | 要运行的探测器 ID 列表（以逗号分隔）                    |
| `-e, --exclude-detectors <ids>` | `none`   | 要跳过的探测器                                          |
| `-A, --all`                     | `false`  | 运行所有探测器（包括默认禁用的探测器）                  |
| `--no-cache`                    | `false`  | 禁用分析缓存                                            |
| `--no-git`                      | `false`  | 禁用 git 集成 (跳过 churn 分析)                         |

## 示例

### 使用 Markdown 报告进行扫描

```bash
archlint scan --format markdown --report report.md
```

### 导出为 SARIF (用于 GitHub Code Scanning)

```bash
archlint scan --format sarif --report results.sarif
```

### 导出为 JUnit（用于 CI 测试报告）

```bash
archlint scan --format junit --report archlint-junit.xml
```

GitLab 可以将该文件发布为测试报告。参见 [GitLab CI](/zh/integrations/gitlab-ci#junit-测试报告)。

### 仅运行循环依赖检测

```bash
archlint scan --detectors cycles,circular_type_deps
```

### 仅限高严重程度

```bash
archlint scan --min-severity high
```

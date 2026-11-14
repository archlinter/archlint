---
title: scan
description: 'Perform a complete architectural analysis of your project with the archlint scan command, supporting multiple output formats like JSON, Markdown, SARIF, and JUnit.'
---

# archlint scan

The `scan` command performs a complete architectural analysis of your project.

## Usage

```bash
archlint scan [path] [options]
```

## Options

| Option                          | Default  | Description                                                  |
| ------------------------------- | -------- | ------------------------------------------------------------ |
| `-f, --format <format>`         | `table`  | Output format: `table`, `json`, `markdown`, `sarif`, `junit` |
| `-j, --json`                    | `false`  | Shortcut for `--format json`                                 |
| `-r, --report <file>`           | `stdout` | Save the report to a file                                    |
| `-s, --min-severity <sev>`      | `low`    | Filter by severity: `low`, `medium`, `high`, `critical`      |
| `-S, --min-score <score>`       | `none`   | Filter by minimum health score                               |
| `-d, --detectors <ids>`         | `all`    | Comma-separated list of detectors to run                     |
| `-e, --exclude-detectors <ids>` | `none`   | Detectors to skip                                            |
| `-A, --all`                     | `false`  | Run all detectors (including disabled by default)            |
| `--no-cache`                    | `false`  | Disable analysis caching                                     |
| `--no-git`                      | `false`  | Disable git integration (skip churn analysis)                |

## Examples

### Scan with Markdown report

```bash
archlint scan --format markdown --report report.md
```

### Export to SARIF (for GitHub Code Scanning)

```bash
archlint scan --format sarif --report results.sarif
```

### Export to JUnit (for CI test reports)

```bash
archlint scan --format junit --report archlint-junit.xml
```

GitLab, Jenkins, and Azure DevOps can publish this file as a unit-test report. Each smell is a failed test case, grouped by detector. A clean scan emits one passing test case. See [GitLab CI](/integrations/gitlab-ci#junit-test-reports) for a pipeline example.

### Only run cycle detection

```bash
archlint scan --detectors cycles,circular_type_deps
```

### High severity only

```bash
archlint scan --min-severity high
```

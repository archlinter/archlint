# archlint scan

The `scan` command performs a complete architectural analysis of your project.

## Usage

```bash
archlint scan [path] [options]
```

## Options

| Option                      | Default  | Description                                             |
| --------------------------- | -------- | ------------------------------------------------------- |
| `--format <format>`         | `table`  | Output format: `table`, `json`, `markdown`              |
| `--report <file>`           | `stdout` | Save the report to a file                               |
| `--min-severity <sev>`      | `low`    | Filter by severity: `low`, `medium`, `high`, `critical` |
| `--detectors <ids>`         | `all`    | Comma-separated list of detectors to run                |
| `--exclude-detectors <ids>` | `none`   | Detectors to skip                                       |
| `--no-cache`                | `false`  | Disable analysis caching                                |

## Examples

### Scan with Markdown report

```bash
archlint scan --format markdown --report report.md
```

### Only run cycle detection

```bash
archlint scan --detectors cycles,circular_type_deps
```

### High severity only

```bash
archlint scan --min-severity high
```

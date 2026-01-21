---
title: diff
description: "Compare your current codebase against a baseline to detect new architectural regressions and worsened smells, supporting the Ratchet philosophy."
---

# archlint diff

The `diff` command is the heart of the "Ratchet" approach. It compares your current codebase against a previously saved snapshot or another git branch/commit.

## Usage

```bash
# Compare against a snapshot file
archlint diff <baseline.json> [options]

# Compare against a git ref
archlint diff <git-ref> [options]
```

## How it works

archlint doesn't just count issues. It performs a **semantic diff** of the architectural smells:

1. **New smells**: Smells that exist now but didn't exist in the baseline (e.g., a new cycle).
2. **Worsened smells**: Existing smells that have become more severe (e.g., a cycle grew from 3 files to 5).
3. **Fixed smells**: Smells that existed in the baseline but are now gone.

## Options

| Option                 | Default | Description                                                          |
| ---------------------- | ------- | -------------------------------------------------------------------- |
| `-j, --json`           | `false` | Output report in JSON format                                         |
| `-v, --verbose`        | `false` | Enable verbose output                                                |
| `-p, --path <path>`    | `.`     | Project path                                                         |
| `--fail-on <severity>` | `low`   | Exit with code 1 if a regression of this severity or higher is found |
| `--explain`            | `false` | Provide a detailed explanation for each regression                   |

## Configuration

You can fine-tune the diff engine in your `.archlint.yaml` file:

```yaml
diff:
  metric_threshold_percent: 20 # report as regression only if metric worsened by >20%
  line_tolerance: 50 # ignore shifts within 50 lines during fuzzy matching
```

See [Configuration Guide](/configuration/index#diff-configuration) for details.

## Examples

### Check against main branch in CI

```bash
archlint diff origin/main --fail-on low --explain
```

### Check against a local baseline

```bash
archlint diff .archlint-baseline.json
```

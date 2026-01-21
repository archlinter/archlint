---
title: Configuration
description: "Learn how to configure archlint using .archlint.yaml, define architectural layers, and configure rules for detectors."
---

# Configuration

archlint can be configured using an `.archlint.yaml` file in your project root. If no configuration file is found, the tool uses sensible defaults for all detectors.

## Configuration File Structure

```yaml
# Files and directories to ignore (global)
ignore:
  - '**/dist/**'
  - '**/node_modules/**'

# Path aliases (similar to tsconfig.json or webpack)
# By default, archlint automatically loads aliases from tsconfig.json.
# Explicit aliases defined here take precedence over tsconfig-derived values.
aliases:
  '@/*': 'src/*'

# TypeScript integration settings (true, false, or path to file)
tsconfig: true

# Extend from built-in or custom presets
extends:
  - nestjs
  - ./my-company-preset.yaml

# Entry points for analysis (used for dead code detection)
entry_points:
  - 'src/main.ts'

# Rules configuration for each detector
rules:
  # Short form: severity level or "off"
  cycles: high
  dead_code: medium
  cyclomatic_complexity: low
  cognitive_complexity: high

  # Full form: with additional options
  god_module:
    severity: high
    enabled: true
    exclude: ['**/generated/**']
    # Detector-specific options
    fan_in: 15
    fan_out: 15
    churn: 20

  vendor_coupling:
    severity: medium
    ignore_packages: ['lodash', 'rxjs']

  dead_symbols:
    severity: high
    # Match interface methods to avoid false positives for unused implementations
    contract_methods:
      MyInterface: ['method1', 'method2']
      ValidatorConstraintInterface: ['validate', 'defaultMessage']

# Rule overrides for specific paths
overrides:
  - files: ['**/legacy/**']
    rules:
      cyclomatic_complexity: medium
      cognitive_complexity: high
      god_module: off

# Scoring and grading configuration
scoring:
  # Minimum severity level to report (low, medium, high, critical)
  minimum: low
  # Weights for total score calculation
  weights:
    critical: 100
    high: 50
    medium: 20
    low: 5
  # Thresholds for grading (Density = Total Score / Files)
  grade_rules:
    excellent: 1.0
    good: 3.0
    fair: 7.0
    moderate: 15.0
    poor: 30.0

# Auto-detect framework (defaults to true)
auto_detect_framework: true

# Architectural diff settings
diff:
  # Threshold for metric worsening (e.g., complexity growth) to be reported
  metric_threshold_percent: 20
  # Maximum line shift for matching smells between versions during fuzzy diff
  line_tolerance: 50

# Git settings
git:
  enabled: true # default: true
  history_period: '1y'
```

## Extends

The `extends` field allows you to load presets from different sources:

- **Built-in presets**: `nestjs`, `nextjs`, `express`, `react`, `angular`, `vue`, `typeorm`, `prisma`, `oclif`, `class-validator`.
- **Local files**: Relative path to a YAML file (e.g., `./archlint-shared.yaml`).
- **URLs**: Direct URL to a YAML file (e.g., `https://example.com/preset.yaml`).

Presets are merged in the order they are listed. User configuration always has the highest priority.

For remote presets (via URL), the following constraints apply:

- **Security**: Requests to local or private networks (localhost, 127.0.0.1, 10.x.x.x, 192.168.x.x, 172.16-31.x.x) are blocked for SSRF protection.
- **Timeout**: Preset loading has a 30-second timeout. If the server does not respond in time or the URL is unreachable, an error will be reported.
- **Validation**: Only `http` and `https` schemes are supported. Malformed URLs will cause a configuration error.

## Rules and Severity Levels

In the `rules` section, you can use the following severity levels:

- `critical`: Critical issue requiring immediate attention.
- `high`: High-severity architectural issue.
- `medium`: Medium-severity issue or warning.
- `low`: Low-severity or informational message.
- `off`: Completely disables the detector.

## CLI Configuration

You can specify the configuration file path explicitly:

```bash
archlint scan --config custom-config.yaml
```

## TypeScript Integration

archlint can automatically synchronize with your `tsconfig.json`. Use the `tsconfig` field to control this:

- `tsconfig: true` (default): Automatically searches for `tsconfig.json` in the project root.
- `tsconfig: false` or `tsconfig: null`: Disables TypeScript integration.
- `tsconfig: "./path/to/tsconfig.json"`: Uses a specific configuration file.

When enabled, the tool:

1. **Loads Aliases**: Extracts `compilerOptions.paths` and `compilerOptions.baseUrl` to automatically configure `aliases`.
2. **Auto-ignore**: Adds `compilerOptions.outDir` to the global `ignore` list.
3. **Excludes**: Incorporates patterns from the `exclude` field into the `ignore` list.

## Diff Configuration

The `diff` section controls how architectural regressions are detected when comparing two snapshots:

- **`metric_threshold_percent`** (default: `20`): Defines how much a metric (like cyclomatic/cognitive complexity or coupling) must increase before it is reported as a "worsened" smell. For example, with a threshold of 20%, a function's cyclomatic complexity must increase from 10 to at least 12 to be flagged.
- **`line_tolerance`** (default: `50`): Defines the maximum number of lines a code symbol can shift (due to additions or deletions elsewhere in the file) before archlint stops recognizing it as the same smell. This "fuzzy matching" prevents shifted code from being reported as a new regression.

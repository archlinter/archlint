---
title: Configuration
description: Learn how to configure archlint using .archlint.yaml, define architectural layers, and configure rules for detectors.
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
aliases:
  '@/*': 'src/*'

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
  cycles: error
  dead_code: warn

  # Full form: with additional options
  god_module:
    severity: error
    enabled: true
    exclude: ['**/generated/**']
    # Detector-specific options
    fan_in: 15
    fan_out: 15
    churn: 20

  vendor_coupling:
    severity: warn
    ignore_packages: ['lodash', 'rxjs']

# Rule overrides for specific paths
overrides:
  - files: ['**/legacy/**']
    rules:
      complexity: warn
      god_module: off

# Scoring and grading configuration
scoring:
  # Minimum severity level to report (info, warn, error, critical)
  minimum: warn
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

# Enable Git history analysis (defaults to true)
enable_git: true

# Git settings
git:
  history_period: '1y'
```

## Extends

The `extends` field allows you to load presets from different sources:

- **Built-in presets**: `nestjs`, `nextjs`, `react`, `oclif`.
- **Local files**: Relative path to a YAML file (e.g., `./archlint-shared.yaml`).
- **URLs**: Direct URL to a YAML file (e.g., `https://example.com/preset.yaml`).

Presets are merged in the order they are listed. User configuration always has the highest priority.

## Rules and Severity Levels

In the `rules` section, you can use the following severity levels:

- `critical`: Critical issue requiring immediate attention.
- `error`: Architectural error.
- `warn`: Warning about a potential issue.
- `info`: Informational message.
- `off`: Completely disables the detector.

## CLI Configuration

You can specify the configuration file path explicitly:

```bash
archlint scan --config custom-config.yaml
```

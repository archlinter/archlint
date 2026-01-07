---
title: Configuration
description: Learn how to configure archlint using archlint.yaml, define architectural layers, and set custom thresholds for detectors.
---

# Configuration

archlint can be configured using an `archlint.yaml` file in your project root. If no configuration file is found, the tool uses sensible defaults for all detectors.

## Configuration File Structure

```yaml
# Files to ignore
ignore:
  - '**/dist/**'

# Path aliases (e.g., from tsconfig.json)
aliases:
  '@/*': 'src/*'

# Entry points for dead code analysis
entry_points:
  - 'src/index.ts'

# Custom thresholds for detectors
thresholds:
  cycles:
    exclude_patterns: []
  god_module:
    fan_in: 15
    fan_out: 15

# Architectural layers
layers:
  - name: domain
    paths: ['**/domain/**']
    can_import: []

# Framework presets
frameworks:
  - nestjs

# Severity overrides
severity:
  cycles: critical
```

## CLI Configuration

You can also specify the configuration file path via the CLI:

```bash
archlint scan --config custom-config.yaml
```

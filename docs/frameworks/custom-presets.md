# Framework Presets

archlint uses YAML-based presets to understand framework-specific patterns and reduce false positives.

## How it works

archlint automatically detects frameworks by analyzing `package.json` dependencies and configuration files. You can also explicitly extend presets in your `.archlint.yaml`:

```yaml
extends:
  - nestjs
  - ./my-company-preset.yaml
```

## Built-in Presets

- **nestjs**: For NestJS applications.
- **nextjs**: For Next.js projects.
- **react**: For React libraries and applications.
- **oclif**: For CLI tools built with oclif.

## Custom Presets

A preset file is a standard archlint configuration file with an additional `detect` section for auto-discovery.

### Structure

```yaml
name: my-framework
version: 1

# Rules for auto-detection
detect:
  packages:
    any_of: ['my-core-pkg']
  files:
    any_of: ['my-framework.config.js']

# Global rules
rules:
  layer_violation: high
  dead_symbols:
    ignore_methods: ['onInit', 'onDestroy']
  vendor_coupling:
    ignore_packages: ['my-framework/*']

# Path-specific overrides
overrides:
  - files: ['**/*.controller.ts']
    rules:
      lcom: off

# Patterns for dead code analysis
entry_points:
  - '**/*.controller.ts'
```

### Loading Custom Presets

You can load presets from local files or URLs:

```yaml
extends:
  - ./presets/shared.yaml
  - https://raw.githubusercontent.com/org/archlint-presets/main/standard.yaml
```

## Merging Logic

Presets are merged in the order they are specified. The priority is:

1. User configuration in `.archlint.yaml` (highest)
2. Presets in `extends` list
3. Auto-detected presets
4. Default archlint settings (lowest)

For list-based settings (like `entry_points` or `ignore_packages` within rules), archlint performs a union of all values. Rules and overrides are merged recursively.

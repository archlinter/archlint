# Ignoring Files

archlint provides several ways to exclude files or directories from analysis.

## Global Ignore

The `ignore` section at the root of `.archlint.yaml` specifies files that should be completely skipped by all detectors.

```yaml
ignore:
  - '**/node_modules/**'
  - '**/dist/**'
  - '**/coverage/**'
  - '**/tmp/**'
  - '**/*.d.ts'
```

## .gitignore Support

By default, archlint automatically respects your `.gitignore` file. You don't need to duplicate these patterns in `.archlint.yaml`. If you want to disable this behavior, set `enable_git: false`.

## Per-Rule Ignore

You can exclude files from a specific detector using the `exclude` field inside the `rules` section. This is useful if you want a file to be analyzed by most detectors but skipped by one specific detector.

```yaml
rules:
  cycles:
    exclude:
      - '**/generated/**'
      - '**/*.entity.ts'
```

## Path Overrides

For more complex logic (e.g., changing settings or disabling multiple rules for a specific directory), use the `overrides` section:

```yaml
overrides:
  - files: ['**/tests/**', '**/mocks/**']
    rules:
      complexity: off
      god_module: off
      large_file: warn
```

## Inline Ignore

(In Development) We are working on supporting comments like `// archlint-disable` to ignore specific lines or files directly in the code.

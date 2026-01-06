# Ignore Patterns

archlint provides several ways to exclude files or directories from analysis.

## Global Ignore

The `ignore` section in `archlint.yaml` specifies files that should be completely skipped by all detectors.

```yaml
ignore:
  - '**/node_modules/**'
  - '**/dist/**'
  - '**/coverage/**'
  - '**/tmp/**'
  - '**/*.d.ts'
```

## .gitignore Support

By default, archlint automatically respects your `.gitignore` file. You don't need to duplicate those patterns in your `archlint.yaml`.

## Detector-Specific Ignore

Some detectors have their own `exclude_patterns` within the `thresholds` section. This is useful if you want a file to be analyzed by most detectors but skipped by a specific one (e.g., excluding test files from cycle detection).

```yaml
thresholds:
  cycles:
    exclude_patterns:
      - '**/*.test.ts'
      - '**/*.spec.ts'
```

## Inline Ignores

(Coming Soon) We are working on supporting inline comments like `// archlint-disable` to ignore specific lines or files directly in the source code.

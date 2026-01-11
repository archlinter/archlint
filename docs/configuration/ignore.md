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

By default, archlint automatically respects your `.gitignore` file. You don't need to duplicate these patterns in `.archlint.yaml`. If you want to disable this behavior, set `git: { enabled: false }`.

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
      large_file: medium
```

## Inline Ignore

You can ignore specific architectural smells directly in your source code using special comments. This is useful for suppressing warnings in exceptional cases.

### Usage:

1. **Whole File**: Add `// archlint-disable` at the top of the file.
2. **Current Line**: Add `// archlint-disable-line` at the end of the line or on the line above.
3. **Next Line**: Use `// archlint-disable-next-line` before the problematic line.
4. **Blocks**: Use `// archlint-disable` and `// archlint-enable` to wrap a section of code.

### Examples:

```typescript
// prettier-ignore
// archlint-disable-next-line long-params
function processTransaction(id: string, amount: number, currency: string, date: Date, recipient: string, note: string) {
  // Long params detector will be ignored only for this line
}

import { internal } from './private'; // archlint-disable-line layer_violation

// archlint-disable cycles, god_module
// Ignore specific rules for the entire file

/* archlint-disable complexity */
function legacyCode() {
  // This block is ignored
}
/* archlint-enable complexity */
```

You can specify multiple rules separated by commas or use `*` to ignore all rules.

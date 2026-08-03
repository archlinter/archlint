---
title: Ignoring Files
description: 'Learn how to exclude files or directories from archlint analysis using global ignore, gitignore, rule-specific excludes, and inline comments.'
---

# Ignoring Files

archlint provides several ways to exclude files or directories from analysis.

## Global Ignore

The `ignore` section at the root of `.archlint.yaml` excludes files from analysis. Ignored files are still parsed, so imports through them keep resolving, but no smell is reported for them and they do not count towards any detector threshold (for example `min_dependents` or `max_files_per_package`).

```yaml
ignore:
  - '**/node_modules/**'
  - '**/dist/**'
  - '**/coverage/**'
  - '**/tmp/**'
  - '**/*.d.ts'
```

Patterns are matched against paths relative to the project root:

- A plain path — `legacy`, `src/legacy`, `generated.ts` — matches that entry and everything below it, at any depth.
- A pattern containing `*`, `?` or `[` is used as a glob. Note that `*` also matches `/`, so `src/*.ts` covers `src/a/b/c.ts`.
- Unusable patterns are reported as warnings and skipped; the remaining patterns still apply.

Cycles are the one exception to the rule above: a cycle is a fact about a group of files, so a cycle running through both live and ignored files is still reported, with its ignored members listed. A cycle whose members are all ignored is not reported.

Setting `ignore` **replaces** the built-in defaults (`**/*.test.ts`, `**/__tests__/**` and the other test-file patterns) instead of adding to them. Repeat the ones you need if you still want test files excluded.

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
      cyclomatic_complexity: off
      cognitive_complexity: off
      god_module: off
      large_file: medium
```

## Inline Ignore

You can ignore specific architectural smells directly in your source code using special comments. This is useful for suppressing warnings in exceptional cases.

### Usage:

Both single-line (`// archlint-...`) and block comment (`/* archlint-... */`) syntaxes are supported for all patterns.

1. **Whole File**: Add `// archlint-disable` at the top of the file.
2. **Current Line**: Add `// archlint-disable-line` at the end of the line or on the line above.
3. **Next Line**: Use `// archlint-disable-next-line` before the problematic line.
4. **Blocks**: Use `// archlint-disable` and `// archlint-enable` to wrap a section of code.

### Examples:

```typescript
// archlint-disable * - Entire file uses legacy patterns
// Ignore all rules for the entire file

// prettier-ignore
// archlint-disable-next-line long-params - This legacy function requires many parameters
function processTransaction(id: string, amount: number, currency: string, date: Date, recipient: string, note: string) {
  // Long params detector will be ignored only for this line
}

import { internal } from './private'; // archlint-disable-line layer_violation - Temporary exclusion for migration

/* archlint-disable cyclomatic_complexity, cognitive_complexity */
function legacyCode() {
  // This block is ignored for both complexity types
}
/* archlint-enable cyclomatic_complexity, cognitive_complexity */
```

You can specify multiple rules separated by commas or use `*` to ignore all rules.

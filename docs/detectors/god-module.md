# God Module

**ID:** `god_module` | **Severity:** High (default)

A "God Module" is a file that has grown too large and taken on too many responsibilities.

## Why this is a smell

- **Violates Single Responsibility Principle**: The module does too many things.
- **Merge Conflicts**: Frequent changes by different developers lead to constant conflicts.
- **Fragility**: Changes in one part of the module might unexpectedly break another part.
- **Hard to Test**: Requires complex setup to test various unrelated functionalities.

## Detection Criteria

archlint identifies God Modules based on:

- **Fan-in**: Number of other modules depending on it.
- **Fan-out**: Number of modules it depends on.
- **Churn**: Frequency of changes in git.
- **Lines of Code**: Total size of the file.

## How to fix

1. **Identify Responsibilities**: List all different tasks the module performs.
2. **Extract Modules**: Break the file into smaller, focused modules.
3. **Facade Pattern**: If the module acts as a coordinator, keep only the coordination logic and delegate the work to sub-modules.

## Configuration

```yaml
thresholds:
  god_module:
    fan_in: 15
    fan_out: 15
    churn: 20
    max_lines: 500
```

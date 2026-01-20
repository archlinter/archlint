# Large File

**ID:** `large_file` | **Severity:** Medium (default)

Identifies source files that have grown so large they should probably have their own zip code.

## Why this is a smell

Extremely large files are a nightmare to navigate. You spend more time scrolling and searching for symbols than actually writing code. Usually, a 2000-line file is just three or four smaller, logical modules wearing a trench coat. It violates the Single Responsibility Principle and makes merge conflicts almost guaranteed.

## How to fix

Break the file into smaller, more focused modules.

## Configuration

```yaml
rules:
  large_file:
    severity: medium
    max_lines: 1000
```

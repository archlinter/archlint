---
title: Large File
description: "Identify source files that exceed a certain number of lines, indicating violations of the Single Responsibility Principle."
---

# Large File

**ID:** `large_file` | **Severity:** Medium (default)

Identifies source files that exceed a certain number of lines.

## Why this is a smell

Extremely large files are hard to navigate, understand, and maintain. They usually indicate a violation of the Single Responsibility Principle.

## How to fix

Break the file into smaller, more focused modules.

## Configuration

```yaml
rules:
  large_file:
    severity: medium
    max_lines: 1000
```

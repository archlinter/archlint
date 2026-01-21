---
title: Hub Dependency
description: "Detect external packages imported by too many files, creating central points of failure and making upgrades difficult."
---

# Hub Dependency

**ID:** `hub_dependency` | **Severity:** Medium (default)

Identifies external packages that are imported by too many files in your project, creating a central point of failure.

## Why this is a smell

When your project depends too heavily on a single external library, it becomes difficult to replace or upgrade that library. It also suggests that you might be leaking infrastructure details into your application logic.

## Configuration

```yaml
rules:
  hub_dependency:
    severity: medium
    min_dependents: 20
    ignore_packages:
      - 'react'
      - 'lodash'
      - 'typescript'
```

### Options

- `min_dependents` (default: 20): The minimum number of files importing a package to trigger this smell.
- `ignore_packages`: A list of package names or glob patterns to ignore.

## How to fix

Identify why the package is used so widely. If it's a utility library like `lodash`, consider if you really need all those imports or if you can use native language features. For infrastructure libraries, use the **Adapter Pattern** to isolate the dependency.

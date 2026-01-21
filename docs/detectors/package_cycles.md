---
title: Package Cycles
description: "Detect circular dependencies between entire packages that prevent proper versioning and indicate serious modularity flaws."
---

# Package Cycles

**ID:** `package_cycles` | **Severity:** High (default)

Detects circular dependencies between entire packages (folders with `package.json` or logical module boundaries).

## Why this is a smell

Circular dependencies at the package level are even more severe than file-level cycles. They prevent proper versioning, make it impossible to publish packages independently, and indicate a serious flaw in the system's modularity.

## How to fix

Re-evaluate the boundaries between your packages. Often, a package cycle means that two packages should actually be one, or that a third package should be extracted to hold the shared code.

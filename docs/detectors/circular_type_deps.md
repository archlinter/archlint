---
title: Type Cycles
description: "Detect circular dependencies in type-only imports that indicate tight architectural coupling, even though they don't cause runtime issues."
---

# Type Cycles

**ID:** `circular_type_deps` | **Severity:** Medium (default)

Similar to circular dependencies, but specifically for type-only imports (e.g., `import type { ... }`).

## Why this is a smell

While type-only cycles don't cause runtime issues in TypeScript, they still indicate tight architectural coupling. They make it harder to separate modules and can still lead to complex dependency graphs that are hard to reason about.

## How to fix

Move the shared types to a common `types` module or a separate file that doesn't depend on the implementation modules.

---
title: Long Parameter List
description: "Detect functions with too many parameters that are hard to use and read, indicating functions doing too much."
---

# Long Parameter List

**ID:** `long_params` | **Severity:** Low (default)

Identifies functions or methods that have too many parameters.

## Why this is a smell

Functions with many parameters are hard to use and hard to read. They often indicate that the function is doing too much or that some parameters should be grouped into an object.

## How to fix

- **Introduce Parameter Object**: Group related parameters into a single object or interface.
- **Decompose Function**: Split the function into smaller ones that require fewer parameters.

## Configuration

```yaml
rules:
  long_params:
    severity: low
    max_params: 5
    ignore_constructors: true
```

## ESLint Rule

This detector is available as an ESLint rule for real-time feedback in your editor.

```javascript
// eslint.config.js
export default [
  {
    rules: {
      '@archlinter/no-long-params': 'warn',
    },
  },
];
```

See [ESLint Integration](/integrations/eslint) for setup instructions.

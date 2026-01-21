---
title: Deep Nesting
description: "Identify code blocks nested too deeply, making code exponentially harder to read and indicating functions doing too much."
---

# Deep Nesting

**ID:** `deep_nesting` | **Severity:** Low (default)

Identifies code blocks (if, for, while, etc.) that are nested too deeply.

## Why this is a smell

Deeply nested code is exponentially harder to read and understand. It's often a sign that a function is doing too much or that the logic can be simplified.

## How to fix

- **Guard Clauses**: Return early to avoid `else` blocks and reduce nesting.
- **Extract Function**: Move the inner nested block to a new function.
- **Flatten Logic**: Re-evaluate the logic to see if it can be expressed more simply.

## Configuration

```yaml
rules:
  deep_nesting:
    severity: low
    max_depth: 4
```

## ESLint Rule

This detector is available as an ESLint rule for real-time feedback in your editor.

```javascript
// eslint.config.js
export default [
  {
    rules: {
      '@archlinter/no-deep-nesting': 'warn',
    },
  },
];
```

See [ESLint Integration](/integrations/eslint) for setup instructions.

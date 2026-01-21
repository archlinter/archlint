---
title: Layer Violation
description: "Detect when code in one architectural layer incorrectly imports code from another layer, breaking abstractions and the Single Responsibility Principle."
---

# Layer Violation

**ID:** `layer_violation` | **Severity:** High (default)

Layer violation occurs when code in one architectural layer imports code from a layer it shouldn't know about (e.g., Domain layer importing from Infrastructure).

## Why this is a smell

- **Breaks Abstraction**: Internal implementation details leak into high-level business logic.
- **Testing Difficulty**: Business logic becomes hard to test without mocks for infrastructure (DB, API, etc.).
- **Rigidity**: Changing a database or external library requires changing the core business logic.

## Configuration

You must define your layers in `.archlint.yaml`:

```yaml
rules:
  layer_violation:
    layers:
      - name: domain
        path: ['**/domain/**']
        allowed_imports: [] # Domain imports nothing

      - name: application
        path: ['**/application/**']
        allowed_imports: ['domain']

      - name: infrastructure
        path: ['**/infrastructure/**']
        allowed_imports: ['domain', 'application']
```

## ESLint Rule

This detector is available as an ESLint rule for real-time feedback in your editor.

```javascript
// eslint.config.js
export default [
  {
    rules: {
      '@archlinter/no-layer-violations': 'error',
    },
  },
];
```

See [ESLint Integration](/integrations/eslint) for setup instructions.

## How to fix

1. **Dependency Inversion**: Define an interface in the higher layer (Domain) and implement it in the lower layer (Infrastructure).
2. **Refactor**: Move the misplaced code to the appropriate layer.

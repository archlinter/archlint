# Dead Code

**ID:** `dead_code` | **Severity:** Low (default)

Dead code refers to exported functions, classes, or variables that are not imported or used anywhere else in the project.

## Why this is a smell

- **Maintenance Burden**: Developers might spend time updating or refactoring code that isn't even used.
- **Bundle Size**: Increases the final application size (though many bundlers do tree-shaking).
- **Confusion**: Makes the API of a module appear larger and more complex than it actually is.

## Examples

### Bad

```typescript
// utils.ts
export const usedHelper = () => { ... };
export const unusedHelper = () => { ... }; // Reported as dead code

// main.ts
import { usedHelper } from './utils';
```

## How to fix

1. **Delete it**: If it's truly unused, the best action is removal.
2. **Mark as Entry Point**: If it's part of a public API or a dynamic import, add it to `entry_points` in your config.

::: tip
**False Positives**: Architectural analysis can sometimes produce false positives, especially in projects with heavy dynamic loading, reflection, or complex Dependency Injection containers.
:::

## Configuration

```yaml
entry_points:
  - 'src/index.ts'
  - 'src/api/**/*.ts'
```

## ESLint Rule

This detector is available as an ESLint rule for real-time feedback in your editor.

```javascript
// eslint.config.js
export default [
  {
    rules: {
      '@archlinter/no-dead-code': 'warn',
    },
  },
];
```

See [ESLint Integration](/integrations/eslint) for setup instructions.

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

## Configuration

```yaml
entry_points:
  - 'src/index.ts'
  - 'src/api/**/*.ts'
```

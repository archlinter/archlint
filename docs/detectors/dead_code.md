# Dead Code

**ID:** `dead_code` | **Severity:** Low (default)

Dead code is what it sounds like: functions, classes, or variables that are "alive" in your codebase but don't actually do anything because nobody is using them.

## Why this is a smell

- **Wasted mental energy**: Developers shouldn't have to refactor or understand code that isn't even running.
- **False complexity**: It makes your API look bigger and scarier than it really is.
- **Ghost in the machine**: It can lead to "I thought we removed this" moments during debugging.

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
# Rule-specific options
rules:
  dead_code:
    exclude:
      - '**/tests/**'
      - '**/temp/**'

# Global options (root level)
entry_points:
  - 'src/index.ts'
  - 'src/api/**/*.ts'
```

### Options

#### Rule Options (`rules.dead_code`)

- `exclude`: A list of glob patterns to ignore when detecting dead code. Files matching these patterns will be treated as if they don't exist for the purpose of incoming dependency analysis.

#### Global Options (root level)

- `entry_points`: Global entry points that should never be reported as dead code.

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

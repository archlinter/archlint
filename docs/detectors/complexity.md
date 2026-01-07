# High Complexity

**ID:** `complexity` | **Severity:** Medium (default)

This detector identifies functions with high Cyclomatic Complexity.

## Why this is a smell

- **Hard to Understand**: Too many branching paths make the code hard to follow.
- **Bug Prone**: Higher chance of missing edge cases during testing.
- **Maintenance Nightmare**: Small changes can have unpredictable effects due to complex logic.

## How to fix

1. **Extract Method**: Break complex logic into smaller, named functions.
2. **Guard Clauses**: Use early returns to reduce nesting levels.
3. **Replace Conditional with Polymorphism**: Use objects or strategies instead of large `switch` or `if/else` blocks.

## Configuration

```yaml
rules:
  complexity:
    severity: warn
    max_complexity: 15
```

## ESLint Rule

This detector is available as an ESLint rule for real-time feedback in your editor.

```javascript
// eslint.config.js
export default [
  {
    rules: {
      '@archlinter/no-high-complexity': 'warn',
    },
  },
];
```

See [ESLint Integration](/integrations/eslint) for setup instructions.

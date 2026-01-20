# Cyclomatic Complexity

**ID:** `cyclomatic_complexity` | **Severity:** Medium (default)

Cyclomatic complexity is a measure of how many different paths a code execution can take. Think of it as the "if-else" spaghetti factor.

## Why this is a smell

- **Mental maze**: Every `if`, `else`, and `switch` case adds another branch to the maze. If a function has 20 paths, you can bet a developer will get lost in it eventually.
- **Testing nightmare**: To truly test a complex function, you'd need a test case for every possible path. In the real world, that usually means some branches never get tested at all.
- **The "Butterfly Effect"**: In highly complex functions, changing one line of code can have weird, unpredictable consequences five branches away.

## How to fix

1. **Extract Method**: Break complex logic into smaller, named functions.
2. **Guard Clauses**: Use early returns to reduce nesting levels.
3. **Replace Conditional with Polymorphism**: Use objects or strategies instead of large `switch` or `if/else` blocks.

## Configuration

```yaml
rules:
  cyclomatic_complexity:
    severity: medium
    max_complexity: 15
```

## ESLint Rule

This detector is available as an ESLint rule for real-time feedback in your editor.

```javascript
// eslint.config.js
export default [
  {
    rules: {
      '@archlinter/no-high-cyclomatic-complexity': 'warn',
    },
  },
];
```

See [ESLint Integration](/integrations/eslint) for setup instructions.

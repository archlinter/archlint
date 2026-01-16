# Cognitive Complexity

**ID:** `cognitive_complexity` | **Severity:** Medium (default)

This detector identifies functions with high Cognitive Complexity. Cognitive Complexity measures how difficult code is to understand, rather than just how many paths it has.

## Why this is a smell

- **High Mental Load**: Deeply nested logic and complex boolean expressions make it hard for developers to keep the state in their head.
- **Maintenance Risk**: Code that is hard to understand is prone to bugs during modification.
- **Hidden Bugs**: Logic errors often hide in deeply nested structures.

## How it's calculated

Cognitive Complexity is calculated based on:

1.  **Structural Increments**: `if`, `else`, `switch`, `for`, `while`, `do-while`, `catch`, ternary operators, and logical sequences.
2.  **Nesting Penalty**: Increments for control structures are increased based on their nesting level.
3.  **Special Cases**: `switch` counts only once for the whole block, regardless of the number of cases.

## How to fix

1.  **Flatten Logic**: Use guard clauses (early returns) to reduce nesting.
2.  **Extract Method**: Move nested blocks or complex conditions into small, focused functions.
3.  **Simplify Expressions**: Break down complex boolean conditions into intermediate variables or functions.
4.  **Replace Nested Ifs**: Consider using a lookup table or the Strategy pattern.

## Configuration

```yaml
rules:
  cognitive_complexity:
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
      '@archlinter/no-high-cognitive-complexity': 'warn',
    },
  },
];
```

See [ESLint Integration](/integrations/eslint) for setup instructions.

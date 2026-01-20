# Cognitive Complexity

**ID:** `cognitive_complexity` | **Severity:** Medium (default)

Cognitive complexity isn't just about how many branches your code has; it’s about how much effort it takes for a human brain to actually understand it. It’s the difference between "technically correct" and "readable".

## Why this is a smell

- **Mental stack overflow**: Humans aren't good at keeping track of five levels of nested logic and complex boolean algebra at the same time. When the mental load gets too high, we start making mistakes.
- **Invisible bugs**: Bugs love to hide in the shadows of deeply nested `if` statements and ternary operators.
- **Review friction**: If it takes a senior dev 20 minutes to understand a 30-line function during a PR review, it’s too complex.

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

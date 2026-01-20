# Deep Nesting

**ID:** `deep_nesting` | **Severity:** Low (default)

Identifies code blocks (if, for, while, etc.) that are nested so deep they start looking like a pyramid.

## Why this is a smell

Reading deeply nested code is like reading a sentence with too many (parentheticals (inside (other parentheticals))). It's mentally exhausting and usually a sign that your function is trying to handle too many edge cases at once. It’s better to fail fast or extract the logic.

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

# Long Parameter List

**ID:** `long_params` | **Severity:** Low (default)

Identifies functions that require excessive information at once.

## Why this is a smell

Functions with 10 parameters are confusing to call and even more confusing to read. Was the third argument the `userId` or the `orderId`? When you have a long list of arguments, it's a sign that the function is either doing too much or that those parameters belong together in a single object.

## How to fix

- **Introduce Parameter Object**: Group related parameters into a single object or interface.
- **Decompose Function**: Split the function into smaller ones that require fewer parameters.

## Configuration

```yaml
rules:
  long_params:
    severity: low
    max_params: 5
    ignore_constructors: true
```

## ESLint Rule

This detector is available as an ESLint rule for real-time feedback in your editor.

```javascript
// eslint.config.js
export default [
  {
    rules: {
      '@archlinter/no-long-params': 'warn',
    },
  },
];
```

See [ESLint Integration](/integrations/eslint) for setup instructions.

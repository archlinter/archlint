# Code Clone

**ID:** `code_clone` | **Severity:** Medium (default)

This detector finds where someone took the "copy-paste" shortcut. It looks for identical logic that’s been duplicated across your project.

## Why this is a smell

- **Bugs multiply**: If you find a bug in one copy, you have to remember to fix it in the other four. Spoilers: you usually forget one.
- **Maintenance overhead**: Every time you want to change how a specific logic works, you're doing the same work over and over.
- **Inconsistent evolution**: Eventually, one copy gets updated while another doesn't, and suddenly your "identical" logic behaves differently in different parts of the app.

## How to fix

1. **Extract Method**: Move the shared logic into a single function and call it from multiple places.
2. **Generic Components**: For UI code, create a reusable component with props.
3. **Utility Modules**: Move common helper logic to a shared utility file.

## Configuration

```yaml
rules:
  code_clone:
    enabled: true
    severity: medium
    min_tokens: 50
    min_lines: 6
```

### Options

- `min_tokens`: The minimum number of normalized tokens to trigger a clone detection (default: 50).
- `min_lines`: The minimum number of lines the clone must span (default: 6).

## ESLint Rule

This detector is available as an ESLint rule for real-time feedback in your editor.

```javascript
// eslint.config.js
export default [
  {
    rules: {
      '@archlinter/no-code-clone': 'warn',
    },
  },
];
```

See [ESLint Integration](/integrations/eslint) for setup instructions.

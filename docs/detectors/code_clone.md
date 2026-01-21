---
title: Code Clone
description: "Identify duplicated code blocks across your project using AST-based tokenization to find exact matches regardless of formatting."
---

# Code Clone

**ID:** `code_clone` | **Severity:** Medium (default)

This detector identifies duplicated code blocks across your project. It uses AST-based tokenization to find exact matches while ignoring differences in formatting and comments.

## Why this is a smell

- **Maintenance Overhead**: Fixing a bug or making a change in one place requires updating all duplicates.
- **Violation of DRY**: Duplication is a clear sign that abstraction or reuse is missing.
- **Inconsistent Evolution**: Over time, duplicates may drift apart, leading to subtle bugs and harder refactoring.

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

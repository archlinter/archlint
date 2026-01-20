# Barrel Abuse

**ID:** `barrel_file` | **Severity:** Medium (default)

Barrel files (like an `index.ts` that just re-exports everything) are meant to simplify imports, but they often turn into an architectural black hole.

## Why this is a smell

- **Circular dependency factory**: Large barrels are the #1 cause of those annoying indirect circular dependencies that are impossible to trace.
- **Importing the whole world**: When you import one tiny constant from a massive barrel, the bundler often ends up pulling in every single module that barrel references.
- **Slows you down**: They make IDE indexing crawl and can bloat your production bundle if tree-shaking isn't perfect.

## Configuration

```yaml
rules:
  barrel_file:
    severity: high
    max_reexports: 10
```

## ESLint Rule

This detector is available as an ESLint rule for real-time feedback in your editor.

```javascript
// eslint.config.js
export default [
  {
    rules: {
      '@archlinter/no-barrel-abuse': 'warn',
    },
  },
];
```

See [ESLint Integration](/integrations/eslint) for setup instructions.

## How to fix

- Avoid "catch-all" barrel files at the root of large directories.
- Prefer direct imports if a barrel file is causing issues.
- Group exports into smaller, more specific barrel files.

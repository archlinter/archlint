# God Module

**ID:** `god_module` | **Severity:** High (default)

A "God Module" is that one file in your project everyone is afraid to touch because it does everything. It usually starts as a simple utility and somehow grows into a monster that handles auth, database queries, and UI state all at once.

## Why this is a smell

- **It's a Single Responsibility nightmare**: When one module does everything, any change—no matter how small—feels like you're playing Jenga with your architecture.
- **Merge conflict magnet**: Since it's the center of the universe, every developer on the team is constantly fighting over it in git.
- **Fragility**: Changes in one part of the module might unexpectedly break another part because everything is implicitly connected.
- **Testing headache**: You shouldn't have to mock a database and an email service just to test a simple string formatter.

## Detection Criteria

archlint identifies God Modules based on:

- **Fan-in**: Number of other modules depending on it.
- **Fan-out**: Number of modules it depends on.
- **Churn**: Frequency of changes in git.
- **Lines of Code**: Total size of the file.

## How to fix

1. **Identify Responsibilities**: List all different tasks the module performs.
2. **Extract Modules**: Break the file into smaller, focused modules.
3. **Facade Pattern**: If the module acts as a coordinator, keep only the coordination logic and delegate the work to sub-modules.

## Configuration

```yaml
rules:
  god_module:
    severity: high
    fan_in: 15
    fan_out: 15
    churn: 20
```

## ESLint Rule

This detector is available as an ESLint rule for real-time feedback in your editor.

```javascript
// eslint.config.js
export default [
  {
    rules: {
      '@archlinter/no-god-modules': 'warn',
    },
  },
];
```

See [ESLint Integration](/integrations/eslint) for setup instructions.

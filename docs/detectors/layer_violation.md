# Layer Violation

**ID:** `layer_violation` | **Severity:** High (default)

Layer violation happens when your clean architecture starts leaking. It’s when your high-level business logic (Domain) starts asking about your database tables or API endpoints (Infrastructure).

## Why this is a smell

- **Leaky abstractions**: Your business logic shouldn't care if you're using Postgres or a JSON file. When layers leak, you lose that freedom.
- **Brittle tests**: You shouldn't need to spin up a mock database just to test a simple business rule.
- **Change friction**: Want to swap your logging library? Too bad, you've imported it directly into your core domain, and now you have to refactor everything.

## Configuration

You must define your layers in `.archlint.yaml`:

```yaml
rules:
  layer_violation:
    layers:
      - name: domain
        path: ['**/domain/**']
        allowed_imports: [] # Domain imports nothing

      - name: application
        path: ['**/application/**']
        allowed_imports: ['domain']

      - name: infrastructure
        path: ['**/infrastructure/**']
        allowed_imports: ['domain', 'application']
```

## ESLint Rule

This detector is available as an ESLint rule for real-time feedback in your editor.

```javascript
// eslint.config.js
export default [
  {
    rules: {
      '@archlinter/no-layer-violations': 'error',
    },
  },
];
```

See [ESLint Integration](/integrations/eslint) for setup instructions.

## How to fix

1. **Dependency Inversion**: Define an interface in the higher layer (Domain) and implement it in the lower layer (Infrastructure).
2. **Refactor**: Move the misplaced code to the appropriate layer.

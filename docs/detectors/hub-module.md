# Hub Module

**ID:** `hub_module` | **Severity:** Medium (default)

A "Hub Module" is a central point in the dependency graph, characterized by both high Fan-in (many dependants) and high Fan-out (many dependencies), while containing relatively little internal logic.

## Why this is a smell

Hub modules represent dangerous "single points of failure" in your architecture. Because they sit at the center of many paths, they become extremely fragile. A minor change in a hub module can trigger a massive ripple effect across the entire codebase, making them difficult and risky to refactor.

## Examples

### Bad

A module that merely re-exports or coordinates many unrelated services and is itself used by the entire application.

```typescript
// app-hub.ts
import { AuthService } from './auth';
import { ApiService } from './api';
import { LoggerService } from './logger';
import { ConfigService } from './config';
// ... 10 more imports

export class AppHub {
  constructor(
    public auth: AuthService,
    public api: ApiService,
    public logger: LoggerService
    // ... 10 more dependencies
  ) {}
}
```

### Good

Break down the hub into specific, focused coordinators or use dependency injection at the consumer level to avoid a central hub.

```typescript
// auth-coordinator.ts (Focused on auth-related coordination)
import { AuthService } from './auth';
import { SessionStore } from './session';

export class AuthCoordinator {
  constructor(
    private auth: AuthService,
    private session: SessionStore
  ) {}
}
```

## Configuration

```yaml
rules:
  hub_module:
    severity: warn
    min_fan_in: 5
    min_fan_out: 5
    max_complexity: 5
```

## ESLint Rule

This detector is available as an ESLint rule for real-time feedback in your editor.

```javascript
// eslint.config.js
export default [
  {
    rules: {
      '@archlinter/no-hub-modules': 'warn',
    },
  },
];
```

See [ESLint Integration](/integrations/eslint) for setup instructions.

## How to fix

Break the hub! Identify the different paths of data or control passing through the hub and extract them into separate, more focused modules.

# Hub Module

**ID:** `hub_module` | **Severity:** Medium (default)

A "Hub Module" is like a busy traffic intersection with no lights. It’s a module that everyone depends on, and it also depends on everyone else, but it doesn't actually _do_ much logic itself.

## Why this is a smell

Hub modules are dangerous single points of failure. Because they sit at the center of everything, they are incredibly fragile. A tiny change to a hub can break unrelated parts of your app, making it the most terrifying file to refactor in your entire codebase. It’s the ultimate "bottleneck" for your architecture.

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
    severity: medium
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

# Stable Dependencies Principle (SDP)

**ID:** `sdp_violation` | **Severity:** Medium (default)

The Stable Dependencies Principle states that "the dependencies between packages should be in the direction of the stability." In other words, stable (hard to change) modules should not depend on unstable (easy to change) modules.

Stability in this context is measured by how many other modules depend on a module (Fan-in) versus how many modules it depends on (Fan-out).

## Why this is a smell

When a stable module—one that many other components rely on—depends on an unstable module, it becomes difficult to change. Any modification in the unstable dependency can break the stable module, which then ripples through all its dependents. This effectively "freezes" the unstable module or makes the entire system fragile.

## Examples

### Bad

A core domain entity (stable) depending on a specific database implementation or a frequently changing UI component (unstable).

```typescript
// domain/user.ts (Stable, many things depend on User)
import { UserPreferencesUI } from '../ui/user-prefs'; // Unstable dependency

export class User {
  updateSettings(prefs: UserPreferencesUI) {
    // ...
  }
}
```

### Good

The stable module depends on an abstraction (like an interface) that changes rarely.

```typescript
// domain/user.ts
export interface UserSettings {
  theme: string;
  notifications: boolean;
}

export class User {
  updateSettings(settings: UserSettings) {
    // ...
  }
}
```

## Configuration

```yaml
rules:
  sdp_violation:
    severity: medium
    min_fan_total: 5
    instability_diff: 0.3
```

## ESLint Rule

This detector is available as an ESLint rule for real-time feedback in your editor.

```javascript
// eslint.config.js
export default [
  {
    rules: {
      '@archlinter/no-sdp-violations': 'warn',
    },
  },
];
```

See [ESLint Integration](/integrations/eslint) for setup instructions.

## How to fix

Ensure that your core, stable modules don't depend on volatile modules. Use interfaces or abstract classes to decouple them.

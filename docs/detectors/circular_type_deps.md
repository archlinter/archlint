# Type Cycles

**ID:** `circular_type_deps` | **Severity:** Medium (default)

Similar to circular dependencies, but specifically for type-only imports (e.g., `import type { ... }`).

## Why this is a smell

While type-only cycles don't cause runtime issues in TypeScript, they still indicate tight architectural coupling. They make it harder to separate modules and can still lead to complex dependency graphs that are hard to reason about.

## Example

### Bad

```typescript
// userService.ts
import type { UserProfile } from './profileService';

export interface User {
  id: string;
  profile: UserProfile;
}

// profileService.ts
import type { User } from './userService';

export interface UserProfile {
  id: string;
  owner: User;
}
```

## How to fix

Move the shared types to a common `types.ts` or a separate module that doesn't depend on the implementation.

### Good

```typescript
// types.ts
export interface User {
  id: string;
  profile: UserProfile;
}

export interface UserProfile {
  id: string;
  owner: User;
}

// userService.ts
import type { User, UserProfile } from './types';
// ... use User here
```

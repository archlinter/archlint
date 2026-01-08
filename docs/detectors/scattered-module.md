# Scattered Module

**ID:** `module_cohesion` | **Severity:** Medium (default)

Identifies a "module" (typically a file or logical group) where the internal elements (functions, classes) are not well-connected. This indicates that the module lacks a cohesive purpose and is likely a collection of unrelated code.

## Why this is a smell

A module should be cohesive, following the principle that "things that change together should stay together." If a module's internal parts don't interact with each other, it's not a true module—it's just a folder or file used as a random storage bin. This makes the code harder to find and increases cognitive load.

## Examples

### Bad

A file containing unrelated helper functions that share no common logic or data.

```typescript
// misc-utils.ts
export const formatCurrency = (val: number) => { ... };
export const validateEmail = (email: string) => { ... };
export const parseJwt = (token: string) => { ... };
// These three functions share no common state or logic.
```

### Good

Group unrelated functions into specific, cohesive modules.

```typescript
// currency-utils.ts
export const formatCurrency = (val: number) => { ... };

// validation-utils.ts
export const validateEmail = (email: string) => { ... };
```

## Configuration

```yaml
rules:
  module_cohesion:
    severity: warn
    min_exports: 5
    max_components: 2
```

## How to fix

Re-evaluate the purpose of the module. Group the code into more cohesive modules or move the unrelated parts to where they are actually used.

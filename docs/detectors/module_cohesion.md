# Scattered Module

**ID:** `module_cohesion` | **Severity:** Medium (default)

Identifies a "junk drawer" file—a module where the internal parts (functions, classes) have absolutely nothing to do with each other.

## Why this is a smell

A good module should follow the rule: "things that change together, stay together." If your file is just a collection of random helpers that never interact, it's not a module—it's a storage bin. This makes your code harder to find and increases the mental effort needed to understand why these things are even in the same file.

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
    severity: medium
    min_exports: 5
    max_components: 2
```

## How to fix

Re-evaluate the purpose of the module. Group the code into more cohesive modules or move the unrelated parts to where they are actually used.

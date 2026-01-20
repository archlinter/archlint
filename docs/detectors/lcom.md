# Low Cohesion (LCOM4)

**ID:** `lcom` | **Severity:** Medium (default)

Cohesion measures whether the methods and fields in your class actually belong together. If they don't, you probably have a "Frankenstein's monster" class.

## Why this is a smell

- **SRP Violation**: Your class is likely wearing too many hats and trying to do three different jobs at once.
- **Fragility**: You change a method related to "user avatars" and somehow break the "password hashing" logic because they’re sharing the same bloated class.
- **Hard to Reuse**: If you just need the "avatar" logic, you're forced to bring along the whole "password" machinery too.

## How to fix

1. **Extract Class**: Split the class into two or more smaller classes, each with a single responsibility.
2. **Move Method**: Move methods that don't use the class state to a more appropriate location (e.g., a utility module).

## Configuration

```yaml
rules:
  lcom:
    severity: medium
    max_lcom: 4
    min_methods: 3
```

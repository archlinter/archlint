# Low Cohesion (LCOM4)

**ID:** `lcom` | **Severity:** Medium (default)

Cohesion measures how closely related the methods and fields of a class are. archlint uses the **LCOM4** (Lack of Cohesion of Methods) metric.

## Why this is a smell

- **Violation of SRP**: The class is likely doing two or more unrelated things.
- **Fragility**: Changing one part of the class might affect unrelated parts.
- **Hard to Reuse**: You can't use one part of the class without pulling in unrelated logic.

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

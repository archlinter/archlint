# Abstractness Violation

**ID:** `abstractness` | **Severity:** Low (default)

This detector uses Robert C. Martin's "Main Sequence" metrics to evaluate the relationship between a module's Stability (I) and its Abstractness (A). The goal is to ensure that modules sit near the "Main Sequence"—a line where abstractness increases as stability increases.

## Why this is a smell

- **Zone of Pain**: Modules that are highly stable (many things depend on them) but very concrete (no abstractions). These are extremely hard to change because of their dependencies, yet their concrete nature means they _will_ need to change.
- **Zone of Uselessness**: Modules that are highly abstract (many interfaces/abstract classes) but very unstable (no one depends on them). These provide abstractions that aren't actually being used, adding unnecessary complexity.

## How to fix

- **In the Zone of Pain**: Introduce abstractions (interfaces, abstract classes) to decouple the module's implementation from its users.
- **In the Zone of Uselessness**: Consider making the module more concrete or removing unused abstractions to simplify the code.

## Configuration

```yaml
rules:
  abstractness:
    severity: warn
```

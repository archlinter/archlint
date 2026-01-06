# High Coupling

**ID:** `high_coupling` | **Severity:** Medium (default)

High coupling occurs when a module depends on too many other modules (high Fan-out).

## Why this is a smell

- **Rigidity**: A change in any of the dependencies might require a change in this module.
- **Fragility**: The module is more likely to break when any of its dependencies change.
- **Hard to Test**: Requires many mocks to isolate for unit testing.

## How to fix

1. **Extract Responsibilities**: If a module has too many dependencies, it's likely doing too much.
2. **Use Abstractions**: Depend on an interface or a facade instead of many concrete implementations.

## Configuration

```yaml
thresholds:
  high_coupling:
    max_dependencies: 15
```

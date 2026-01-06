# Long Parameter List

**ID:** `long_params` | **Severity:** Low (default)

Identifies functions or methods that have too many parameters.

## Why this is a smell

Functions with many parameters are hard to use and hard to read. They often indicate that the function is doing too much or that some parameters should be grouped into an object.

## How to fix

- **Introduce Parameter Object**: Group related parameters into a single object or interface.
- **Decompose Function**: Split the function into smaller ones that require fewer parameters.

## Configuration

```yaml
thresholds:
  long_params:
    max_params: 5
```

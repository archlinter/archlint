# Deep Nesting

**ID:** `deep_nesting` | **Severity:** Low (default)

Identifies code blocks (if, for, while, etc.) that are nested too deeply.

## Why this is a smell

Deeply nested code is exponentially harder to read and understand. It's often a sign that a function is doing too much or that the logic can be simplified.

## How to fix

- **Guard Clauses**: Return early to avoid `else` blocks and reduce nesting.
- **Extract Function**: Move the inner nested block to a new function.
- **Flatten Logic**: Re-evaluate the logic to see if it can be expressed more simply.

## Configuration

```yaml
thresholds:
  deep_nesting:
    max_depth: 4
```

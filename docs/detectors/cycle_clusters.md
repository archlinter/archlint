# Cyclic Dependency Cluster

**ID:** `cycle_clusters` | **Severity:** Critical (default)

A cyclic dependency cluster is what happens when circular dependencies start breeding. It’s not just a simple "A depends on B, B depends on A" loop—it's a complex web where a dozen modules are all tangled up together.

## Why this is a smell

- **Architectural Rot**: It’s a sign that your module boundaries have completely collapsed.
- **The "Monolith" effect**: You can't just borrow one module from the cluster; you have to pull in the whole tangled mess. It’s a package deal you didn't ask for.
- **Impossible Isolation**: Want to test a single function? Too bad, you're now mocking half your codebase because everything is interconnected.
- **Maintenance Nightmare**: Changing one module in the cluster can trigger an unpredictable ripple effect that breaks something on the other side of the web.

## Examples

### Bad

A group of modules in a "core" directory where almost every module imports several others from the same directory, creating multiple overlapping cycles.

### Good

Modules should be organized in a hierarchy or with clear interface-based decoupling to ensure that cycles do not form clusters.

## Configuration

```yaml
rules:
  cycle_clusters:
    severity: high
    max_cluster_size: 5
```

## How to fix

1. **Break the hub**: Identify "hub" modules that participate in multiple cycles and decouple them first.
2. **Layering**: Enforce strict layering rules to prevent horizontal or upward dependencies.
3. **Refactor Monoliths**: Often clusters are a sign that a single large module was split incorrectly. Consider merging or re-splitting along different boundaries.

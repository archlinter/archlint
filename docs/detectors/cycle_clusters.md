---
title: Cyclic Dependency Cluster
description: "Identify interconnected sets of circular dependencies forming complex webs that indicate architectural rot and extreme coupling."
---

# Cyclic Dependency Cluster

**ID:** `cycle_clusters` | **Severity:** Critical (default)

A cyclic dependency cluster is a set of circular dependencies that are interconnected, forming a complex web of dependencies. Unlike simple cycles (A -> B -> A), clusters involve multiple cycles that overlap (e.g., A -> B -> C -> A and B -> D -> C -> B).

## Why this is a smell

- **Architectural Rot**: Clusters often indicate a lack of clear boundaries between multiple components.
- **Extreme Coupling**: The entire cluster must be treated as a single monolithic unit.
- **Impossible Isolation**: It is nearly impossible to change or test one module in the cluster without affecting all others.
- **Maintenance Nightmare**: Changes in any part of the cluster can have unpredictable effects across all modules involved.

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

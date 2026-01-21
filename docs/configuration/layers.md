---
title: Layers
description: "Define architectural levels in your project and enforce strict dependency rules to maintain a clean architecture and prevent coupling."
---

# Layers

Layer configuration allows you to define architectural levels of your project and enforce dependency rules between them.

## Defining Layers

Layers are configured inside the `layer_violation` rule. Each layer definition consists of:

- `name`: Unique name of the layer.
- `path` (or `paths`): Glob pattern identifying files in this layer.
- `allowed_imports` (or `can_import`): List of layer names that this layer is allowed to import.

## Example: Clean Architecture

```yaml
rules:
  layer_violation:
    severity: high
    layers:
      - name: domain
        path: '**/domain/**'
        allowed_imports: [] # Domain layer should not depend on anything

      - name: application
        path: '**/application/**'
        allowed_imports:
          - domain

      - name: infrastructure
        path: '**/infrastructure/**'
        allowed_imports:
          - domain
          - application

      - name: presentation
        path: '**/presentation/**'
        allowed_imports:
          - domain
          - application
```

## How It Works

When the `layer_violation` detector is enabled:

1. It maps every file in your project to a specific layer based on the `path` pattern.
2. If a file matches multiple patterns, the most specific one (longest pattern) is chosen.
3. The tool checks every import. If a file in layer `A` imports a file in layer `B`, but `B` is not in the `allowed_imports` list of layer `A`, a violation is reported.

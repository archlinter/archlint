# Layers

The `layers` configuration allows you to define your project's architectural layers and enforce the dependency rules between them.

## Defining Layers

Each layer definition consists of:

- `name`: A unique identifier for the layer.
- `paths`: An array of glob patterns that identify the files in this layer.
- `can_import`: An array of layer names that this layer is allowed to depend on.

## Example: Clean Architecture

```yaml
layers:
  - name: domain
    paths: ['**/domain/**']
    can_import: [] # Domain layer must be independent

  - name: application
    paths: ['**/application/**', '**/use-cases/**']
    can_import:
      - domain

  - name: infrastructure
    paths: ['**/infrastructure/**', '**/adapters/**']
    can_import:
      - domain
      - application

  - name: presentation
    paths: ['**/controllers/**', '**/api/**', '**/ui/**']
    can_import:
      - domain
      - application
```

## How it works

When the `layer_violation` detector is enabled:

1. It assigns each file in your project to a layer based on the `paths` patterns.
2. It checks every import in those files.
3. If a file in layer `A` imports a file in layer `B`, but `B` is not in `A`'s `can_import` list, a violation is reported.

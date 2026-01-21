---
title: NestJS Support
description: "Specialized analysis for NestJS modular architecture, recognizing @Module decorators, Controllers, Providers, and layer enforcement."
---

# NestJS Support

archlint understands the modular architecture of NestJS and provides specialized analysis for it.

## Key Features

- **Module Analysis**: Recognizes `@Module` as a coordination point and relaxes coupling rules for it.
- **Entry Points**: Automatically marks Controllers and Providers as entry points.
- **Layer Enforcement**: Works perfectly with NestJS-style layer architectures (Controllers -> Services -> Repositories).
- **LCOM Overrides**: Ignores NestJS decorators in cohesion analysis to focus on the actual logic.

## Recommended Configuration

```yaml
extends:
  - nestjs

rules:
  layer_violation:
    layers:
  - name: presentation
    path: ['**/*.controller.ts']
    allowed_imports: ['application']

  - name: application
    path: ['**/*.service.ts']
    allowed_imports: ['domain']

  - name: domain
    path: ['**/entities/**']
    allowed_imports: []
```

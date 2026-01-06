# NestJS Support

archlint understands the modular architecture of NestJS and provides specialized analysis for it.

## Key Features

- **Module Analysis**: Recognizes `@Module` as a coordination point and relaxes coupling rules for it.
- **Entry Points**: Automatically marks Controllers and Providers as entry points.
- **Layer Enforcement**: Works perfectly with NestJS-style layer architectures (Controllers -> Services -> Repositories).
- **LCOM Overrides**: Ignores NestJS decorators in cohesion analysis to focus on the actual logic.

## Recommended Configuration

```yaml
frameworks:
  - nestjs

layers:
  - name: presentation
    paths: ['**/*.controller.ts']
    can_import: ['application']

  - name: application
    paths: ['**/*.service.ts']
    can_import: ['domain']

  - name: domain
    paths: ['**/entities/**']
    can_import: []
```

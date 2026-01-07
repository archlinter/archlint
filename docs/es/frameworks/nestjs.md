# Soporte para NestJS

archlint comprende la arquitectura modular de NestJS y proporciona un análisis especializado para ella.

## Características Clave

- **Análisis de Módulos**: Reconoce `@Module` como un punto de coordinación y relaja las reglas de acoplamiento para él.
- **Puntos de Entrada**: Marca automáticamente los Controladores (Controllers) y Proveedores (Providers) como puntos de entrada.
- **Aplicación de Capas**: Funciona perfectamente con arquitecturas de capas al estilo NestJS (Controllers -> Services -> Repositories).
- **Sobrescritura de LCOM**: Ignora los decoradores de NestJS en el análisis de cohesión para centrarse en la lógica real.

## Configuración Recomendada

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

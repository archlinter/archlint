# Capas (Layers)

La configuración de `layers` te permite definir las capas arquitectónicas de tu proyecto y aplicar reglas de dependencia entre ellas.

## Definición de Capas

Cada definición de capa consta de:

- `name`: Un identificador único para la capa.
- `paths`: Una lista de patrones glob que identifican los archivos en esta capa.
- `can_import`: Una lista de nombres de capas de las que esta capa puede depender.

## Ejemplo: Arquitectura Limpia (Clean Architecture)

```yaml
layers:
  - name: domain
    paths: ['**/domain/**']
    can_import: [] # La capa domain debe ser independiente

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

## Cómo funciona

Cuando el detector `layer_violation` está habilitado:

1. Asigna cada archivo de tu proyecto a una capa basándose en los patrones de `paths`.
2. Comprueba cada importación en esos archivos.
3. Si un archivo en la capa `A` importa un archivo en la capa `B`, pero `B` no está en la lista `can_import` de `A`, se informa de una violación.

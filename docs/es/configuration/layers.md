---
title: Layers
description: "Define niveles arquitectónicos en tu proyecto y aplica reglas estrictas de dependencias para mantener una arquitectura limpia y prevenir el acoplamiento."
---

# Capas

La configuración de capas le permite definir niveles arquitectónicos en su proyecto y hacer cumplir las reglas de dependencia entre ellos.

## Definición de Capas

Las capas se configuran dentro de la regla `layer_violation`. Cada definición de capa consiste en:

- `name`: Nombre único de la capa.
- `path` (o `paths`): Patrón glob que identifica los archivos en esta capa.
- `allowed_imports` (o `can_import`): Lista de nombres de capas que esta capa tiene permitido importar.

## Ejemplo: Arquitectura Limpia (Clean Architecture)

```yaml
rules:
  layer_violation:
    severity: high
    layers:
      - name: domain
        path: '**/domain/**'
        allowed_imports: [] # La capa domain no debe depender de nada

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

## Cómo Funciona

Cuando el detector `layer_violation` está habilitado:

1. Mapea cada archivo de su proyecto a una capa específica basándose en el patrón `path`.
2. Si un archivo coincide con varios patrones, se elige el más específico (el patrón más largo).
3. La herramienta verifica cada importación. Si un archivo en la capa `A` importa un archivo en la capa `B`, pero `B` no está en la lista `allowed_imports` de la capa `A`, se informa de una violación.

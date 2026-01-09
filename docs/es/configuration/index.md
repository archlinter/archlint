---
title: Configuración
description: Aprenda a configurar archlint usando .archlint.yaml, definir capas arquitectónicas y configurar reglas para detectores.
---

# Configuración

archlint se puede configurar usando un archivo `.archlint.yaml` en la raíz de su proyecto. Si no se encuentra ningún archivo de configuración, la herramienta utiliza valores predeterminados razonables para todos los detectores.

## Estructura del Archivo de Configuración

```yaml
# Archivos y directorios a ignorar (global)
ignore:
  - '**/dist/**'
  - '**/node_modules/**'

# Alias de rutas (similar a tsconfig.json o webpack)
aliases:
  '@/*': 'src/*'

# Extender desde presets integrados o personalizados
extends:
  - nestjs
  - ./my-company-preset.yaml

# Puntos de entrada para el análisis (utilizados para detección de código muerto)
entry_points:
  - 'src/main.ts'

# Configuración de reglas para cada detector
rules:
  # Forma corta: nivel de severidad o "off"
  cycles: error
  dead_code: warn

  # Forma completa: con opciones adicionales
  god_module:
    severity: error
    enabled: true
    exclude: ['**/generated/**']
    # Opciones específicas del detector
    fan_in: 15
    fan_out: 15
    churn: 20

  vendor_coupling:
    severity: warn
    ignore_packages: ['lodash', 'rxjs']

# Sobrescritura de reglas para rutas específicas
overrides:
  - files: ['**/legacy/**']
    rules:
      complexity: warn
      god_module: off

# Configuración de puntuación y calificación
scoring:
  # Nivel mínimo de severidad para informar (info, warn, error, critical)
  minimum: warn
  # Pesos para el cálculo de la puntuación total
  weights:
    critical: 100
    high: 50
    medium: 20
    low: 5
  # Umbrales para la calificación (Densidad = Puntuación Total / Archivos)
  grade_rules:
    excellent: 1.0
    good: 3.0
    fair: 7.0
    moderate: 15.0
    poor: 30.0

# Autodetección de framework (por defecto true)
auto_detect_framework: true

# Habilitar análisis de historial de Git (por defecto true)
enable_git: true

# Configuración de Git
git:
  history_period: '1y'
```

## Extends (Extensión)

El campo `extends` le permite cargar presets desde diferentes fuentes:

- **Presets integrados**: `nestjs`, `nextjs`, `react`, `oclif`.
- **Archivos locales**: Ruta relativa a un archivo YAML (por ejemplo, `./archlint-shared.yaml`).
- **URLs**: URL directa a un archivo YAML (por ejemplo, `https://example.com/preset.yaml`).

Los presets se fusionan en el orden en que se listan. La configuración del usuario siempre tiene la prioridad más alta.

## Reglas y Niveles de Severidad

En la sección `rules`, puede usar los siguientes niveles:

- `critical`: Problema crítico que requiere atención inmediata.
- `error`: Error arquitectónico.
- `warn`: Advertencia sobre un problema potencial.
- `info`: Mensaje informativo.
- `off`: Desactiva completamente el detector.

## Configuración vía CLI

Puede especificar la ruta del archivo de configuración explícitamente:

```bash
archlint scan --config custom-config.yaml
```

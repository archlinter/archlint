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
# Por defecto, archlint carga automáticamente los alias desde tsconfig.json
aliases:
  '@/*': 'src/*'

# Configuración de integración con TypeScript (true, false o ruta al archivo)
tsconfig: true

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
  cycles: high
  dead_code: medium

  # Forma completa: con opciones adicionales
  god_module:
    severity: high
    enabled: true
    exclude: ['**/generated/**']
    # Opciones específicas del detector
    fan_in: 15
    fan_out: 15
    churn: 20

  vendor_coupling:
    severity: medium
    ignore_packages: ['lodash', 'rxjs']

# Sobrescritura de reglas para rutas específicas
overrides:
  - files: ['**/legacy/**']
    rules:
      complexity: medium
      god_module: off

# Configuración de puntuación y calificación
scoring:
  # Nivel mínimo de severidad para informar (low, medium, high, critical)
  minimum: low
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

# Configuración de Git
git:
  enabled: true # habilitar análisis (por defecto true)
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

- `critical`: Nivel más alto — problema crítico que requiere atención inmediata.
- `high`: Error arquitectónico de alta severidad.
- `medium`: Advertencia o problema de severidad media.
- `low`: Mensaje informativo o de baja severidad.
- `off`: Desactiva completamente el detector.

## Configuración vía CLI

Puede especificar la ruta del archivo de configuración explícitamente:

```bash
archlint scan --config custom-config.yaml
```

## Integración con TypeScript

archlint puede sincronizarse automáticamente con su `tsconfig.json`. Use el campo `tsconfig` para controlar esto:

- `tsconfig: true` (por defecto): Busca automáticamente `tsconfig.json` en la raíz del proyecto.
- `tsconfig: false` o `tsconfig: null`: Desactiva la integración con TypeScript.
- `tsconfig: "./ruta/al/tsconfig.json"`: Utiliza un archivo de configuración específico.

Cuando está habilitado, la herramienta:

1. **Carga Alias**: Extrae `compilerOptions.paths` y `compilerOptions.baseUrl` para configurar automáticamente `aliases`
2. **Auto-ignorar**: Agrega `compilerOptions.outDir` a la lista global de `ignore`.
3. **Exclusiones**: Incorpora patrones del campo `exclude` en la lista de `ignore`.

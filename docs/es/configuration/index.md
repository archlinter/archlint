---
title: Configuración
description: Aprenda a configurar archlint usando .archlint.yaml, definir capas arquitectónicas y configurar reglas para detectores.
---

# Configuración

archlint se puede configurar usando un archivo `.archlint.yaml` en la raíz de su proyecto. Si no se encuentra ningún archivo de configuración, la herramienta utiliza valores predeterminados razonables para todos los detectores.

## Estructura del archivo de configuración

```yaml
# Archivos y directorios a ignorar (global)
ignore:
  - '**/dist/**'
  - '**/node_modules/**'

# Alias de rutas (similar a tsconfig.json o webpack)
# Por defecto, archlint carga automáticamente los alias desde tsconfig.json.
# Los alias definidos explícitamente aquí tienen prioridad sobre los de tsconfig.json.
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
  cyclomatic_complexity: low
  cognitive_complexity: high

  # Forma completa: con opciones adicionales
  god_module:
    severity: high
    enabled: true
    exclude: ['**/generated/**']
    # Opciones específicas del detector
    fan_in: 15
    fan_out: 15
    churn: 20

  dead_symbols:
    severity: high
    # Coincidir con métodos de interfaz para evitar falsos positivos
    contract_methods:
      MyInterface: ['method1', 'method2']
      ValidatorConstraintInterface: ['validate', 'defaultMessage']

  vendor_coupling:
    severity: medium
    ignore_packages: ['lodash', 'rxjs']

# Sobrescritura de reglas para rutas específicas
overrides:
  - files: ['**/legacy/**']
    rules:
      cyclomatic_complexity: medium
      cognitive_complexity: high
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

# Configuración de diff arquitectónico
diff:
  # Umbral de porcentaje para que el empeoramiento de una métrica se considere regresión
  metric_threshold_percent: 20
  # Desplazamiento máximo de línea para considerar smells como iguales durante el diff difuso
  line_tolerance: 50

# Configuración de Git
git:
  enabled: true # habilitar análisis (por defecto true)
  history_period: '1y'
```

## Extensión (extends)

El campo `extends` le permite cargar presets desde diferentes fuentes:

- **Presets integrados**: `nestjs`, `nextjs`, `express`, `react`, `angular`, `vue`, `typeorm`, `prisma`, `oclif`, `class-validator`.
- **Archivos locales**: Ruta relativa a un archivo YAML (por ejemplo, `./archlint-shared.yaml`).
- **URLs**: URL directa a un archivo YAML (por ejemplo, `https://example.com/preset.yaml`).

Los presets se fusionan en el orden en que se listan. La configuración del usuario siempre tiene la prioridad más alta.

## Reglas y niveles de severidad

En la sección `rules`, puede usar los siguientes niveles:

- `critical`: Problema crítico que requiere atención inmediata.
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

1. **Carga Alias**: Extrae `compilerOptions.paths` y `compilerOptions.baseUrl` para configurar automáticamente `aliases`.
2. **Auto-ignorar**: Agrega `compilerOptions.outDir` a la lista global de `ignore`.
3. **Exclusiones**: Incorpora patrones del campo `exclude` en la lista de `ignore`.

## Configuración de diff

La sección `diff` controla cómo se detectan las regresiones arquitectónicas al comparar dos instantáneas:

- **`metric_threshold_percent`** (por defecto: `20`): Define cuánto debe aumentar una métrica (como la complejidad ciclomática/cognitiva o el acoplamiento) antes de que se informe como un smell "empeorado". Por ejemplo, con un umbral del 20%, la complejidad ciclomática o cognitiva de una función debe aumentar de 10 a al menos 12 para ser señalada.
- **`line_tolerance`** (por defecto: `50`): Define el número máximo de líneas que un símbolo de código puede desplazarse (debido a adiciones o eliminaciones en otras partes del archivo) antes de que archlint deje de reconocerlo como el mismo smell. Este "emparejamiento difuso" evita que el código desplazado se informe como una nueva regresión.

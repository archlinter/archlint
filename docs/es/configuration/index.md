---
title: Configuración
description: Aprende a configurar archlint usando archlint.yaml, definir capas arquitectónicas y establecer umbrales personalizados para los detectores.
---

# Configuración

archlint puede configurarse utilizando un archivo `archlint.yaml` en la raíz de tu proyecto. Si no se encuentra ningún archivo de configuración, la herramienta utiliza valores predeterminados razonables para todos los detectores.

## Estructura del Archivo de Configuración

```yaml
# Archivos a ignorar
ignore:
  - '**/dist/**'

# Alias de rutas (ej., desde tsconfig.json)
aliases:
  '@/*': 'src/*'

# Puntos de entrada para el análisis de código muerto (dead code)
entry_points:
  - 'src/index.ts'

# Umbrales personalizados para los detectores
thresholds:
  cycles:
    exclude_patterns: []
  god_module:
    fan_in: 15
    fan_out: 15

# Capas arquitectónicas
layers:
  - name: domain
    paths: ['**/domain/**']
    can_import: []

# Ajustes preestablecidos (presets) de frameworks
frameworks:
  - nestjs

# Sobrescritura de severidad
severity:
  cycles: critical
```

## Configuración vía CLI

También puedes especificar la ruta del archivo de configuración a través de la CLI:

```bash
archlint scan --config custom-config.yaml
```

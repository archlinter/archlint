# Soporte para oclif

archlint proporciona soporte integrado para [oclif](https://oclif.io/), el Open CLI Framework.

## Características

- **Puntos de entrada CLI**: Reconoce automáticamente los archivos de comando como puntos de entrada.
- **Detección de hooks**: Identifica hooks de oclif para evitar falsos positivos en el análisis de código muerto.
- **Reglas arquitectónicas**: Proporciona presets que siguen la estructura de directorios recomendada por oclif.

## Configuración

Para habilitar el soporte de oclif, agréguelo a su lista `extends`:

```yaml
extends:
  - oclif
```

## Lógica de detección

El preset de oclif se detecta automáticamente si:

1. `package.json` contiene `@oclif/core` en las dependencias.
2. El proyecto tiene un archivo `oclif.manifest.json`.

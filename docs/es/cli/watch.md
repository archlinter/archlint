---
title: watch
description: "Ejecuta archlint en modo watch para re-analizar automáticamente tu proyecto cada vez que cambien los archivos, proporcionando retroalimentación continua."
---

# archlint watch

El comando `watch` ejecuta archlint en segundo plano y vuelve a analizar tu proyecto cada vez que cambia un archivo.

## Uso

```bash
archlint watch [options]
```

## Opciones

| Opción            | Por defecto | Descripción                                               |
| ----------------- | ----------- | --------------------------------------------------------- |
| `--debounce <ms>` | `300`       | Tiempo de espera para más cambios antes de re-analizar    |
| `--clear`         | `false`     | Limpia la pantalla de la terminal antes de cada ejecución |

## Ejemplos

### Retroalimentación en tiempo real durante el desarrollo

```bash
archlint watch --clear --debounce 500
```

---
title: Referencia de CLI
description: Referencia completa de los comandos de la CLI de archlint, incluyendo scan, diff, snapshot y watch.
---

# Referencia de CLI

La CLI de archlint es la forma principal de interactuar con la herramienta.

## Uso General

```bash
archlint [command] [options]
```

## Comandos

| Comando                        | Descripción                                                 |
| ------------------------------ | ----------------------------------------------------------- |
| [`init`](/es/cli/init)         | Inicializa un nuevo archivo de configuración                |
| [`scan`](/es/cli/scan)         | Ejecuta un análisis arquitectónico único                    |
| [`diff`](/es/cli/diff)         | Compara el estado actual con una referencia (baseline)      |
| [`snapshot`](/es/cli/snapshot) | Guarda el estado actual en un archivo JSON                  |
| [`watch`](/es/cli/watch)       | Ejecuta en modo watch para retroalimentación en tiempo real |

## Opciones Globales

| Opción                | Descripción                                |
| --------------------- | ------------------------------------------ |
| `-c, --config <path>` | Ruta al archivo de configuración           |
| `-v, --verbose`       | Habilita el registro detallado (verbose)   |
| `-q, --quiet`         | Modo apto para CI (sin barras de progreso) |
| `-V, --version`       | Muestra información de la versión          |
| `-h, --help`          | Muestra la ayuda para un comando           |

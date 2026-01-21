---
title: init
description: "Configura rápidamente archlint en un nuevo proyecto generando un archivo de configuración con valores predeterminados sensatos."
---

# archlint init

El comando `init` te ayuda a configurar rápidamente archlint en un nuevo proyecto generando un archivo de configuración.

## Uso

```bash
archlint init [options]
```

## Opciones

| Opción             | Por defecto | Descripción                                                                      |
| ------------------ | ----------- | -------------------------------------------------------------------------------- |
| `-f, --force`      | `false`     | Sobrescribe el archivo `.archlint.yaml` si ya existe                             |
| `--no-interactive` | `false`     | Omite la selección interactiva de presets (paso 2)                               |
| `--presets <list>` | `none`      | Especifica explícitamente los presets (separados por comas o repitiendo el flag) |

## Cómo funciona

1. **Detección de Frameworks**: archlint analiza tu `package.json` y la estructura del proyecto para detectar los frameworks utilizados.
2. **Selección Interactiva**: A menos que se use `--no-interactive`, te pedirá confirmar o seleccionar presets adicionales.
3. **Generación de Configuración**: Crea un archivo `.archlint.yaml` con los presets seleccionados y una referencia al esquema JSON para soporte en el IDE.

## Ejemplos

### Inicialización interactiva

```bash
archlint init
```

### Inicialización no interactiva con presets específicos

```bash
# Separados por comas
archlint init --no-interactive --presets nestjs,prisma

# O repitiendo el flag
archlint init --no-interactive --presets nestjs --presets prisma
```

### Sobrescribir configuración existente

```bash
archlint init --force
```

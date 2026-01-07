# archlint scan

El comando `scan` realiza un análisis arquitectónico completo de tu proyecto.

## Uso

```bash
archlint scan [path] [options]
```

## Opciones

| Opción                      | Por defecto | Descripción                                               |
| --------------------------- | ----------- | --------------------------------------------------------- |
| `--format <format>`         | `table`     | Formato de salida: `table`, `json`, `markdown`            |
| `--report <file>`           | `stdout`    | Guarda el informe en un archivo                           |
| `--min-severity <sev>`      | `low`       | Filtra por severidad: `low`, `medium`, `high`, `critical` |
| `--detectors <ids>`         | `all`       | Lista de detectores a ejecutar, separados por comas       |
| `--exclude-detectors <ids>` | `none`      | Detectores a omitir                                       |
| `--no-cache`                | `false`     | Deshabilita el almacenamiento en caché del análisis       |

## Ejemplos

### Escaneo con informe en Markdown

```bash
archlint scan --format markdown --report report.md
```

### Ejecutar solo detección de ciclos

```bash
archlint scan --detectors cycles,circular_type_deps
```

### Solo severidad alta (high)

```bash
archlint scan --min-severity high
```

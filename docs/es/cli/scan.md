# archlint scan

El comando `scan` realiza un análisis arquitectónico completo de tu proyecto.

## Uso

```bash
archlint scan [path] [options]
```

## Opciones

| Opción                          | Por defecto | Descripción                                               |
| ------------------------------- | ----------- | --------------------------------------------------------- |
| `-f, --format <format>`         | `table`     | Formato de salida: `table`, `json`, `markdown`, `sarif`   |
| `-j, --json`                    | `false`     | Atajo para `--format json`                                |
| `-r, --report <file>`           | `stdout`    | Guarda el informe en un archivo                           |
| `-s, --min-severity <sev>`      | `low`       | Filtra por severidad: `low`, `medium`, `high`, `critical` |
| `-S, --min-score <score>`       | `none`      | Filtra por puntuación mínima de salud                     |
| `-d, --detectors <ids>`         | `all`       | Lista de detectores a ejecutar, separados por comas       |
| `-e, --exclude-detectors <ids>` | `none`      | Detectores a omitir                                       |
| `-A, --all`                     | `false`     | Ejecuta todos los detectores (incluidos los desactivados) |
| `--no-cache`                    | `false`     | Deshabilita el almacenamiento en caché del análisis       |
| `--no-git`                      | `false`     | Deshabilita la integración con git (salta análisis churn) |

## Ejemplos

### Escaneo con informe en Markdown

```bash
archlint scan --format markdown --report report.md
```

### Exportar a SARIF (para GitHub Code Scanning)

```bash
archlint scan --format sarif --report results.sarif
```

### Ejecutar solo detección de ciclos

```bash
archlint scan --detectors cycles,circular_type_deps
```

### Solo severidad alta (high)

```bash
archlint scan --min-severity high
```

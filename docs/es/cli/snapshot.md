# archlint snapshot

El comando `snapshot` captura el estado actual de la arquitectura de tu proyecto y lo guarda en un archivo JSON. Este archivo puede utilizarse posteriormente con el comando `diff`.

## Uso

```bash
archlint snapshot [options]
```

## Opciones

| Opción                | Por defecto              | Descripción                              |
| --------------------- | ------------------------ | ---------------------------------------- |
| `--output, -o <file>` | `archlint-snapshot.json` | El archivo donde se guardará el snapshot |

## Ejemplos

### Crear una referencia (baseline) para el proyecto

```bash
archlint snapshot -o .archlint-baseline.json
```

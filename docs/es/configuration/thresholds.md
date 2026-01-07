# Umbrales (Thresholds)

Los umbrales te permiten ajustar con precisión cuándo un detector debe informar de un "smell".

## Umbrales Comunes

| Detector       | Opción             | Por defecto | Descripción                                                 |
| -------------- | ------------------ | ----------- | ----------------------------------------------------------- |
| `cycles`       | `exclude_patterns` | `[]`        | Patrones glob para ignorar en la detección de ciclos        |
| `god_module`   | `fan_in`           | `10`        | Máximo de dependencias entrantes                            |
| `god_module`   | `fan_out`          | `10`        | Máximo de dependencias salientes                            |
| `god_module`   | `churn`            | `20`        | Máximo de commits de git en el historial                    |
| `god_module`   | `max_lines`        | `500`       | Máximo de líneas de código en el archivo                    |
| `complexity`   | `max_complexity`   | `15`        | Complejidad ciclomática máxima por función                  |
| `deep_nesting` | `max_depth`        | `4`         | Profundidad máxima de anidamiento para bloques              |
| `long_params`  | `max_params`       | `5`         | Máximo de parámetros por función                            |
| `large_file`   | `max_lines`        | `1000`      | Máximo de líneas por archivo                                |
| `lcom`         | `threshold`        | `1`         | Máximo de componentes no conectados permitidos en una clase |

## Ejemplo de Configuración

```yaml
thresholds:
  god_module:
    fan_in: 20
    max_lines: 800

  complexity:
    max_complexity: 10

  large_file:
    max_lines: 2000
```

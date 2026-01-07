# Lista de Parámetros Larga

**ID:** `long_params` | **Severity:** Low (default)

Identifica funciones o métodos que tienen demasiados parámetros.

## Por qué es un "smell"

Las funciones con muchos parámetros son difíciles de usar y de leer. A menudo indican que la función está haciendo demasiado o que algunos parámetros deberían agruparse en un objeto.

## Cómo solucionarlo

- **Introduce Parameter Object**: Agrupa parámetros relacionados en un solo objeto o interfaz.
- **Decompose Function**: Divide la función en otras más pequeñas que requieran menos parámetros.

## Configuración

```yaml
thresholds:
  long_params:
    max_params: 5
```

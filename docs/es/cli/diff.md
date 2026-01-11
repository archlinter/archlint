# archlint diff

El comando `diff` es la funcionalidad clave que implementa el enfoque Ratchet (mejora progresiva). Compara tu base de código actual con una instantánea (snapshot) guardada previamente o con otra rama/commit de git.

## Uso

```bash
# Comparar con un archivo de snapshot
archlint diff <baseline.json> [options]

# Comparar con una referencia de git
archlint diff <git-ref> [options]
```

## Cómo funciona

archlint no solo cuenta problemas. Realiza un **diff semántico** de los defectos arquitectónicos (smells):

1. **Nuevos problemas**: Defectos que existen ahora pero no existían en la referencia (ej., un nuevo ciclo).
2. **Problemas agravados**: Defectos existentes que se han vuelto más severos (ej., un ciclo que creció de 3 a 5 archivos).
3. **Problemas corregidos**: Defectos que existían en la referencia pero que ya no están.

## Opciones

| Opción                 | Por defecto | Descripción                                                                  |
| ---------------------- | ----------- | ---------------------------------------------------------------------------- |
| `--fail-on <severity>` | `medium`    | Sale con código 1 si se encuentra una regresión de esta severidad o superior |
| `--explain`            | `false`     | Proporciona una explicación detallada para cada regresión                    |

## Configuración

Puede ajustar el motor de diff en su archivo `.archlint.yaml`:

```yaml
diff:
  metric_threshold_percent: 20 # informar como regresión solo si la métrica empeoró >20%
  line_tolerance: 50 # ignorar desplazamientos de hasta 50 líneas en el diff difuso
```

Consulte la [Guía de configuración](/es/configuration/index#configuración-de-diff) para más detalles.

## Ejemplos

### Comprobar contra la rama principal en CI

```bash
archlint diff origin/main --fail-on medium --explain
```

### Comprobar contra una referencia local

```bash
archlint diff .archlint-baseline.json
```

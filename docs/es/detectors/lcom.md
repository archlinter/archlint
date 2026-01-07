# Cohesión Baja (LCOM4)

**ID:** `lcom` | **Severity:** Medium (default)

La cohesión mide qué tan estrechamente relacionados están los métodos y campos de una clase. archlint utiliza la métrica **LCOM4** (Lack of Cohesion of Methods).

## Por qué es un "smell"

- **Violación del SRP**: Es probable que la clase esté haciendo dos o más cosas no relacionadas.
- **Fragilidad**: Cambiar una parte de la clase podría afectar a partes no relacionadas.
- **Difícil de Reutilizar**: No puedes usar una parte de la clase sin arrastrar lógica no relacionada.

## Cómo solucionarlo

1. **Extract Class**: Divide la clase en dos o más clases más pequeñas, cada una con una única responsabilidad.
2. **Move Method**: Mueve los métodos que no utilizan el estado de la clase a una ubicación más apropiada (por ejemplo, un módulo de utilidades).

## Configuración

```yaml
thresholds:
  lcom:
    threshold: 1 # Número de componentes desconectados permitidos (por defecto es 1)
```

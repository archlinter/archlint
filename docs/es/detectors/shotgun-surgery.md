# Cirugía de Escopeta (Shotgun Surgery)

**ID:** `shotgun_surgery` | **Severity:** Medium (default)

La cirugía de escopeta ocurre cuando un solo cambio en tus requisitos requiere que realices muchos pequeños cambios en muchos módulos diferentes.

## Por qué es un smell

Indica que las responsabilidades están demasiado dispersas por la base de código. Hace que los cambios sean difíciles, lentos y propensos a errores.

## Cómo corregir

- **Mover Método/Campo (Move Method/Field)**: Consolida la lógica relacionada en un solo módulo.
- **Clase Inline (Inline Class)**: Si una clase es solo una colección de métodos que siempre se usan junto con otra clase, combínalas.

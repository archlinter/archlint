# Importaciones con Efectos Secundarios

**ID:** `side_effect_import` | **Severity:** Low (default)

Identifica importaciones que se realizan solo por sus efectos secundarios (p. ej., `import './globals';`), que a menudo modifican el estado global o los prototipos.

## Por qué es un smell

Las importaciones con efectos secundarios hacen que el gráfico de dependencias sea menos explícito y pueden llevar a un comportamiento no determinista según el orden de importación. A menudo son dependencias "ocultas" difíciles de rastrear.

## Cómo corregir

Intenta que la inicialización sea explícita. En lugar de confiar en una importación con efectos secundarios, exporta una función `init()` y llámala explícitamente.

# Estado Mutable Compartido

**ID:** `shared_mutable_state` | **Severity:** Medium (default)

Identifica variables exportadas que son mutables (p. ej., `export let ...` o `export var ...`).

## Por qué es un smell

El estado mutable global o compartido es una fuente común de errores que son extremadamente difíciles de rastrear. Hace que el comportamiento de un módulo sea impredecible y dependa del orden de ejecución.

## Cómo corregir

- **Usa Const**: Exporta solo constantes.
- **Encapsula**: Usa una clase o una función para gestionar el estado y proporcionar acceso controlado a través de métodos.
- **Usa un Gestor de Estado**: Si el estado realmente necesita ser compartido, usa una biblioteca de gestión de estado adecuada (Redux, Zustand, etc.).

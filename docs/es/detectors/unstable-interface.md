# Interfaz Inestable

**ID:** `unstable_interface` | **Severidad:** Medium (default)

Identifica módulos cuya API pública (exports) cambia frecuentemente según el historial de git, mientras que muchos otros módulos dependen de ella.

## Por qué esto es un problema

Una interfaz inestable causa un efecto dominó. Cada vez que la interfaz cambia, todos sus dependientes podrían necesitar ser actualizados, lo que lleva a mucho trabajo innecesario y posibles errores.

## Cómo corregir

- **Estabiliza la API**: Dedica más tiempo a diseñar la interfaz antes de la implementación.
- **Usa Versiones**: Si es posible, soporta múltiples versiones de la interfaz simultáneamente durante una transición.
- **Estrecha la Interfaz**: Exporta solo lo que sea absolutamente necesario.

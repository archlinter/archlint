# Interfaz Inestable

**ID:** `unstable_interface` | **Severidad:** Medium (default)

Identifica módulos que son un "blanco móvil" y cambian su API pública constantemente mientras todos los demás intentan construir sobre ellos.

## Por qué esto es un problema

- **El efecto dominó**: Cada vez que cambias un export público en un módulo inestable, potencialmente estás rompiendo una docena de otros archivos que dependen de él.
- **Trabajo innecesario**: Los desarrolladores pasan más tiempo arreglando imports y ajustándose a cambios de API que construyendo features.
- **Frustración**: Es difícil confiar en un módulo que rompe sus promesas cada dos semanas.

## Cómo corregir

- **Estabiliza la API**: Dedica más tiempo a diseñar la interfaz antes de la implementación.
- **Usa Versiones**: Si es posible, soporta múltiples versiones de la interfaz simultáneamente durante una transición.
- **Estrecha la Interfaz**: Exporta solo lo que sea absolutamente necesario.

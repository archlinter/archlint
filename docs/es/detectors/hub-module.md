# Módulo Hub

**ID:** `hub_module` | **Severity:** Medium (default)

Un "Módulo Hub" (Hub Module) es un módulo que actúa como un punto central en el grafo de dependencias, teniendo tanto un alto Fan-in como un alto Fan-out.

## Por qué esto es un "smell"

Los módulos hub son "puntos únicos de falla" peligrosos en su arquitectura. Debido a que tantas cosas dependen de ellos, y ellos dependen de tantas otras cosas, son extremadamente frágiles y difíciles de cambiar sin causar un efecto dominó en toda la base de código.

## Cómo solucionar

¡Rompa el hub! Identifique las diferentes rutas de datos o de control que pasan a través del hub y extráigalas en módulos separados y más enfocados.

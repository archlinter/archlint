---
title: Estado Mutable Compartido
description: "Detecta variables mutables exportadas que crean comportamiento impredecible y son una fuente común de errores difíciles de rastrear."
---

# Estado Mutable Compartido

**ID:** `shared_state` | **Severidad:** Medium (default)

Identifica variables exportadas que son mutables (p. ej., `export let ...` o `export var ...`).

## Por qué esto es un problema

El estado mutable global o compartido es una fuente común de errores que son extremadamente difíciles de rastrear. Hace que el comportamiento de un módulo sea impredecible y dependa del orden de ejecución.

## Cómo corregir

- **Usa Const**: Exporta solo constantes.
- **Encapsula**: Usa una clase o una función para gestionar el estado y proporcionar acceso controlado a través de métodos.
- **Usa un Gestor de Estado**: Si el estado realmente necesita ser compartido, usa una biblioteca de gestión de estado adecuada (Redux, Zustand, etc.).

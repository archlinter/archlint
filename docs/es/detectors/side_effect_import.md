---
title: Importaciones con Efectos Secundarios
description: "Identifica importaciones realizadas solo por sus efectos secundarios que modifican el estado global, haciendo las dependencias menos explícitas y el comportamiento no determinista."
---

# Importaciones con Efectos Secundarios

**ID:** `side_effect_import` | **Severidad:** Low (default)

Identifica importaciones que se realizan solo por sus efectos secundarios (p. ej., `import './globals';`), que a menudo modifican el estado global o los prototipos.

## Por qué esto es un problema

Las importaciones con efectos secundarios hacen que el gráfico de dependencias sea menos explícito y pueden llevar a un comportamiento no determinista según el orden de importación. A menudo son dependencias "ocultas" difíciles de rastrear.

## Patrones Excluidos

archlint ignora automáticamente las siguientes importaciones con efectos secundarios:

- **CSS/Recursos**: `import './styles.css'`, `import './image.png'`, etc.
- **Importaciones Dinámicas**: `import('./module')` o `require('./module')` dentro de funciones (a menudo utilizadas para carga perezosa o importaciones condicionales).

## Cómo corregir

Intenta que la inicialización sea explícita. En lugar de confiar en una importación con efectos secundarios, exporta una función `init()` y llámala explícitamente.

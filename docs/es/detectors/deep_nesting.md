---
title: Anidamiento Profundo
description: "Identifica bloques de código anidados demasiado profundamente, haciendo el código exponencialmente más difícil de leer e indicando funciones que hacen demasiado."
---

# Anidamiento Profundo

**ID:** `deep_nesting` | **Severidad:** Low (default)

Identifica bloques de código (if, for, while, etc.) que están anidados con demasiada profundidad.

## Por qué esto es un problema

El código profundamente anidado es exponencialmente más difícil de leer y comprender. A menudo es una señal de que una función está haciendo demasiado o que la lógica puede simplificarse.

## Cómo solucionar

- **Guard Clauses**: Devuelva temprano para evitar bloques `else` y reducir el anidamiento.
- **Extract Function**: Mueva el bloque anidado interno a una nueva función.
- **Flatten Logic**: Reevalúe la lógica para ver si se puede expresar de forma más sencilla.

## Configuración

```yaml
rules:
  deep_nesting:
    severity: low
    max_depth: 4
```

## Regla ESLint

Este detector está disponible como una regla ESLint para recibir retroalimentación en tiempo real en tu editor.

```javascript
// eslint.config.js
export default [
  {
    rules: {
      '@archlinter/no-deep-nesting': 'warn',
    },
  },
];
```

Consulta [Integración con ESLint](/es/integrations/eslint) para instrucciones de configuración.

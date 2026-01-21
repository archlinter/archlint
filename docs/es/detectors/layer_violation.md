---
title: Violación de Capa
description: "Detecta cuando el código en una capa arquitectónica importa incorrectamente código de otra capa, rompiendo abstracciones y el Principio de Responsabilidad Única."
---

# Violación de Capa

**ID:** `layer_violation` | **Severidad:** High (default)

La violación de capa (Layer violation) ocurre cuando el código en una capa arquitectónica importa código de una capa que no debería conocer (por ejemplo, la capa Domain importando de Infrastructure).

## Por qué esto es un problema

- **Rompe la Abstracción**: Los detalles de la implementación interna se filtran en la lógica de negocio de alto nivel.
- **Dificultad de Prueba**: La lógica de negocio se vuelve difícil de probar sin mocks para la infraestructura (BD, API, etc.).
- **Rigidez**: Cambiar una base de datos o una biblioteca externa requiere cambiar la lógica de negocio principal.

## Configuración

Debe definir sus capas en `.archlint.yaml`:

```yaml
rules:
  layer_violation:
    layers:
  - name: domain
    path: ['**/domain/**']
    allowed_imports: [] # Domain no importa nada

  - name: application
    path: ['**/application/**']
    allowed_imports: ['domain']

  - name: infrastructure
    path: ['**/infrastructure/**']
    allowed_imports: ['domain', 'application']
```

## Regla ESLint

Este detector está disponible como una regla ESLint para recibir retroalimentación en tiempo real en tu editor.

```javascript
// eslint.config.js
export default [
  {
    rules: {
      '@archlinter/no-layer-violations': 'error',
    },
  },
];
```

Consulta [Integración con ESLint](/es/integrations/eslint) para instrucciones de configuración.

## Cómo solucionar

1. **Inversión de Dependencia**: Defina una interfaz en la capa superior (Domain) e impleméntela en la capa inferior (Infrastructure).
2. **Refactorizar**: Mueva el código mal colocado a la capa adecuada.

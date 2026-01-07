# Lista de Parámetros Larga

**ID:** `long_params` | **Severidad:** Low (default)

Identifica funciones o métodos que tienen demasiados parámetros.

## Por qué esto es un problema

Las funciones con muchos parámetros son difíciles de usar y de leer. A menudo indican que la función está haciendo demasiado o que algunos parámetros deberían agruparse en un objeto.

## Cómo solucionarlo

- **Introduce Parameter Object**: Agrupa parámetros relacionados en un solo objeto o interfaz.
- **Decompose Function**: Divide la función en otras más pequeñas que requieran menos parámetros.

## Configuración

```yaml
rules:
  long_params:
    severity: info
    max_params: 5
    ignore_constructors: true
```

## Regla ESLint

Este detector está disponible como una regla ESLint para recibir retroalimentación en tiempo real en tu editor.

```javascript
// eslint.config.js
export default [
  {
    rules: {
      '@archlinter/no-long-params': 'warn',
    },
  },
];
```

Consulta [Integración con ESLint](/es/integrations/eslint) para instrucciones de configuración.

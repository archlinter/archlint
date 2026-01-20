# Lista de Parámetros Larga

**ID:** `long_params` | **Severidad:** Low (default)

Identifica funciones que piden demasiada información de una sola vez.

## Por qué esto es un problema

Las funciones con 10 parámetros son confusas al llamarlas y aún más confusas al leerlas. ¿El tercer argumento era el `userId` o el `orderId`? Cuando tienes una lista larga de argumentos, es señal de que la función está haciendo demasiado o que esos parámetros deberían estar juntos en un solo objeto.

## Cómo solucionarlo

- **Introduce Parameter Object**: Agrupa parámetros relacionados en un solo objeto o interfaz.
- **Decompose Function**: Divide la función en otras más pequeñas que requieran menos parámetros.

## Configuración

```yaml
rules:
  long_params:
    severity: low
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

# Anidamiento Profundo

**ID:** `deep_nesting` | **Severidad:** Low (default)

Identifica bloques de código (if, for, while, etc.) que están anidados con tanta profundidad que empiezan a parecer una pirámide.

## Por qué esto es un problema

Leer código con un anidamiento profundo es como leer una frase con demasiados (paréntesis (dentro de (otros paréntesis))). Es mentalmente agotador y suele ser una señal de que tu función está intentando manejar demasiados casos especiales a la vez. Es mejor fallar rápido o extraer la lógica.

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

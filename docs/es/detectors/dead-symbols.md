# Símbolos Muertos (Dead Symbols)

**ID:** `dead_symbols` | **Severidad:** Baja (por defecto)

Identifica funciones, variables o clases que se definen dentro de un archivo pero que nunca se utilizan, ni siquiera localmente.

## Por qué esto es un problema

Es simplemente desorden. Hace que el archivo sea más difícil de leer y mantener sin aportar ningún valor.

## Cómo solucionarlo

Elimina los símbolos no utilizados.

## Regla ESLint

Este detector está disponible como una regla ESLint para recibir retroalimentación en tiempo real en tu editor.

```javascript
// eslint.config.js
export default [
  {
    rules: {
      '@archlinter/no-dead-symbols': 'warn',
    },
  },
];
```

Consulta [Integración con ESLint](/es/integrations/eslint) para instrucciones de configuración.

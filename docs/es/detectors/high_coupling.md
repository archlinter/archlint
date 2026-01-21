---
title: Alto Acoplamiento
description: "Identifica módulos que dependen de demasiados otros módulos, creando rigidez y fragilidad en tu base de código."
---

# Alto Acoplamiento

**ID:** `high_coupling` | **Severidad:** Medium (default)

El alto acoplamiento ocurre cuando un módulo depende de demasiados otros módulos (alto Fan-out).

## Por qué esto es un problema

- **Rigidez**: Un cambio en cualquiera de las dependencias podría requerir un cambio en este módulo.
- **Fragilidad**: Es más probable que el módulo se rompa cuando cambia cualquiera de sus dependencias.
- **Difícil de Probar**: Requiere muchos mocks para aislarlo en las pruebas unitarias.

## Cómo solucionar

1. **Extraer responsabilidades**: Si un módulo tiene demasiadas dependencias, es probable que esté haciendo demasiado.
2. **Usar abstracciones**: Depende de una interfaz o una fachada en lugar de muchas implementaciones concretas.

## Configuración

```yaml
rules:
  high_coupling:
    severity: medium
    max_cbo: 20
```

## Regla ESLint

Este detector está disponible como una regla ESLint para recibir retroalimentación en tiempo real en tu editor.

```javascript
// eslint.config.js
export default [
  {
    rules: {
      '@archlinter/no-high-coupling': 'warn',
    },
  },
];
```

Consulta [Integración con ESLint](/es/integrations/eslint) para instrucciones de configuración.

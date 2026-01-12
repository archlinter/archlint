# Símbolos Muertos (Dead Symbols)

**ID:** `dead_symbols` | **Severidad:** Baja (por defecto)

Identifica funciones, variables o clases que se definen dentro de un archivo pero que nunca se utilizan, ni siquiera localmente.

## Por qué esto es un problema

Es simplemente desorden. Hace que el archivo sea más difícil de leer y mantener sin aportar ningún valor.

## Cómo solucionarlo

Elimina los símbolos no utilizados.

## Configuración

```yaml
rules:
  dead_symbols:
    severity: low
    # Lista de nombres de métodos a ignorar (por ejemplo, métodos de ciclo de vida del framework)
    ignore_methods:
      - 'constructor'
    # Mapa de métodos de interfaz/clase a ignorar cuando se implementan
    contract_methods:
      MyInterface: ['method1', 'method2']
      ValidatorConstraintInterface: ['validate', 'defaultMessage']
```

::: tip
**Falsos Positivos**: El análisis arquitectónico a veces puede producir falsos positivos, especialmente en proyectos con carga dinámica pesada, reflexión o contenedores complejos de Inyección de Dependencias (DI).
:::

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

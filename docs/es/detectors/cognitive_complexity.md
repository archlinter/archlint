# Complejidad cognitiva

**ID:** `cognitive_complexity` | **Severidad:** Media (por defecto)

Este detector identifica funciones con una alta complejidad cognitiva. La complejidad cognitiva mide qué tan difícil es entender el código, en lugar de solo cuántos caminos tiene.

## Por qué esto es un problema

- **Alta carga mental**: La lógica profundamente anidada y las expresiones booleanas complejas dificultan que los desarrolladores mantengan el estado en su cabeza.
- **Riesgo de mantenimiento**: El código que es difícil de entender es propenso a errores durante la modificación.
- **Errores ocultos**: Los errores de lógica a menudo se esconden en estructuras profundamente anidadas.

## Cómo se calcula

La complejidad cognitiva se calcula basándose en:

1.  **Incrementos estructurales**: `if`, `else`, `switch`, `for`, `while`, `do-while`, `catch`, operadores ternarios y secuencias lógicas.
2.  **Penalización por anidamiento**: Los incrementos para las estructuras de control aumentan según su nivel de anidamiento.
3.  **Casos especiales**: `switch` cuenta solo una vez para todo el bloque, independientemente del número de casos.

## Cómo solucionarlo

1.  **Aplanar la lógica**: Utiliza cláusulas de guarda (retornos tempranos) para reducir el anidamiento.
2.  **Extraer método**: Mueve bloques anidados o condiciones complejas para funciones pequeñas y enfocadas.
3.  **Simplificar expresiones**: Divide las condiciones booleanas complejas en variables o funciones intermedias.
4.  **Reemplazar ifs anidados**: Considera usar una tabla de búsqueda o el patrón Strategy.

## Configuración

```yaml
rules:
  cognitive_complexity:
    severity: medium
    max_complexity: 15
```

## Regla ESLint

Este detector está disponible como una regla ESLint para recibir retroalimentación en tiempo real en tu editor.

```javascript
// eslint.config.js
export default [
  {
    rules: {
      '@archlinter/no-high-cognitive-complexity': 'warn',
    },
  },
];
```

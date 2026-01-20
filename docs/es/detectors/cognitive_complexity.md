# Complejidad cognitiva

**ID:** `cognitive_complexity` | **Severidad:** Media (por defecto)

La complejidad cognitiva no trata solo de cuántas ramas tiene tu código; trata de cuánto esfuerzo le cuesta a un cerebro humano entenderlo de verdad. Es la diferencia entre un código "técnicamente correcto" y uno "legible".

## Por qué esto es un problema

- **Desbordamiento de la pila mental**: Los humanos no somos buenos manteniendo el hilo de cinco niveles de lógica anidada y álgebra booleana compleja al mismo tiempo. Cuando la carga mental es demasiado alta, empezamos a cometer errores.
- **Bugs invisibles**: A los errores les encanta esconderse en las sombras de los `if` anidados y los operadores ternarios infinitos.
- **Fricción en las revisiones**: Si a un desarrollador senior le lleva 20 minutos entender una función de 30 líneas durante una revisión de PR, es que es demasiado compleja.

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

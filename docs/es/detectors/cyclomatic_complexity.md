# Complejidad ciclomática (Cyclomatic Complexity)

**ID:** `cyclomatic_complexity` | **Severidad:** Media (por defecto)

La complejidad ciclomática mide cuántos caminos diferentes puede tomar la ejecución de tu código. Piénsalo como el factor "espagueti" de tus `if-else` y `switch`.

## Por qué esto es un problema

- **Laberinto mental**: Cada `if`, `else` y `case` añade un nuevo giro al laberinto. Si una función tiene 20 caminos, puedes apostar que un desarrollador acabará perdiéndose tarde o temprano.
- **Pesadilla de testing**: Para probar de verdad una función compleja, necesitarías un caso de prueba para cada camino posible. En el mundo real, eso suele significar que algunas ramas nunca se llegan a probar.
- **El "Efecto Mariposa"**: En funciones muy complejas, cambiar una sola línea de código puede tener consecuencias extrañas e impredecibles a cinco ramas de distancia.

## Cómo solucionarlo

1. **Extraer método (Extract Method)**: Divide la lógica compleja en funciones más pequeñas con nombre.
2. **Cláusulas de guarda (Guard Clauses)**: Utiliza retornos tempranos para reducir los niveles de anidamiento.
3. **Reemplazar condicional con polimorfismo**: Utiliza objetos o estrategias en lugar de bloques `switch` o `if/else` grandes.

## Configuración

```yaml
rules:
  cyclomatic_complexity:
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
      '@archlinter/no-high-cyclomatic-complexity': 'warn',
    },
  },
];
```

Consulta [Integración con ESLint](/es/integrations/eslint) para instrucciones de configuración.

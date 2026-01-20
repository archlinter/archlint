# Dependencias Cíclicas

**ID:** `cycles` | **Severidad:** Crítica (por defecto)

Las dependencias circulares ocurren cuando dos o más módulos dependen entre sí, ya sea directa o indirectamente. Es el clásico problema de "¿Qué fue primero, el huevo o la gallina?" aplicado al software.

## Por qué esto es un problema

- **Acoplamiento inseparable**: No puedes simplemente tomar un módulo y usarlo en otro lugar; te obliga a llevarte a toda su "familia" de dependencias contigo.
- **Trampas de inicialización**: Dependiendo de tu empaquetador (bundler), podrías terminar con importaciones "undefined" en tiempo de ejecución porque el ciclo no se pudo resolver a tiempo.
- **Pesadilla de pruebas**: Buena suerte intentando simular una parte del ciclo sin que toda la estructura colapse como un castillo de naipes.
- **Sobrecarga cognitiva**: Intentar seguir el flujo de datos en un ciclo es como leer un libro de "elige tu propia aventura" donde cada página te lleva de vuelta al inicio.

## Ejemplos

### Mal

```typescript
// orders.ts
import { processPayment } from './payments';
export const createOrder = () => {
  /* ... */
};

// payments.ts
import { createOrder } from './orders';
export const processPayment = () => {
  /* ... */
};
```

### Bien

Extrae la lógica compartida a un tercer módulo.

```typescript
// types.ts
export interface Order {
  /* ... */
}

// orders.ts
import { Order } from './types';
export const createOrder = () => {
  /* ... */
};

// payments.ts
import { Order } from './types';
export const processPayment = (order: Order) => {
  /* ... */
};
```

## Configuración

```yaml
rules:
  cycles:
    severity: critical
    exclude: ['**/*.test.ts']
```

## Cómo solucionarlo

1. **Extraer lógica compartida**: Mueve las partes comunes a un nuevo módulo del que dependan ambos módulos existentes.
2. **Inyección de Dependencias (Dependency Injection)**: Pasa las dependencias como argumentos en lugar de importarlas.
3. **Usar eventos**: Utiliza un bus de eventos o callbacks para desacoplar los módulos.

## Regla ESLint

Este detector está disponible como una regla ESLint para recibir retroalimentación en tiempo real en tu editor.

```javascript
// eslint.config.js
export default [
  {
    rules: {
      '@archlinter/no-cycles': 'error',
    },
  },
];
```

Consulta [Integración con ESLint](/es/integrations/eslint) para instrucciones de configuración.

# Dependencias Cíclicas

**ID:** `cycles` | **Severidad:** Crítica (por defecto)

Las dependencias circulares ocurren cuando dos o más módulos dependen entre sí, ya sea directa o indirectamente.

## Por qué esto es un problema (smell)

- **Acoplamiento Fuerte**: Los módulos son inseparables, lo que dificulta su reutilización independiente.
- **Problemas de Inicialización**: Pueden dar lugar a importaciones "undefined" en tiempo de ejecución si el bundler no las maneja con cuidado.
- **Dificultad en las Pruebas**: Es difícil simular (mock) o aislar un módulo sin incluir todo el ciclo.
- **Carga Cognitiva**: Es más difícil para los desarrolladores entender el flujo de datos y de control.

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
thresholds:
  cycles:
    exclude_patterns: ['**/*.test.ts']
```

## Cómo solucionarlo

1. **Extraer lógica compartida**: Mueve las partes comunes a un nuevo módulo del que dependan ambos módulos existentes.
2. **Inyección de Dependencias (Dependency Injection)**: Pasa las dependencias como argumentos en lugar de importarlas.
3. **Usar Eventos**: Utiliza un bus de eventos o callbacks para desacoplar los módulos.

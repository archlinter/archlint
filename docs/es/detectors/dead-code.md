# Código Muerto (Dead Code)

**ID:** `dead_code` | **Severidad:** Baja (por defecto)

El código muerto se refiere a funciones, clases o variables exportadas que no se importan ni se utilizan en ningún otro lugar del proyecto.

## Por qué esto es un problema (smell)

- **Carga de Mantenimiento**: Los desarrolladores podrían dedicar tiempo a actualizar o refactorizar código que ni siquiera se utiliza.
- **Tamaño del Bundle**: Aumenta el tamaño final de la aplicación (aunque muchos bundlers realizan tree-shaking).
- **Confusión**: Hace que la API de un módulo parezca más grande y compleja de lo que realmente es.

## Ejemplos

### Mal

```typescript
// utils.ts
export const usedHelper = () => { ... };
export const unusedHelper = () => { ... }; // Reportado como código muerto

// main.ts
import { usedHelper } from './utils';
```

## Cómo solucionarlo

1. **Eliminarlo**: Si realmente no se usa, la mejor acción es su eliminación.
2. **Marcar como Punto de Entrada (Entry Point)**: Si es parte de una API pública o una importación dinámica, añádelo a `entry_points` en tu configuración.

## Configuración

```yaml
entry_points:
  - 'src/index.ts'
  - 'src/api/**/*.ts'
```

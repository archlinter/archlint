# Código Muerto (Dead Code)

**ID:** `dead_code` | **Severidad:** Baja (por defecto)

El código muerto se refiere a funciones, clases o variables exportadas que no se importan ni se utilizan en ningún otro lugar del proyecto.

## Por qué esto es un problema

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
# Opciones específicas de la regla
rules:
  dead_code:
    exclude:
      - '**/tests/**'
      - '**/temp/**'

# Opciones globales (nivel raíz)
entry_points:
  - 'src/index.ts'
  - 'src/api/**/*.ts'
```

### Opciones

#### Opciones de la regla (`rules.dead_code`)

- `exclude`: Una lista de patrones glob para ignorar al detectar código muerto. Los archivos que coincidan con estos patrones se tratarán como si no existieran a efectos del análisis de dependencias entrantes.

#### Opciones globales (nivel raíz)

- `entry_points`: Puntos de entrada globales que nunca deben ser reportados como código muerto.

## Regla ESLint

Este detector está disponible como una regla ESLint para recibir retroalimentación en tiempo real en tu editor.

```javascript
// eslint.config.js
export default [
  {
    rules: {
      '@archlinter/no-dead-code': 'warn',
    },
  },
];
```

Consulta [Integración con ESLint](/es/integrations/eslint) para instrucciones de configuración.

# Código Muerto (Dead Code)

**ID:** `dead_code` | **Severidad:** Baja (por defecto)

El código muerto es exactamente lo que parece: funciones, clases o variables que están "vivas" en tu código pero que no hacen nada porque nadie las usa.

## Por qué esto es un problema

- **Gasto de energía mental**: Los desarrolladores no deberían tener que refactorizar o intentar entender código que ni siquiera se está ejecutando.
- **Falsa complejidad**: Hace que la API de tus módulos parezca más grande y aterradora de lo que realmente es.
- **Fantasmas en la máquina**: Puede provocar momentos de "pensé que habíamos eliminado esto" durante la depuración.

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

# Complejidad Elevada

**ID:** `complexity` | **Severidad:** Media (por defecto)

Este detector identifica funciones con una alta Complejidad Ciclomática.

## Por qué esto es un problema

- **Difícil de Entender**: Demasiadas ramificaciones hacen que el código sea difícil de seguir.
- **Propenso a Errores**: Mayor probabilidad de pasar por alto casos borde durante las pruebas.
- **Pesadilla de Mantenimiento**: Pequeños cambios pueden tener efectos impredecibles debido a la lógica compleja.

## Cómo solucionarlo

1. **Extraer Método (Extract Method)**: Divide la lógica compleja en funciones más pequeñas con nombre.
2. **Cláusulas de Guarda (Guard Clauses)**: Utiliza retornos tempranos para reducir los niveles de anidamiento.
3. **Reemplazar Condicional con Polimorfismo**: Utiliza objetos o estrategias en lugar de bloques `switch` o `if/else` grandes.

## Configuración

```yaml
rules:
  complexity:
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
      '@archlinter/no-high-complexity': 'warn',
    },
  },
];
```

Consulta [Integración con ESLint](/es/integrations/eslint) para instrucciones de configuración.

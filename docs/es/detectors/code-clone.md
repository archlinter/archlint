# Clon de Código

**ID:** `code_clone` | **Severidad:** Media (por defecto)

Este detector identifica bloques de código duplicados en su proyecto. Utiliza la tokenización basada en AST para encontrar coincidencias exactas, ignorando las diferencias en el formato y los comentarios.

## Por qué es un problema

- **Sobrecarga de mantenimiento**: Corregir un error o realizar un cambio en un lugar requiere actualizar todos los duplicados.
- **Violación de DRY**: La duplicación es una señal clara de que falta abstracción o reutilización.
- **Evolución inconsistente**: Con el tiempo, los duplicados pueden divergir, lo que provoca errores sutiles y dificulta la refactorización.

## Cómo solucionar

1. **Extract Method**: Mueva la lógica compartida a una sola función y llámela desde varios lugares.
2. **Componentes genéricos**: Para el código de UI, cree un componente reutilizable con props.
3. **Módulos de utilidad**: Mueva la lógica de ayuda común a un archivo de utilidad compartido.

## Configuración

```yaml
rules:
  code_clone:
    enabled: true
    severity: warn
    min_tokens: 50
    min_lines: 6
```

### Opciones

- `min_tokens`: El número mínimo de tokens normalizados para activar la detección de un clon (por defecto: 50).
- `min_lines`: El número mínimo de líneas que debe ocupar el clon (por defecto: 6).

## Regla de ESLint

Este detector está disponible como una regla de ESLint para comentarios en tiempo real en su editor.

```javascript
// eslint.config.js
export default [
  {
    rules: {
      '@archlinter/no-code-clone': 'warn',
    },
  },
];
```

Consulte [Integración con ESLint](/es/integrations/eslint) para obtener instrucciones de configuración.

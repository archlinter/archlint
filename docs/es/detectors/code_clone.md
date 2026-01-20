# Clon de Código

**ID:** `code_clone` | **Severidad:** Media (por defecto)

Este detector encuentra dónde alguien tomó el atajo del "copiar-pegar". Busca lógica idéntica que ha sido duplicada en todo tu proyecto.

## Por qué es un problema

- **Los bugs se multiplican**: Si encuentras un bug en una copia, tienes que recordar arreglarlo en las otras cuatro. Spoiler: normalmente olvidas una.
- **Sobrecarga de mantenimiento**: Cada vez que quieres cambiar cómo funciona una lógica específica, estás haciendo el mismo trabajo una y otra vez.
- **Evolución inconsistente**: Con el tiempo, una copia se actualiza mientras otra no, y de repente tu lógica "idéntica" se comporta distinto en diferentes partes de la app.

## Cómo solucionar

1. **Extract Method**: Mueva la lógica compartida a una sola función y llámela desde varios lugares.
2. **Componentes genéricos**: Para el código de UI, cree un componente reutilizable con props.
3. **Módulos de utilidad**: Mueva la lógica de ayuda común a un archivo de utilidad compartido.

## Configuración

```yaml
rules:
  code_clone:
    enabled: true
    severity: medium
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

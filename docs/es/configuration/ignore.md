# Ignorar Archivos

archlint proporciona varias formas de excluir archivos o directorios del análisis.

## Ignorar Globalmente

La sección `ignore` en la raíz de `.archlint.yaml` especifica los archivos que todos los detectores deben omitir por completo.

```yaml
ignore:
  - '**/node_modules/**'
  - '**/dist/**'
  - '**/coverage/**'
  - '**/tmp/**'
  - '**/*.d.ts'
```

## Soporte para .gitignore

Por defecto, archlint respeta automáticamente su archivo `.gitignore`. No necesita duplicar estos patrones en `.archlint.yaml`. Si desea desactivar este comportamiento, establezca `git: { enabled: false }`.

## Ignorar por Regla

Puede excluir archivos de un detector específico usando el campo `exclude` dentro de la sección `rules`. Esto es útil si desea que un archivo sea analizado por la mayoría de los detectores pero omitido por uno específico.

```yaml
rules:
  cycles:
    exclude:
      - '**/generated/**'
      - '**/*.entity.ts'
```

## Sobrescrituras de Rutas (Overrides)

Para una lógica más compleja (por ejemplo, cambiar configuraciones o desactivar varias reglas para un directorio específico), use la sección `overrides`:

```yaml
overrides:
  - files: ['**/tests/**', '**/mocks/**']
    rules:
      complexity: off
      god_module: off
      large_file: medium
```

## Ignorar en Línea

Puede ignorar problemas arquitectónicos específicos directamente en su código fuente utilizando comentarios especiales. Esto es útil para suprimir advertencias en casos excepcionales.

### Uso:

1. **Todo el archivo**: Agregue `// archlint-disable` al principio del archivo.
2. **Línea actual**: Agregue `// archlint-disable-line` al final de la línea o en la línea de arriba.
3. **Siguiente línea**: Use `// archlint-disable-next-line` antes de la línea problemática.

### Ejemplos:

```typescript
// archlint-disable-next-line complexity
function veryComplexFunction() {
  // El detector de complejidad será ignorado para esta función
}

import { internal } from './private'; // archlint-disable-line layer_violation

// archlint-disable cycles, god_module
// Ignorar reglas específicas para todo el archivo
```

Puede especificar múltiples reglas separadas por comas o usar `*` para ignorar todas las reglas.

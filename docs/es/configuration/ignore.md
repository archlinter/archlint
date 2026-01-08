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

Por defecto, archlint respeta automáticamente su archivo `.gitignore`. No necesita duplicar estos patrones en `.archlint.yaml`. Si desea desactivar este comportamiento, establezca `enable_git: false`.

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
      large_file: warn
```

## Ignorar en Línea

(En desarrollo) Estamos trabajando en el soporte de comentarios como `// archlint-disable` para ignorar líneas o archivos específicos directamente en el código.

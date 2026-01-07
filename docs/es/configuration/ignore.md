# Patrones de Ignorado (Ignore Patterns)

archlint proporciona varias formas de excluir archivos o directorios del análisis.

## Ignorado Global

La sección `ignore` en `archlint.yaml` especifica los archivos que todos los detectores deben omitir por completo.

```yaml
ignore:
  - '**/node_modules/**'
  - '**/dist/**'
  - '**/coverage/**'
  - '**/tmp/**'
  - '**/*.d.ts'
```

## Soporte para .gitignore

Por defecto, archlint respeta automáticamente tu archivo `.gitignore`. No necesitas duplicar esos patrones en tu `archlint.yaml`.

## Ignorado Específico por Detector

Algunos detectores tienen sus propios `exclude_patterns` dentro de la sección `thresholds`. Esto es útil si quieres que un archivo sea analizado por la mayoría de los detectores pero omitido por uno específico (por ejemplo, excluir archivos de prueba de la detección de ciclos).

```yaml
thresholds:
  cycles:
    exclude_patterns:
      - '**/*.test.ts'
      - '**/*.spec.ts'
```

## Ignorados en Línea (Inline)

(Próximamente) Estamos trabajando para soportar comentarios en línea como `// archlint-disable` para ignorar líneas o archivos específicos directamente en el código fuente.

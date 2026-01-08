# Archivo Grande

**ID:** `large_file` | **Severidad:** Medium (default)

Identifica archivos de origen que superan un cierto número de líneas.

## Por qué esto es un problema

Los archivos extremadamente grandes son difíciles de navegar, comprender y mantener. Por lo general, indican una violación del Principio de Responsabilidad Única.

## Cómo solucionar

Divida el archivo en módulos más pequeños y enfocados.

## Configuración

```yaml
rules:
  large_file:
    max_lines: 1000
```

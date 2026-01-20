# Archivo Grande

**ID:** `large_file` | **Severidad:** Medium (default)

Identifica archivos que han crecido tanto que probablemente merecerían su propio código postal.

## Por qué esto es un problema

Los archivos extremadamente grandes son una pesadilla para navegar. Pasas más tiempo haciendo scroll y buscando símbolos que escribiendo código real. Normalmente, un archivo de 2000 líneas es simplemente tres o cuatro módulos lógicos más pequeños disfrazados. Viola el Principio de Responsabilidad Única y prácticamente garantiza conflictos de merge.

## Cómo solucionar

Divida el archivo en módulos más pequeños y enfocados.

## Configuración

```yaml
rules:
  large_file:
    max_lines: 1000
```

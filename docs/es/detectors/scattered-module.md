# Módulo Disperso

**ID:** `scattered_module` | **Severity:** Medium (default)

Identifica un "módulo" (a menudo una carpeta o una agrupación lógica) donde los archivos internos no están bien conectados entre si, lo que indica que el módulo es solo una colección aleatoria de código.

## Por qué es un "smell"

Un módulo debe ser cohesivo. Si sus partes internas no interactúan entre sí, es probable que no sea un módulo real y deba dividirse o reestructurarse.

## Cómo solucionarlo

Reevalúa el propósito del módulo. Agrupa el código en módulos más cohesivos o mueve las partes no relacionadas a donde se utilicen realmente.

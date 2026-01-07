# Módulo Dios

**ID:** `god_module` | **Severity:** High (default)

Un "Módulo Dios" (God Module) es un archivo que ha crecido demasiado y ha asumido demasiadas responsabilidades.

## Por qué esto es un "smell"

- **Viola el Principio de Responsabilidad Única**: El módulo hace demasiadas cosas.
- **Conflictos de Fusión**: Los cambios frecuentes de diferentes desarrolladores provocan conflictos constantes.
- **Fragilidad**: Los cambios en una parte del módulo pueden romper inesperadamente otra parte.
- **Difícil de Probar**: Requiere una configuración compleja para probar varias funcionalidades no relacionadas.

## Criterios de Detección

archlint identifica los Módulos Dios basándose en:

- **Fan-in**: Número de otros módulos que dependen de él.
- **Fan-out**: Número de módulos de los que depende.
- **Churn**: Frecuencia de cambios en git.
- **Lines of Code**: Tamaño total del archivo.

## Cómo solucionar

1. **Identificar Responsabilidades**: Enumere todas las diferentes tareas que realiza el módulo.
2. **Extraer Módulos**: Divida el archivo en módulos más pequeños y enfocados.
3. **Patrón de Fachada**: Si el módulo actúa como coordinador, mantenga solo la lógica de coordinación y delegue el trabajo a los submódulos.

## Configuración

```yaml
thresholds:
  god_module:
    fan_in: 15
    fan_out: 15
    churn: 20
    max_lines: 500
```

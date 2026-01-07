# Abuso de Archivos Barrel

**ID:** `barrel_file_abuse` | **Severidad:** Media (por defecto)

Los archivos barrel (por ejemplo, archivos `index.ts` que solo reexportan otros archivos) pueden volverse problemáticos cuando crecen demasiado o incluyen demasiadas exportaciones no relacionadas.

## Por qué esto es un problema (smell)

- **Dependencias Circulares**: Los archivos barrel grandes son una causa común de dependencias circulares indirectas.
- **Acoplamiento Innecesario**: Importar una sola cosa de un archivo barrel grande puede hacer que el bundler incluya muchos módulos no relacionados.
- **Rendimiento**: Puede ralentizar tanto el desarrollo (indexación del IDE) como la producción (tamaño del bundle/tiempo de carga).

## Cómo solucionarlo

- Evita los archivos barrel "atrapa-todo" en la raíz de directorios grandes.
- Prefiere importaciones directas si un archivo barrel está causando problemas.
- Agrupa las exportaciones en archivos barrel más pequeños y específicos.

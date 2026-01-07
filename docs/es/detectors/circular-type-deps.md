# Ciclos de Tipos

**ID:** `circular_type_deps` | **Severidad:** Media (por defecto)

Similar a las dependencias circulares, pero específicamente para importaciones de solo tipos (por ejemplo, `import type { ... }`).

## Por qué esto es un problema (smell)

Aunque los ciclos de solo tipos no causan problemas en tiempo de ejecución en TypeScript, siguen indicando un acoplamiento arquitectónico fuerte. Hacen que sea más difícil separar los módulos y pueden dar lugar a grafos de dependencias complejos que son difíciles de razonar.

## Cómo solucionarlo

Mueve los tipos compartidos a un módulo de `types` común o a un archivo separado que no dependa de los módulos de implementación.

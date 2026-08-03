---
title: Ignoring Files
description: 'Aprende cómo excluir archivos o directorios del análisis de archlint usando ignore global, gitignore, exclusiones específicas por regla y comentarios en línea.'
---

# Ignorar archivos

Archlint proporciona varias formas de excluir archivos o directorios del análisis.

## Ignorar globalmente

La sección `ignore` en la raíz de `.archlint.yaml` excluye archivos del análisis. Los archivos ignorados se siguen analizando sintácticamente, por lo que los imports que pasan por ellos se siguen resolviendo, pero no se reporta ningún smell para ellos y no cuentan para ningún umbral de detector (por ejemplo, `min_dependents` o `max_files_per_package`).

```yaml
ignore:
  - '**/node_modules/**'
  - '**/dist/**'
  - '**/coverage/**'
  - '**/tmp/**'
  - '**/*.d.ts'
```

Los patrones se comparan con rutas relativas a la raíz del proyecto:

- Una ruta simple — `legacy`, `src/legacy`, `generated.ts` — coincide con esa entrada y con todo lo que hay debajo, a cualquier profundidad.
- Un patrón que contenga `*`, `?` o `[` se usa como glob. Ten en cuenta que `*` también coincide con `/`, por lo que `src/*.ts` cubre `src/a/b/c.ts`.
- Los patrones inutilizables se reportan como advertencias y se omiten; el resto de los patrones sigue aplicándose.

Los ciclos son la única excepción a la regla anterior: un ciclo es un hecho sobre un grupo de archivos, por lo que un ciclo que pasa tanto por archivos activos como ignorados se sigue reportando, con sus miembros ignorados incluidos. Un ciclo cuyos miembros están todos ignorados no se reporta.

Definir `ignore` **reemplaza** los valores predeterminados integrados (`**/*.test.ts`, `**/__tests__/**` y los demás patrones de archivos de prueba) en lugar de añadirse a ellos. Repite los que necesites si aún quieres excluir archivos de prueba.

## Soporte para .gitignore

Por defecto, Archlint respeta automáticamente su archivo `.gitignore`. No necesita duplicar estos patrones en `.archlint.yaml`. Si desea desactivar este comportamiento, establezca `git: { enabled: false }`.

## Ignorar por regla

Puede excluir archivos de un detector específico usando el campo `exclude` dentro de la sección `rules`. Esto es útil si desea que un archivo sea analizado por la mayoría de los detectores pero omitido por uno específico.

```yaml
rules:
  cycles:
    exclude:
      - '**/generated/**'
      - '**/*.entity.ts'
```

## Sobrescrituras de rutas (overrides)

Para una lógica más compleja (por ejemplo, cambiar configuraciones o desactivar varias reglas para un directorio específico), utilice la sección `overrides`:

```yaml
overrides:
  - files: ['**/tests/**', '**/mocks/**']
    rules:
      cyclomatic_complexity: off
      cognitive_complexity: off
      god_module: off
      large_file: medium
```

## Ignorar en línea

Puede ignorar problemas arquitectónicos específicos directamente en su código fuente utilizando comentarios especiales. Esto es útil para suprimir advertencias en casos excepcionales.

### Uso:

Se admiten tanto la sintaxis de comentario de una sola línea (`// archlint-...`) como la de comentario de bloque (`/* archlint-... */`) para todos los patrones.

1. **Todo el archivo**: Agregue `// archlint-disable` al principio del archivo.
2. **Línea actual**: Agregue `// archlint-disable-line` al final de la línea o en la línea de arriba.
3. **Siguiente línea**: Utilice `// archlint-disable-next-line` antes de la línea problemática.
4. **Bloques**: Utilice `// archlint-disable` y `// archlint-enable` para envolver una sección de código.

### Ejemplos:

```typescript
// archlint-disable * - Todo el archivo utiliza patrones heredados
// Ignorar todas las reglas para todo el archivo

// prettier-ignore
// archlint-disable-next-line long-params - Esta función heredada requiere muchos parámetros
function processTransaction(id: string, amount: number, currency: string, date: Date, recipient: string, note: string) {
  // El detector de parámetros largos será ignorado solo para esta línea
}

import { internal } from './private'; // archlint-disable-line layer_violation - Exclusión temporal para migración

/* archlint-disable cyclomatic_complexity, cognitive_complexity */
function legacyCode() {
  // Este bloque será ignorado para ambos tipos de complejidad
}
/* archlint-enable cyclomatic_complexity, cognitive_complexity */
```

Puede especificar múltiples reglas separadas por comas o usar `*` para ignorar todas las reglas.

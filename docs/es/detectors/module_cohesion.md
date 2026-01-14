# Módulo Disperso

**ID:** `module_cohesion` | **Severidad:** Medium (default)

Identifica un "módulo" (típicamente un archivo o grupo lógico) donde los elementos internos (funciones, clases) no están bien conectados. Esto indica que el módulo carece de un propósito cohesivo y probablemente es una colección de código no relacionado.

## Por qué esto es un problema

Un módulo debe ser cohesivo, siguiendo el principio de que "las cosas que cambian juntas deben permanecer juntas". Si las partes internas de un módulo no interactúan entre sí, no es un módulo real—es solo una carpeta o archivo usado como contenedor aleatorio. Esto hace que el código sea más difícil de encontrar y aumenta la carga cognitiva.

## Ejemplos

### Mal

Un archivo que contiene funciones auxiliares no relacionadas que no comparten lógica ni datos comunes.

```typescript
// misc-utils.ts
export const formatCurrency = (val: number) => { ... };
export const validateEmail = (email: string) => { ... };
export const parseJwt = (token: string) => { ... };
// Estas tres funciones no comparten estado ni lógica común.
```

### Bien

Agrupa las funciones no relacionadas en módulos específicos y cohesivos.

```typescript
// currency-utils.ts
export const formatCurrency = (val: number) => { ... };

// validation-utils.ts
export const validateEmail = (email: string) => { ... };
```

## Configuración

```yaml
rules:
  module_cohesion:
    severity: medium
    min_exports: 5
    max_components: 2
```

## Cómo solucionarlo

Reevalúa el propósito del módulo. Agrupa el código en módulos más cohesivos o mueve las partes no relacionadas a donde se utilicen realmente.

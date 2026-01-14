# Obsesión por los Primitivos

**ID:** `primitive_obsession` | **Severidad:** Low (default)

La obsesión por los primitivos es el uso excesivo de tipos primitivos (cadenas, números, booleanos) para representar conceptos del dominio que podrían representarse mejor mediante un tipo o clase específica (por ejemplo, usar un `string` para una dirección de correo electrónico o un `number` para una moneda).

## Por qué esto es un problema

Los primitivos no tienen comportamiento ni validación. Al utilizar un tipo específico del dominio, puedes encapsular la lógica de validación y hacer que el código se autodocumente mejor.

## Cómo solucionarlo

Crea una clase o un alias de tipo (en TypeScript) con lógica de validación para el concepto del dominio.

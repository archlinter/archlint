# Módulo Espalhado

**ID:** `module_cohesion` | **Gravidade:** Medium (default)

Identifica um "módulo" (geralmente um arquivo ou agrupamento lógico) onde os elementos internos (funções, classes) não estão bem conectados. Isso indica que o módulo carece de um propósito coeso e provavelmente é uma coleção de código não relacionado.

## Por que isso é um smell

Um módulo deve ser coeso, seguindo o princípio de que "coisas que mudam juntas devem ficar juntas". Se as partes internas de um módulo não interagem entre si, não é um módulo de verdade—é apenas uma pasta ou arquivo usado como depósito aleatório. Isso torna o código mais difícil de encontrar e aumenta a carga cognitiva.

## Exemplos

### Ruim

Um arquivo contendo funções auxiliares não relacionadas que não compartilham lógica ou dados comuns.

```typescript
// misc-utils.ts
export const formatCurrency = (val: number) => { ... };
export const validateEmail = (email: string) => { ... };
export const parseJwt = (token: string) => { ... };
// Essas três funções não compartilham estado ou lógica comum.
```

### Bom

Agrupe funções não relacionadas em módulos específicos e coesos.

```typescript
// currency-utils.ts
export const formatCurrency = (val: number) => { ... };

// validation-utils.ts
export const validateEmail = (email: string) => { ... };
```

## Configuração

```yaml
rules:
  module_cohesion:
    severity: medium
    min_exports: 5
    max_components: 2
```

## Como consertar

Reavalie o propósito do módulo. Agrupe o código em módulos mais coesos ou mova as partes não relacionadas para onde elas são realmente usadas.

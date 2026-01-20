# Ciclos de Tipos (Type Cycles)

**ID:** `circular_type_deps` | **Gravidade:** Média (padrão)

Semelhante a dependências circulares, mas especificamente para imports apenas de tipos (ex: `import type { ... }`).

## Por que isso é um smell

Embora ciclos apenas de tipos não causem problemas em tempo de execução no TypeScript, eles ainda indicam um acoplamento arquitetural forte. Eles tornam mais difícil separar módulos e ainda podem levar a grafos de dependência complexos e difíceis de entender.

## Exemplo

### Ruim

```typescript
// userService.ts
import type { UserProfile } from './profileService';

export interface User {
  id: string;
  profile: UserProfile;
}

// profileService.ts
import type { User } from './userService';

export interface UserProfile {
  id: string;
  owner: User;
}
```

## Como corrigir

Mova os tipos compartilhados para um arquivo `types.ts` comum ou um módulo separado que não dependa da implementação.

### Bom

```typescript
// types.ts
export interface User {
  id: string;
  profile: UserProfile;
}

export interface UserProfile {
  id: string;
  owner: User;
}

// userService.ts
import type { User, UserProfile } from './types';
// ... use User aqui
```

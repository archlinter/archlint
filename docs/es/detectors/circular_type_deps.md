# Ciclos de Tipos

**ID:** `circular_type_deps` | **Severidad:** Media (por defecto)

Similar a las dependencias circulares, pero específicamente para importaciones de solo tipos (por ejemplo, `import type { ... }`).

## Por qué esto es un problema

Aunque los ciclos de solo tipos no causan problemas en tiempo de ejecución en TypeScript, siguen indicando un acoplamiento arquitectónico fuerte. Hacen que sea más difícil separar los módulos y pueden dar lugar a grafos de dependencias complejos que son difíciles de razonar.

## Ejemplo

### Mal

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

## Cómo solucionarlo

Mueve los tipos compartidos a un archivo `types.ts` común o a un módulo separado que no dependa de la implementación.

### Bien

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
// ... usa User aquí
```

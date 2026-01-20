# Циклы в типах

**ID:** `circular_type_deps` | **Степень критичности:** Средняя (по умолчанию)

Аналогично циклическим зависимостям, но относится конкретно к импортам только типов (например, `import type { ... }`).

## Почему это «запах»

Хотя циклы только в типах не вызывают проблем во время выполнения в TypeScript, они все же указывают на сильную архитектурную связанность. Они затрудняют разделение модулей и могут приводить к сложным графам зависимостей, в которых трудно ориентироваться.

## Пример

### Плохо

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

## Как исправить

Вынесите общие типы в отдельный файл `types.ts` или независимый модуль, который не зависит от реализации.

### Хорошо

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
// ... используйте User здесь
```

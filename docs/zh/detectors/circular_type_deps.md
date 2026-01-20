# 类型循环 (Type Cycles)

**ID:** `circular_type_deps` | **严重程度:** 中 (默认)

类似于循环依赖，但专门针对仅类型的导入（例如 `import type { ... }`）。

## 为什么这是一种坏味道

虽然仅类型循环在 TypeScript 中不会导致运行时问题，但它们仍然表明存在紧密的架构耦合。它们使得分离模块变得更加困难，并且仍然会导致难以推导的复杂依赖图。

## 示例

### 坏习惯

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

## 如何修复

将共享类型移动到通用的 `types.ts` 或不依赖于实现模块的单独文件中。

### 好的做法

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
// ... 在这里使用 User
```

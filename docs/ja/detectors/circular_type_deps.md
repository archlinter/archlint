# 型の循環 (Type Cycles)

**ID:** `circular_type_deps` | **重要度:** 中 (デフォルト)

循環依存に似ていますが、特に型のみのインポート（例：`import type { ... }`）に関するものです。

## なぜこれが「不吉な臭い」なのか

型のみの循環は TypeScript において実行時の問題を引き起こしませんが、依然としてアーキテクチャ上の密結合を示しています。モジュールの分離を困難にし、推論が難しい複雑な依存関係グラフを招く可能性があります。

## 例

### Bad

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

## 修正方法

共有されている型を共通の `types.ts` モジュール、または実装モジュールに依存しない別のファイルに移動してください。

### Good

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
// ... ここで User を使用します
```

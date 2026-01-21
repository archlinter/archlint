---
title: 安定依存の原則 (SDP)
description: "依存関係が安定性に向かって流れることを保証します—安定したモジュールは不安定なモジュールに依存すべきではなく、ファンインとファンアウトで測定されます。"
---

# 安定依存の原則 (SDP)

**ID:** `sdp_violation` | **重要度:** Medium (default)

安定依存の原則（Stable Dependencies Principle）は、「パッケージ間の依存関係は、安定している方向に向かうべきである」と述べています。言い換えれば、安定した（変更が困難な）モジュールが、不安定な（変更が容易な）モジュールに依存すべきではありません。

この文脈での安定度は、他のモジュールがそのモジュールに依存している数（ファンイン）と、そのモジュールが依存しているモジュールの数（ファンアウト）によって測定されます。

## なぜこれが「不吉な臭い」なのか

多くの他のコンポーネントが依存している安定したモジュールが、不安定なモジュールに依存すると、変更が困難になります。不安定な依存関係の変更が安定したモジュールを壊す可能性があり、それがすべての依存元に波及します。これにより、不安定なモジュールが事実上「凍結」されるか、システム全体が脆弱になります。

## 例

### Bad

コアドメインエンティティ（安定）が、特定のデータベース実装や頻繁に変更される UI コンポーネント（不安定）に依存している場合。

```typescript
// domain/user.ts (安定、多くのものが User に依存)
import { UserPreferencesUI } from '../ui/user-prefs'; // 不安定な依存関係

export class User {
  updateSettings(prefs: UserPreferencesUI) {
    // ...
  }
}
```

### Good

安定したモジュールは、変更が稀な抽象（インターフェースなど）に依存します。

```typescript
// domain/user.ts
export interface UserSettings {
  theme: string;
  notifications: boolean;
}

export class User {
  updateSettings(settings: UserSettings) {
    // ...
  }
}
```

## 設定

```yaml
rules:
  sdp_violation:
    severity: medium
    min_fan_total: 5
    instability_diff: 0.3
```

## ESLint ルール

このディテクターは、エディター内でリアルタイムのフィードバックを提供する ESLint ルールとして利用可能です。

```javascript
// eslint.config.js
export default [
  {
    rules: {
      '@archlinter/no-sdp-violations': 'warn',
    },
  },
];
```

セットアップ手順については [ESLint Integration](/ja/integrations/eslint) を参照してください。

## 修正方法

コアとなる安定したモジュールが、揮発性の高いモジュールに依存していないことを確認してください。インターフェースや抽象クラスを使用して、それらを疎結合（decouple）にします。

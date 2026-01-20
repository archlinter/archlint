# ハブモジュール

**ID:** `hub_module` | **重要度:** Medium (default)

「ハブモジュール」は、信号機のない忙しい交差点のようなものです。誰もが依存するモジュールであり、それ自体も他のすべてに依存していますが、実際には自分自身で*多くのロジックを行う*わけではありません。

## なぜこれが「不吉な臭い」なのか

ハブモジュールは危険な単一障害点です。すべての中心に位置するため、信じられないほど脆弱です。ハブへの小さな変更が、アプリの無関係な部分を壊す可能性があり、コードベース全体で最も恐ろしいリファクタリング対象のファイルになります。アーキテクチャの究極の「ボトルネック」です。

## 例

### Bad

多くの無関係なサービスを単に再エクスポートまたは調整するだけで、アプリケーション全体から使用されるモジュール。

```typescript
// app-hub.ts
import { AuthService } from './auth';
import { ApiService } from './api';
import { LoggerService } from './logger';
import { ConfigService } from './config';
// ... 10以上のインポート

export class AppHub {
  constructor(
    public auth: AuthService,
    public api: ApiService,
    public logger: LoggerService
    // ... 10以上の依存関係
  ) {}
}
```

### Good

ハブを特定の焦点を持ったコーディネーターに分割するか、コンシューマーレベルで依存性注入を使用して中央ハブを回避します。

```typescript
// auth-coordinator.ts (認証関連の調整に特化)
import { AuthService } from './auth';
import { SessionStore } from './session';

export class AuthCoordinator {
  constructor(
    private auth: AuthService,
    private session: SessionStore
  ) {}
}
```

## 設定

```yaml
rules:
  hub_module:
    severity: medium
    min_fan_in: 5
    min_fan_out: 5
    max_complexity: 5
```

## ESLint ルール

このディテクターは、エディター内でリアルタイムのフィードバックを提供する ESLint ルールとして利用可能です。

```javascript
// eslint.config.js
export default [
  {
    rules: {
      '@archlinter/no-hub-modules': 'warn',
    },
  },
];
```

セットアップ手順については [ESLint Integration](/ja/integrations/eslint) を参照してください。

## 修正方法

ハブを解消しましょう！ハブを通過するデータや制御のさまざまなパスを特定し、それらを別々の、より焦点の絞られたモジュールに抽出します。

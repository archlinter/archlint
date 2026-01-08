# ハブモジュール

**ID:** `hub_module` | **重要度:** Medium (default)

「ハブモジュール」とは、依存関係グラフの中心点であり、高いファンイン（多くの依存元）と高いファンアウト（多くの依存先）の両方を持ちながら、内部ロジックが比較的少ないモジュールのことです。

## なぜこれが「不吉な臭い」なのか

ハブモジュールは、アーキテクチャにおける危険な「単一障害点」です。多くのパスの中心に位置するため、非常に脆弱になります。ハブモジュールへの小さな変更が、コードベース全体に大規模な波及効果を引き起こす可能性があり、リファクタリングが困難でリスクが高くなります。

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
    severity: warn
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

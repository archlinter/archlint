# 未使用のコード (Dead Code)

**ID:** `dead_code` | **重要度:** 低 (デフォルト)

デッドコード（死んだコード）とは、文字通り、プロジェクト内のどこからも使われていないのに「生きて」存在している関数やクラス、変数のことです。

## なぜこれが「不吉な臭い」なのか

- **メンタルエネルギーの無駄遣い**: 開発者は、実行すらされていないコードのリファクタリングや理解に時間を費やすべきではありません。
- **偽の複雑さ**: モジュールのAPIが実際よりも大きく、難解に見えてしまいます。
- **コードの「幽霊」**: デバッグ中に「これ、消したはずじゃなかったっけ？」という混乱を招く原因になります。

## 例 (Examples)

### Bad

```typescript
// utils.ts
export const usedHelper = () => { ... };
export const unusedHelper = () => { ... }; // Reported as dead code

// main.ts
import { usedHelper } from './utils';
```

## 修正方法

1. **削除する**: 本当に使われていないのであれば、削除するのが最善です。
2. **エントリポイントとしてマークする**: パブリック API の一部であったり、動적インポートであったりする場合は、設定の `entry_points` に追加してください。

## 設定 (Configuration)

```yaml
# ルール固有のオプション
rules:
  dead_code:
    exclude:
      - '**/tests/**'
      - '**/temp/**'

# グローバルオプション (ルートレベル)
entry_points:
  - 'src/index.ts'
  - 'src/api/**/*.ts'
```

### オプション

#### ルールオプション (`rules.dead_code`)

- `exclude`: 未使用コードの検出時に無視する glob パターンのリスト。これらのパターンに一致するファイルは、インポート依存関係の分析において存在しないものとして扱われます。

#### グローバルオプション (ルートレベル)

- `entry_points`: 未使用のコードとして報告されないグローバルなエントリポイント。

## ESLint ルール

このディテクターは、エディター内でリアルタイムのフィードバックを提供する ESLint ルールとして利用可能です。

```javascript
// eslint.config.js
export default [
  {
    rules: {
      '@archlinter/no-dead-code': 'warn',
    },
  },
];
```

セットアップ手順については [ESLint Integration](/ja/integrations/eslint) を参照してください。

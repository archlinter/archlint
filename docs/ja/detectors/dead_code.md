---
title: 未使用のコード
description: "プロジェクト内のどこでも使用されていないエクスポートされた関数、クラス、または変数を検出し、保守の負担と混乱を減らします。"
---

# 未使用のコード (Dead Code)

**ID:** `dead_code` | **重要度:** 低 (デフォルト)

未使用のコード（デッドコード）とは、プロジェクト内のどこからもインポートや使用がされていない、エクスポートされた関数、クラス、または変数のことを指します。

## なぜこれが「不吉な臭い」なのか

- **メンテナンスの負担**: 実際には使用されていないコードの更新やリファクタリングに、開発者が時間を費やす可能性があります。
- **バンドルサイズ**: 最終的なアプリケーションサイズを増加させます（多くのバンドラーはツリーシェイキングを行いますが）。
- **混乱**: モジュールの API が実際よりも大きく、複雑に見えるようになります。

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

# 未使用のシンボル (Dead Symbols)

**ID:** `dead_symbols` | **重要度:** 低 (デフォルト)

ファイル内で定義されているものの、ローカルでも一度も使用されていない関数、変数、またはクラスを特定します。

## なぜこれが「不吉な臭い」なのか

単なるノイズです。価値を付加することなく、ファイルの可読性とメンテナンス性を低下させます。

## 修正方法

未使用のシンボルを削除してください。

## 設定 (Configuration)

```yaml
rules:
  dead_symbols:
    severity: low
    # 無視するメソッド名のリスト（フレームワークのライフサイクルメソッドなど）
    ignore_methods:
      - 'constructor'
    # 実装時に無視するインターフェース/クラスメソッドのマップ
    contract_methods:
      MyInterface: ['method1', 'method2']
      ValidatorConstraintInterface: ['validate']
```

## ESLint ルール

このディテクターは、エディター内でリアルタイムのフィードバックを提供する ESLint ルールとして利用可能です。

```javascript
// eslint.config.js
export default [
  {
    rules: {
      '@archlinter/no-dead-symbols': 'warn',
    },
  },
];
```

セットアップ手順については [ESLint Integration](/ja/integrations/eslint) を参照してください。

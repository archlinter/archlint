# 深いネスト

**ID:** `deep_nesting` | **重要度:** Low (default)

深い階層までネストされたコードブロック（if、for、whileなど）を特定します。あまりに深いと、コードがピラミッドや階段のように見えてきます。

## なぜこれが「不吉な臭い」なのか

深いネストがあるコードを読むのは、（括弧の（中に（さらに括弧が）））ある文章を読むようなものです。脳に非常に大きな負担がかかり、その関数がいっぺんに多くのエッジケースを処理しようとしているサインでもあります。「ガード句」を使って早めにリターンするか、ロジックを別の関数に切り出すのが賢明です。

## 修正方法

- **ガード句（Guard Clauses）**: 早期リターンを使用して `else` ブロックを避け、ネストを減らします。
- **関数の抽出（Extract Function）**: 内部のネストされたブロックを新しい関数に移動します。
- **ロジックの平坦化（Flatten Logic）**: ロジックを再評価し、より単純に表現できないか検討します。

## 設定

```yaml
rules:
  deep_nesting:
    severity: low
    max_depth: 4
```

## ESLint ルール

このディテクターは、エディター内でリアルタイムのフィードバックを提供する ESLint ルールとして利用可能です。

```javascript
// eslint.config.js
export default [
  {
    rules: {
      '@archlinter/no-deep-nesting': 'warn',
    },
  },
];
```

セットアップ手順については [ESLint Integration](/ja/integrations/eslint) を参照してください。

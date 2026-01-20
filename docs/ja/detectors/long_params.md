# 長すぎる引数リスト

**ID:** `long_params` | **重要度:** Low (default)

一度に大量の情報を要求する関数を特定します。

## なぜこれが「不吉な臭い」なのか

10個のパラメーターを持つ関数は、呼び出すのも混乱しますが、読むのはさらに混乱します。3番目の引数は `userId` でしたか、それとも `orderId` でしたか？引数のリストが長い場合、それは関数がやりすぎているか、それらのパラメーターが1つのオブジェクトにまとまるべきであるサインです。

## 修正方法

- **Introduce Parameter Object (引数オブジェクトの導入)**: 関連する引数を1つのオブジェクトまたはインターフェースにまとめます。
- **Decompose Function (関数の分解)**: 関数を、より少ない引数で済む小さな関数に分割します。

## 設定

```yaml
rules:
  long_params:
    severity: low
    max_params: 5
    ignore_constructors: true
```

## ESLint ルール

このディテクターは、エディター内でリアルタイムのフィードバックを提供する ESLint ルールとして利用可能です。

```javascript
// eslint.config.js
export default [
  {
    rules: {
      '@archlinter/no-long-params': 'warn',
    },
  },
];
```

セットアップ手順については [ESLint Integration](/ja/integrations/eslint) を参照してください。

---
title: 深いネスト
description: "ネストが深すぎるコードブロックを特定し、コードの読みやすさを指数関数的に難しくし、関数が多すぎることを示します。"
---

# 深いネスト

**ID:** `deep_nesting` | **重要度:** Low (default)

ネストが深すぎるコードブロック（if、for、while など）を特定します。

## なぜこれが「不吉な臭い」なのか

深くネストされたコードは、指数関数的に読み取りや理解が困難になります。これは多くの場合、関数が多くのことを行いすぎているか、ロジックを簡素化できることを示しています。

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

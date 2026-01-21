---
title: 長すぎる引数リスト
description: "パラメータが多すぎて使用や読み取りが困難な関数を検出し、関数が多すぎることを示します。"
---

# 長すぎる引数リスト

**ID:** `long_params` | **重要度:** Low (default)

引数が多すぎる関数やメソッドを特定します。

## なぜこれが「不吉な臭い」なのか

引数が多い関数は、使いにくく、読みにくいものです。多くの場合、関数が多くのことを行いすぎているか、一部の引数をオブジェクトにまとめるべきであることを示しています。

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

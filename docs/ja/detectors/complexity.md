# 高い複雑度 (High Complexity)

**ID:** `complexity` | **重要度:** 中 (デフォルト)

このディテクターは、循環的複雑度（Cyclomatic Complexity）が高い関数を特定します。

## なぜこれが「不吉な臭い」なのか

- **理解が困難**: 分岐が多すぎると、コードを追うのが難しくなります。
- **バグの温床**: テスト中にエッジケースを見落とす可能性が高まります。
- **メンテナンスの悪夢**: 複雑なロジックのため、小さな変更が予期せぬ影響を及ぼす可能性があります。

## 修正方法

1. **メソッドの抽出 (Extract Method)**: 複雑なロジックを、名前の付いた小さな関数に分割してください。
2. **ガード節 (Guard Clauses)**: 早期リターンを使用して、ネストの深さを下げてください。
3. **条件分岐のポリモーフィズムによる置き換え (Replace Conditional with Polymorphism)**: 巨大な `switch` や `if/else` ブロックの代わりに、オブジェクトやストラテジーパターンを使用してください。

## 設定 (Configuration)

```yaml
rules:
  complexity:
    severity: warn
    max_complexity: 15
```

## ESLint ルール

このディテクターは、エディター内でリアルタイムのフィードバックを提供する ESLint ルールとして利用可能です。

```javascript
// eslint.config.js
export default [
  {
    rules: {
      '@archlinter/no-high-complexity': 'warn',
    },
  },
];
```

セットアップ手順については [ESLint Integration](/ja/integrations/eslint) を参照してください。

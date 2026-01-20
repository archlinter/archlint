# 循環的複雑度 (Cyclomatic Complexity)

**ID:** `cyclomatic_complexity` | **重要度:** 中 (デフォルト)

循環的複雑度は、コードの実行パスがどれだけ存在するかを示す指標です。コードがいかに「スパゲッティ状態」になっているかを表す数値だと考えてください。

## なぜこれが「不吉な臭い」なのか

- **頭脳の迷路**: `if` や `else`、`switch` の分岐が増えるたびに、コードの迷路に新しい曲がり角が追加されます。一つの関数に20ものパスがあると、いつか開発者はそこで迷子になってしまうでしょう。
- **テストの限界**: 複雑な関数を完璧にテストするには、考えられるすべてのパスに対してテストケースを用意する必要があります。現実的には、テストされないまま放置される分岐が出てきてしまいます。
- **バタフライ効果**: 複雑すぎる関数では、たった1行の変更が、数段階先の分岐で予期せぬ不具合を引き起こす可能性があります。

## 修正方法

1. **メソッドの抽出 (Extract Method)**: 複雑なロジックを、名前の付いた小さな関数に分割してください。
2. **ガード節 (Guard Clauses)**: 早期リターンを使用して、ネストの深さを下げてください。
3. **条件分岐のポリモーフィズムによる置き換え (Replace Conditional with Polymorphism)**: 巨大な `switch` や `if/else` ブロックの代わりに、オブジェクトやストラテジーパターンを使用してください。

## 設定 (Configuration)

```yaml
rules:
  cyclomatic_complexity:
    severity: medium
    max_complexity: 15
```

## ESLint ルール

このディテクターは、エディター内でリアルタイムのフィードバックを提供する ESLint ルールとして利用可能です。

```javascript
// eslint.config.js
export default [
  {
    rules: {
      '@archlinter/no-high-cyclomatic-complexity': 'warn',
    },
  },
];
```

セットアップ手順については [ESLint Integration](/ja/integrations/eslint) を参照してください。

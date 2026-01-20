# レイヤー違反

**ID:** `layer_violation` | **重要度:** High (default)

レイヤー違反は、せっかくの「クリーンアーキテクチャ」に穴が開いてしまった状態です。上位レベルのビジネスロジック（Domain）が、データベースのテーブルやAPIエンドポイント（Infrastructure）といった詳細な実装について関心を持ち始めてしまうことを指します。

## なぜこれが「不吉な臭い」なのか

- **抽象化の漏洩**: ビジネスロジックは、データベースがPostgresなのか単なるJSONファイルなのかを知る必要はありません。レイヤーが漏れ出すと、その独立性が失われます。
- **壊れやすいテスト**: 単純なビジネスルールをテストしたいだけなのに、データベースのモックを用意しなければならないような状況は不自然です。
- **変更の摩擦**: ロギングライブラリを変更したいだけなのに、ドメインのコア部分に直接インポートされているために、プロジェクト全体をリファクタリングしなければならなくなります。

## 設定

`.archlint.yaml` でレイヤーを定義する必要があります。

```yaml
rules:
  layer_violation:
    layers:
      - name: domain
        path: ['**/domain/**']
        allowed_imports: [] # Domain は何もインポートしません

      - name: application
        path: ['**/application/**']
        allowed_imports: ['domain']

      - name: infrastructure
        path: ['**/infrastructure/**']
        allowed_imports: ['domain', 'application']
```

## ESLint ルール

このディテクターは、エディター内でリアルタイムのフィードバックを提供する ESLint ルールとして利用可能です。

```javascript
// eslint.config.js
export default [
  {
    rules: {
      '@archlinter/no-layer-violations': 'error',
    },
  },
];
```

セットアップ手順については [ESLint Integration](/ja/integrations/eslint) を参照してください。

## 修正方法

1. **依存関係逆転（Dependency Inversion）**: 上位レイヤー（Domain）でインターフェースを定義し、下位レイヤー（Infrastructure）でそれを実装します。
2. **リファクタリング**: 誤って配置されたコードを適切なレイヤーに移動します。

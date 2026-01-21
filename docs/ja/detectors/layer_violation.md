---
title: レイヤー違反
description: "1つのアーキテクチャレイヤーのコードが別のレイヤーからコードを誤ってインポートしている場合を検出し、抽象化と単一責任の原則を破ります。"
---

# レイヤー違反

**ID:** `layer_violation` | **重要度:** High (default)

レイヤー違反（Layer violation）は、あるアーキテクチャレイヤーのコードが、知るべきではないレイヤーのコードをインポートした場合に発生します（例：Domain レイヤーが Infrastructure レイヤーをインポートする）。

## なぜこれが「不吉な臭い」なのか

- **抽象化の破壊**: 内部の実装詳細が、高レベルのビジネスロジックに漏れ出します。
- **テストの困難さ**: インフラストラクチャ（DB、API など）のモックなしでは、ビジネスロジックのテストが困難になります。
- **硬直性**: データベースや外部ライブラリを変更する際に、コアなビジネスロジックの変更が必要になります。

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

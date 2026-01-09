# フレームワークプリセット

archlintはYAMLベースのプリセットを使用して、フレームワーク固有のパターンを理解し、偽陽性（False Positives）を削減します。

## 仕組み

archlintは、`package.json`の依存関係と設定ファイルを分析することでフレームワークを自動的に検出します。また、`.archlint.yaml`で明示的にプリセットを拡張することもできます。

```yaml
extends:
  - nestjs
  - ./my-company-preset.yaml
```

## 組み込みプリセット

- **nestjs**: NestJSアプリケーション用。
- **nextjs**: Next.jsプロジェクト用。
- **react**: Reactライブラリおよびアプリケーション用。
- **oclif**: oclifで構築されたCLIツール用。

## カスタムプリセット

プリセットファイルは、自動検出のための追加の`detect`セクションを含む標準のarchlint設定ファイルです。

### 構造

```yaml
name: my-framework
version: 1

# 自動検出ルール
detect:
  packages:
    any_of: ['my-core-pkg']
  files:
    any_of: ['my-framework.config.js']

# グローバルルール
rules:
  layer_violation: error
  dead_symbols:
    ignore_methods: ['onInit', 'onDestroy']
  vendor_coupling:
    ignore_packages: ['my-framework/*']

# パス固有のオーバーライド
overrides:
  - files: ['**/*.controller.ts']
    rules:
      lcom: off

# デッドコード分析のパターン
entry_points:
  - '**/*.controller.ts'
```

### カスタムプリセットの読み込み

ローカルファイルまたはURLからプリセットを読み込むことができます。

```yaml
extends:
  - ./presets/shared.yaml
  - https://raw.githubusercontent.com/org/archlint-presets/main/standard.yaml
```

## マージロジック

プリセットは指定された順序でマージされます。優先順位は次のとおりです。

1. `.archlint.yaml`のユーザー設定（最優先）
2. `extends`リスト内のプリセット
3. 自動検出されたプリセット
4. archlintのデフォルト設定（最低優先）

リストベースの設定（`entry_points`やルール内の`ignore_packages`など）の場合、archlintはすべての値の和集合を実行します。ルールとオーバーライドは再帰的にマージされます。

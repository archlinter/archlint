# oclif サポート

archlint は [oclif](https://oclif.io/) (Open CLI Framework) の組み込みサポートを提供します。

## 特徴

- **CLI エントリポイント**: コマンドファイルをエントリポイントとして自動的に認識します。
- **フック検出**: oclif フックを識別し、デッドコード分析での誤検知を防ぎます。
- **アーキテクチャルール**: oclif 推奨のディレクトリ構造に従ったプリセットを提供します。

## 設定

oclif サポートを有効にするには、`extends` リストに追加します：

```yaml
extends:
  - oclif
```

## 検出ロジック

以下の条件で oclif プリセットが自動的に検出されます：

1. `package.json` の依存関係に `@oclif/core` が含まれている。
2. プロジェクトに `oclif.manifest.json` ファイルが存在する。

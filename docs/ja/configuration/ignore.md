# 無視パターン（Ignore Patterns）

archlintは、分析からファイルやディレクトリを除外するためのいくつかの方法を提供します。

## グローバルな無視（Global Ignore）

`archlint.yaml`の`ignore`セクションでは、すべての検出器によって完全にスキップされるファイルを指定します。

```yaml
ignore:
  - '**/node_modules/**'
  - '**/dist/**'
  - '**/coverage/**'
  - '**/tmp/**'
  - '**/*.d.ts'
```

## .gitignoreのサポート

デフォルトでは、archlintは自動的に`.gitignore`ファイルを尊重します。これらのパターンを`archlint.yaml`に重複して記述する必要はありません。

## 検出器固有の無視（Detector-Specific Ignore）

一部の検出器には、`thresholds`セクション内に独自の`exclude_patterns`があります。これは、あるファイルをほとんどの検出器で分析したいが、特定の検出器（例：サイクル検出からテストファイルを除外するなど）ではスキップしたい場合に便利です。

```yaml
thresholds:
  cycles:
    exclude_patterns:
      - '**/*.test.ts'
      - '**/*.spec.ts'
```

## インライン無視（Inline Ignores）

（近日公開予定）ソースコード内で直接、特定の行やファイルを無視するための`// archlint-disable`のようなインラインコメントのサポートに取り組んでいます。

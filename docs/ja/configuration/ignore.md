# ファイルの無視

archlint は、解析からファイルやディレクトリを除外するためのいくつかの方法を提供します。

## グローバルな無視

`.archlint.yaml` のルートにある `ignore` セクションは、すべての検出器によって完全にスキップされるファイルを指定します。

```yaml
ignore:
  - '**/node_modules/**'
  - '**/dist/**'
  - '**/coverage/**'
  - '**/tmp/**'
  - '**/*.d.ts'
```

## .gitignore のサポート

デフォルトでは、archlint は `.gitignore` ファイルを自動的に尊重します。`.archlint.yaml` でこれらのパターンを重複させる必要はありません。この動作を無効にするには、`enable_git: false` を設定してください。

## ルールごとの無視

`rules` セクション内の `exclude` フィールドを使用して、特定の検出器からファイルを除外できます。これは、ファイルをほとんどの検出器で解析したいが、特定の 1 つの検出器だけはスキップしたい場合に便利です。

```yaml
rules:
  cycles:
    exclude:
      - '**/generated/**'
      - '**/*.entity.ts'
```

## パスのオーバーライド (Overrides)

より複雑なロジック (例: 特定のディレクトリに対して設定を変更したり、複数のルールを無効にしたりする場合) には、`overrides` セクションを使用します。

```yaml
overrides:
  - files: ['**/tests/**', '**/mocks/**']
    rules:
      complexity: off
      god_module: off
      large_file: warn
```

## インラインでの無視

(開発中) コード内で直接特定の行やファイルを無視するための `// archlint-disable` のようなコメントのサポートに取り組んでいます。

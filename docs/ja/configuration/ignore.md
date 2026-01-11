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

デフォルトでは、archlint は `.gitignore` ファイルを自動的に尊重します。`.archlint.yaml` でこれらのパターンを重複させる必要はありません。この動作を無効にするには、`git: { enabled: false }` を設定してください。

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
      large_file: medium
```

## インラインでの無視

特別なコメントを使用して、ソースコード内で直接特定のアーキテクチャ上の問題を無視できます。これは、例外的なケースで警告を抑制するのに役立ちます。

### 使用方法:

1. **ファイル全体**: ファイルの先頭に `// archlint-disable` を追加します。
2. **現在の行**: 行の末尾、またはその上の行に `// archlint-disable-line` を追加します。
3. **次の行**: 問題のある行の前に `// archlint-disable-next-line` を使用します。
4. **ブロック**: `// archlint-disable` と `// archlint-enable` を使用してコードのセクションを囲みます。

### 例:

```typescript
// archlint-disable-next-line complexity
function veryComplexFunction() {
  // この関数に対して複雑度検出は無視されます
}

import { internal } from './private'; // archlint-disable-line layer_violation

// archlint-disable cycles, god_module
// ファイル全体で特定のルールを無視する

/* archlint-disable complexity */
function legacyCode() {
  // このブロックは無視されます
}
/* archlint-enable complexity */
```

カンマで区切って複数のルールを指定したり、`*` を使用してすべてのルールを無視したりできます。

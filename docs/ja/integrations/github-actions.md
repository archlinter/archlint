# GitHub Actions

archlintをGitHubワークフローに統合して、美しいコメントとアノテーション（annotations）により、すべてのプルリクエスト（Pull Request）でアーキテクチャの回帰（regressions）を防ぎます。

## archlint Action

GitHubでarchlintを使用する最も簡単な方法は、公式の [GitHub Action](https://github.com/marketplace/actions/archlint) を使用することです。

### 主な機能

- **PRコメント**: プルリクエストに詳細なレポートを自動的に投稿します。
- **インラインアノテーション**: アーキテクチャの回帰を、その原因となったコード行に直接表示します。
- **自動サマリー**: ジョブの実行ページにサマリーレポートを追加します。

### ワークフローの例

`.github/workflows/architecture.yml` を作成します：

<div v-pre>

```yaml
name: Architecture

on: [pull_request]

jobs:
  archlint:
    runs-on: ubuntu-latest
    permissions:
      contents: read
      pull-requests: write # PRコメントに必要
    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0 # git diff分析のために重要

      - name: archlint
        uses: archlinter/action@v1
        with:
          baseline: origin/${{ github.base_ref }}
          fail-on: medium
          github-token: ${{ github.token }}
```

</div>

## 入力パラメータ

<div v-pre>

| 入力                | 説明                                                                     | デフォルト            |
| ------------------- | ------------------------------------------------------------------------ | --------------------- |
| `baseline`          | 比較対象となるGitリファレンスまたはスナップショットファイル              | `origin/main`         |
| `fail-on`           | 失敗とする最小重要度（`low`, `medium`, `high`, `critical`）              | `medium`              |
| `comment`           | アーキテクチャレポートを含むPRコメントを投稿する                         | `true`                |
| `annotations`       | アーキテクチャ上の不吉なにおいに対してインラインアノテーションを表示する | `true`                |
| `working-directory` | 分析対象のディレクトリ                                                   | `.`                   |
| `github-token`      | コメント投稿用のGitHubトークン                                           | `${{ github.token }}` |

</div>

## 手動でのCLI使用

CLIを手動で実行したい場合は、`npx @archlinter/cli` を使用できます：

<div v-pre>

```yaml
- name: Check for architectural regressions
  run: npx @archlinter/cli diff origin/${{ github.base_ref }} --fail-on medium --explain
```

</div>

### CLI CIフラグ

- `--fail-on <severity>`: このレベル以上の回帰が見つかった場合、終了コード1で終了します。
- `--explain`: 不吉なにおいがなぜ悪いのか、そしてどのように修正すればよいかについての詳細なアドバイスを表示します。
- `--json`: カスタム処理のために結果をJSON形式で出力します。

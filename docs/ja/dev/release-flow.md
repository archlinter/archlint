# リリースフロー

このドキュメントでは、archlintのリリースプロセスについて説明します。

## 概要

archlintは、リリースワークフロー全体を自動化するために **semantic-release** を使用しています。バージョン番号は、Conventional Commits形式に従ったコミットメッセージに基づいて計算されます。

## コミットメッセージの形式

すべてのコミットは、Conventional Commits形式に**従わなければなりません**。これは、CIにおいてcommitlintによって強制されます。

### 形式

```
<type>(<scope>): <subject>

<body>

<footer>
```

### タイプ（Types）

| タイプ     | 説明                     | バージョンの更新  |
| ---------- | ------------------------ | ----------------- |
| `feat`     | 新機能                   | **Minor** (0.x.0) |
| `fix`      | バグ修正                 | **Patch** (0.0.x) |
| `perf`     | パフォーマンスの向上     | **Patch** (0.0.x) |
| `refactor` | コードのリファクタリング | なし              |
| `docs`     | ドキュメント             | なし              |
| `test`     | テスト                   | なし              |
| `chore`    | メンテナンス             | なし              |
| `ci`       | CI/CDの変更              | なし              |
| `build`    | ビルドシステム           | なし              |

### 破壊的変更（Breaking Changes）

タイプの後に `!` を追加するか、フッターに `BREAKING CHANGE:` と記述することで、**Major** バージョンの更新をトリガーします：

```bash
# メジャーバージョンの更新 (1.0.0)
git commit -m "feat!: APIのシグネチャを変更"

# または
git commit -m "feat: 新機能を追加

BREAKING CHANGE: 公開APIが変更されます"
```

## リリースプロセス

### 1. 開発

機能ブランチで機能を開発し、`main` にマージします。

### プレリリースブランチ

`.releaserc.json` ファイルには、`beta` および `alpha` チャネルの静的なブランチ構成が含まれています。ただし、**プレリリースブランチはリリースワークフロー中にCIによって動的に構成されます**。ワークフローは、選択されたチャネルと現在のブランチ名に基づいて自動的にブランチ構成を作成するため、`.releaserc.json` の静的なエントリは実際のリリース時には使用されません。

### 2. リリースの実行

リリースの準備ができたら、手動でReleaseワークフローをトリガーします：

1. **Actions** -> **Release** ワークフローに移動します。
2. **Run workflow** をクリックします。
3. （任意）実際に公開せずに何が起こるかを確認するには、`dry_run` を `true` に設定します。

### 3. 自動ステップ

ワークフローは以下の手順を実行します：

1. **バージョンの計算**: `semantic-release` が前回のリリース以降のコミットを分析します。
2. **ファイルの更新**: `Cargo.toml`、`package.json`、および `CHANGELOG.md` を自動的に更新します。
3. **コミットとタグ付け**: リリースのための新しいコミットとGitタグを作成します。
4. **CIのトリガー**: タグのプッシュによりCIワークフローがトリガーされ、すべてのバイナリがビルドされます。
5. **npmへの公開**: CIがすべてのパッケージをnpmレジストリに公開します（タグ時のみ）。
6. **バイナリの添付**: CIがスタンドアロンバイナリをGitHub Releaseにアップロードします。

## バージョン番号

すべてのパッケージは同じバージョンを共有します（統合バージョニング）：

- `@archlinter/cli@0.2.0`
- `@archlinter/cli-darwin-arm64@0.2.0`
- `@archlinter/cli-linux-x64@0.2.0`
- など。

## リリース状況の確認

### ワークフローのステータスを表示

https://github.com/archlinter/archlint/actions

### npmへの公開を確認

```bash
npm view @archlinter/cli
```

### インストールのテスト

```bash
npx @archlinter/cli@latest --version
```

## トラブルシューティング

### commitlintによってコミットが拒否された

**修正方法**: Conventional Commits形式に従ってください：

```bash
git commit --amend -m "feat: 正しいコミットメッセージ"
```

### リリースワークフローが失敗した

以下を確認してください：

1. NPM_TOKEN シークレットが設定されていますか？
2. GH_PAT シークレットが設定されていますか？
3. CIビルドが失敗していませんか？

## 参考資料

- [Conventional Commits](https://www.conventionalcommits.org/)
- [Semantic Versioning](https://semver.org/)
- [semantic-release](https://github.com/semantic-release/semantic-release)

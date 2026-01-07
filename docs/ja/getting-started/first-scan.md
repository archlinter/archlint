# 最初のスキャン

インストールが完了したら、最初のスキャンを実行するのは簡単です。

## 基本スキャンの実行

プロジェクトのルートディレクトリに移動して実行します：

```bash
npx @archlinter/cli scan
```

デフォルトでは、archlintは以下を行います：

1. カレントディレクトリ内のすべてのTypeScriptおよびJavaScriptファイルをスキャンします。
2. `.gitignore`ファイルを尊重します。
3. 28以上のすべての検出器に対してデフォルトのしきい値を使用します。
4. 検出された問題（スメル）のカラーテーブルの概要を出力します。

## スナップショットの保存

「ラチェット（劣化防止）」アプローチを使用するには、まずアーキテクチャの現在の状態をキャプチャする必要があります：

```bash
npx @archlinter/cli snapshot -o .archlint-baseline.json
```

このファイルは、あなたのアーキテクチャの基準（ベースライン）を表します。これをリポジトリにコミットする必要があります。

## 退行（レグレッション）のチェック

開発を進めながら、変更によって新しいアーキテクチャ上の問題が導入されていないかチェックできます：

```bash
npx @archlinter/cli diff .archlint-baseline.json
```

CI環境では、通常、メインブランチと比較します：

```bash
npx @archlinter/cli diff origin/main --fail-on medium
```

## 次のステップ

- [すべての検出器について学ぶ](/ja/detectors/)
- [archlint.yaml を設定する](/ja/configuration/)
- [CI/CD に統合する](/ja/integrations/github-actions)

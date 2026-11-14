---
title: GitLab CI
description: 'GitLabのマージリクエストでアーキテクチャルールを強制し、archlintの検出結果をJUnitテストレポートとして公開します。'
---

# GitLab CI

GitLabのマージリクエスト（Merge Requests）でアーキテクチャルールを強制します。

## `.gitlab-ci.yml` の例

```yaml
architecture_check:
  image: node:20
  stage: test
  script:
    - npx @archlinter/cli diff $CI_MERGE_REQUEST_TARGET_BRANCH_NAME --fail-on medium --explain
  rules:
    - if: $CI_PIPELINE_SOURCE == "merge_request_event"
```

## JUnitテストレポート

`archlint scan --format junit` は XML を stdout に出力します。ファイルへ書き出すのは `--report` を付けたときだけです。`archlint scan --format junit --report archlint-junit.xml` は、GitLab が [`artifacts:reports:junit`](https://docs.gitlab.com/ci/testing/unit_test_reports/) で収集するファイルを書き出します。結果はパイプラインの **Tests** タブとマージリクエストのテスト概要に表示されます。

各検出器は `<testsuite>` になります。各 smell は失敗した `<testcase>` になります。

| 項目                     | 値                                           |
| ------------------------ | -------------------------------------------- |
| スイート名 / `classname` | 検出器 ID。例: `dead_symbols`                |
| `name`                   | 説明と、主なファイルおよび行                 |
| `file` と `line`         | プロジェクトルートからの相対パスでの主な位置 |
| `<failure type>`         | 重要度: `low`、`medium`、`high`、`critical`  |
| 失敗の本文               | 理由、リスク、推奨事項、位置                 |

smell がない scan は、`no architectural smells` という名前の成功したテストケースを 1 件出力します。

```yaml
architecture_report:
  image: node:20
  stage: test
  script:
    - npx @archlinter/cli scan --format junit --report archlint-junit.xml
  artifacts:
    when: always
    paths:
      - archlint-junit.xml
    reports:
      junit: archlint-junit.xml
```

`when: always` はジョブが失敗してもレポートをアップロードします。`paths` は XML をジョブのアーティファクトからダウンロードできるようにします。レポートをマージリクエストだけで実行する場合は、diff ジョブと同じ `rules` ブロックを追加します。

### ジョブの状態

JUnit ファイルはすべての smell を失敗したテストとして記録します。ジョブの成否は `scan` の終了コードに従います。

| 検出結果                                     | 終了コード |
| -------------------------------------------- | ---------- |
| `critical` の smell が 1 件以上              | `2`        |
| `high` の smell があり、`critical` はない    | `1`        |
| `low` または `medium` のみ、もしくは検出なし | `0`        |

low と medium の smell は失敗したテストとして表示され、ジョブは成功のままです。ジョブを失敗させる検出結果だけをレポートに載せるには `--min-severity high` を渡します。

```bash
npx @archlinter/cli scan --format junit --min-severity high --report archlint-junit.xml
```

パイプラインで全体の一覧ではなく新しい回帰を止めたい場合は `diff --fail-on` を使います。

## ベストプラクティス

1. **`diff`を使用する**: 常にターゲットブランチと比較して、新しい問題に焦点を当てます。
2. **早期に失敗させる**: `--fail-on` を使用して、メインブランチに回帰が入り込まないようにします。
3. **説明（Explanations）を確認する**: `--explain` の出力は、開発者がマージリクエスト（MR）から離れることなく回帰を修正する方法を理解するのに役立ちます。
4. **JUnitレポートを公開する**: `--format junit` を `artifacts:reports:junit` で収集し、検出結果をパイプラインの Tests タブとマージリクエストに表示します。

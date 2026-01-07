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

## ベストプラクティス

1. **`diff`を使用する**: 常にターゲットブランチと比較して、新しい問題に焦点を当てます。
2. **早期に失敗させる**: `--fail-on` を使用して、メインブランチに回帰が入り込まないようにします。
3. **説明（Explanations）を確認する**: `--explain` の出力は、開発者がマージリクエスト（MR）から離れることなく回帰を修正する方法を理解するのに役立ちます。

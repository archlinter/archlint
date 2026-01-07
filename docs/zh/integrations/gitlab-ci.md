# GitLab CI

在 GitLab Merge Request 中强制执行架构规则。

## `.gitlab-ci.yml` 示例

```yaml
architecture_check:
  image: node:20
  stage: test
  script:
    - npx @archlinter/cli diff $CI_MERGE_REQUEST_TARGET_BRANCH_NAME --fail-on medium --explain
  rules:
    - if: $CI_PIPELINE_SOURCE == "merge_request_event"
```

## 最佳实践

1. **使用 `diff`**：始终与目标分支进行比较，以专注于新问题。
2. **尽早失败**：使用 `--fail-on` 确保没有任何退化进入主分支。
3. **查看解释**：`--explain` 的输出有助于开发人员在不离开 MR 的情况下理解如何修复退化。

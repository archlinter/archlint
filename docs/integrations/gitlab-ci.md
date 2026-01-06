# GitLab CI

Enforce architectural rules in your GitLab Merge Requests.

## Example `.gitlab-ci.yml`

```yaml
architecture_check:
  image: node:20
  stage: test
  script:
    - npx @archlinter/cli diff $CI_MERGE_REQUEST_TARGET_BRANCH_NAME --fail-on medium --explain
  rules:
    - if: $CI_PIPELINE_SOURCE == "merge_request_event"
```

## Best Practices

1. **Use `diff`**: Always compare against the target branch to focus on new issues.
2. **Fail early**: Use `--fail-on` to ensure no regressions make it to the main branch.
3. **Review Explanations**: The `--explain` output helps developers understand how to fix the regressions without leaving the MR.

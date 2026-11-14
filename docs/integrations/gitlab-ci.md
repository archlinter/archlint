---
title: GitLab CI
description: 'Enforce architectural rules in GitLab Merge Requests, and publish archlint findings as a JUnit test report.'
---

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

## JUnit test reports

`archlint scan --format junit` prints XML to stdout. It writes a file only when you pass `--report`. `archlint scan --format junit --report archlint-junit.xml` writes the file GitLab collects with [`artifacts:reports:junit`](https://docs.gitlab.com/ci/testing/unit_test_reports/). GitLab lists the results on the pipeline **Tests** tab and in the merge request test summary.

Each detector becomes a `<testsuite>`. Each smell becomes a failed `<testcase>`:

| Field                    | Value                                            |
| ------------------------ | ------------------------------------------------ |
| Suite name / `classname` | Detector id, such as `dead_symbols`              |
| `name`                   | Explanation, plus the primary file and line      |
| `file` and `line`        | Primary location, relative to the project root   |
| `<failure type>`         | Severity: `low`, `medium`, `high`, or `critical` |
| Failure body             | Reason, risks, recommendations, and locations    |

A scan with no smells emits one passing test case named `no architectural smells`.

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

`when: always` uploads the report when the job fails. `paths` keeps the XML downloadable from the job artifacts. Add the same `rules` block as the diff job when the report should run only on merge requests.

### Job status

The JUnit file records every smell as a failed test. The job status follows the `scan` exit code:

| Findings                                       | Exit code |
| ---------------------------------------------- | --------- |
| Any `critical` smell                           | `2`       |
| Any `high` smell, and no `critical` smell      | `1`       |
| Only `low` or `medium` smells, or a clean scan | `0`       |

Low and medium smells still show up as failed tests, and the job stays green. Pass `--min-severity high` when the test report should list only findings that also fail the job:

```bash
npx @archlinter/cli scan --format junit --min-severity high --report archlint-junit.xml
```

Use `diff --fail-on` when the pipeline should block new regressions instead of the full inventory.

## Best Practices

1. **Use `diff`**: Always compare against the target branch to focus on new issues.
2. **Fail early**: Use `--fail-on` to ensure no regressions make it to the main branch.
3. **Review Explanations**: The `--explain` output helps developers understand how to fix the regressions without leaving the MR.
4. **Publish a JUnit report**: Collect `--format junit` with `artifacts:reports:junit` so findings show up in the pipeline Tests tab and the merge request.

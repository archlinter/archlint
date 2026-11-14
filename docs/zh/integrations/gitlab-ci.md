---
title: GitLab CI
description: '在 GitLab Merge Request 中强制执行架构规则，并将 archlint 的发现发布为 JUnit 测试报告。'
---

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

## JUnit 测试报告

`archlint scan --format junit` 把 XML 打印到 stdout。只有传入 `--report` 时才会写入文件。`archlint scan --format junit --report archlint-junit.xml` 会写出 GitLab 通过 [`artifacts:reports:junit`](https://docs.gitlab.com/ci/testing/unit_test_reports/) 收集的文件。结果出现在流水线的 **Tests** 标签页和合并请求的测试摘要中。

每个检测器是一个 `<testsuite>`。每个 smell 是一个失败的 `<testcase>`：

| 字段                 | 值                                              |
| -------------------- | ----------------------------------------------- |
| 套件名 / `classname` | 检测器 id，例如 `dead_symbols`                  |
| `name`               | 说明，以及主要文件和行号                        |
| `file` 与 `line`     | 相对于项目根目录的主要位置                      |
| `<failure type>`     | 严重程度：`low`、`medium`、`high` 或 `critical` |
| 失败正文             | 原因、风险、建议和位置                          |

没有 smell 的扫描会生成一个名为 `no architectural smells` 的通过用例。

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

`when: always` 会在作业失败时仍然上传报告。`paths` 让 XML 可以从作业产物中下载。如果报告只应在合并请求上运行，请加上与 diff 作业相同的 `rules` 块。

### 作业状态

JUnit 文件把每个 smell 记为失败的测试。作业状态遵循 `scan` 的退出码：

| 发现                                     | 退出码 |
| ---------------------------------------- | ------ |
| 存在任意 `critical` smell                | `2`    |
| 存在任意 `high` smell，且没有 `critical` | `1`    |
| 只有 `low` 或 `medium` smell，或扫描干净 | `0`    |

low 和 medium 的 smell 仍会显示为失败测试，作业保持成功。若报告只应列出也会导致作业失败的发现，请传入 `--min-severity high`：

```bash
npx @archlinter/cli scan --format junit --min-severity high --report archlint-junit.xml
```

若流水线应阻止新的回归，而不是整个清单，请使用 `diff --fail-on`。

## 最佳实践

1. **使用 `diff`**：始终与目标分支进行比较，以专注于新问题。
2. **尽早失败**：使用 `--fail-on` 确保没有任何退化进入主分支。
3. **查看解释**：`--explain` 的输出有助于开发人员在不离开 MR 的情况下理解如何修复退化。
4. **发布 JUnit 报告**：用 `artifacts:reports:junit` 收集 `--format junit`，使发现出现在流水线 Tests 标签页和合并请求中。

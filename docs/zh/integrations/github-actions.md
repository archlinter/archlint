---
title: GitHub Actions
description: "将 archlint 集成到您的 GitHub 工作流中，通过自动评论和注解防止 Pull Request 中的架构退化。"
---

# GitHub Actions

将 archlint 集成到您的 GitHub 工作流中，通过精美的评论和注解防止每个 Pull Request 中的架构退化。

## archlint Action

在 GitHub 上使用 archlint 最简单的方法是通过我们的官方 [GitHub Action](https://github.com/marketplace/actions/archlint)。

### 特性

- **PR 评论**：自动在您的 PR 中发布详细报告。
- **行内注解**：直接在导致架构退化的代码行上显示退化信息。
- **自动摘要**：在任务执行页面添加摘要报告。

### 工作流示例

创建 `.github/workflows/architecture.yml`：

<div v-pre>

```yaml
name: Architecture

on: [pull_request]

jobs:
  archlint:
    runs-on: ubuntu-latest
    permissions:
      contents: read
      pull-requests: write # PR 评论所需
      security-events: write # 上传 SARIF 所需
    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0 # 对 git diff 分析很重要

      - name: archlint
        uses: archlinter/action@v1
        with:
          baseline: origin/${{ github.base_ref }}
          fail-on: medium
          github-token: ${{ github.token }}
```

</div>

## 输入参数

<div v-pre>

| 输入参数            | 描述                                                         | 默认值                |
| ------------------- | ------------------------------------------------------------ | --------------------- |
| `baseline`          | 要对比的 Git 引用或快照文件                                  | `origin/main`         |
| `fail-on`           | 导致失败的最低严重程度 (`low`, `medium`, `high`, `critical`) | `medium`              |
| `comment`           | 发布包含架构报告的 PR 评论                                   | `true`                |
| `annotations`       | 为架构坏味道显示行内注解                                     | `true`                |
| `working-directory` | 要分析的目录                                                 | `.`                   |
| `github-token`      | 用于发布评论的 GitHub token                                  | `${{ github.token }}` |

</div>

## 手动使用 CLI

如果您更喜欢手动运行 CLI，可以使用 `npx @archlinter/cli`：

<div v-pre>

```yaml
- name: Check for architectural regressions
  run: npx @archlinter/cli diff origin/${{ github.base_ref }} --fail-on medium --explain
```

</div>

### CLI CI 标志

- `--fail-on <severity>`：如果发现此级别或更高级别的退化，则以退出码 1 退出。
- `--explain`：关于为什么该坏味道是不好的以及如何修复它的详细建议。
- `--json`：将结果输出为 JSON，以便进行自定义处理。
- `--format sarif`：以 SARIF 格式输出，用于与 GitHub Code Scanning 集成。

## GitHub Code Scanning 集成

您可以将 archlint 的结果上传到 GitHub Code Scanning，以便在 "Security" 选项卡中查看架构问题，并作为 PR 注解显示。

::: tip 权限说明
上传 SARIF 文件需要 `security-events: write` 权限。
:::

```yaml
jobs:
  archlint:
    runs-on: ubuntu-latest
    permissions:
      contents: read
      security-events: write # 上传 SARIF 所需
    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0

      - name: Scan architecture
        run: npx @archlinter/cli scan --format sarif --report archlint.sarif

      - name: Upload SARIF file
        uses: github/codeql-action/upload-sarif@v3
        with:
          sarif_file: archlint.sarif
```

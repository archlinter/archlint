# GitHub Actions

Integrate archlint into your GitHub workflow to prevent architectural regressions in every Pull Request with beautiful comments and annotations.

## The archlint Action

The easiest way to use archlint on GitHub is via our official [GitHub Action](https://github.com/marketplace/actions/archlint).

### Features

- **PR Comments**: Automatically posts a detailed report in your PR.
- **Inline Annotations**: Shows architectural regressions directly on the lines of code that caused them.
- **Automatic Summary**: Adds a summary report to the job execution page.

### Example Workflow

Create `.github/workflows/architecture.yml`:

<div v-pre>

```yaml
name: Architecture

on: [pull_request]

jobs:
  archlint:
    runs-on: ubuntu-latest
    permissions:
      contents: read
      pull-requests: write # Required for PR comments
    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0 # Important for git diff analysis

      - name: archlint
        uses: archlinter/action@v1
        with:
          baseline: origin/${{ github.base_ref }}
          fail-on: medium
          github-token: ${{ github.token }}
```

</div>

## Inputs

<div v-pre>

| Input               | Description                                                    | Default               |
| ------------------- | -------------------------------------------------------------- | --------------------- |
| `baseline`          | Git ref or snapshot file to compare against                    | `origin/main`         |
| `fail-on`           | Minimum severity to fail (`low`, `medium`, `high`, `critical`) | `medium`              |
| `comment`           | Post PR comment with architecture report                       | `true`                |
| `annotations`       | Show inline annotations for architectural smells               | `true`                |
| `working-directory` | Directory to analyze                                           | `.`                   |
| `github-token`      | GitHub token for posting comments                              | `${{ github.token }}` |

</div>

## Manual CLI Usage

If you prefer to run the CLI manually, you can use `npx @archlinter/cli`:

<div v-pre>

```yaml
- name: Check for architectural regressions
  run: npx @archlinter/cli diff origin/${{ github.base_ref }} --fail-on medium --explain
```

</div>

### CLI CI Flags

- `--fail-on <severity>`: Exit with 1 if regressions of this level or higher are found.
- `--explain`: Detailed advice on why a smell is bad and how to fix it.
- `--json`: Output result as JSON for custom processing.

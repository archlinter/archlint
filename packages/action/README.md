# @archlinter/action

Official GitHub Action for [archlint](https://archlinter.github.io/archlint/) to prevent architectural regressions in your Pull Requests.

## Features

- **Architectural Gate**: Block PRs that introduce new circular dependencies or other smells.
- **Beautiful PR Comments**: Get a detailed report with "Why" and "How to fix" explanations directly in the PR.
- **Inline Annotations**: See architectural issues directly in the code diff.
- **Ratchet Approach**: Don't fix everything at once—just don't let it get worse.

## Usage

Add this to your `.github/workflows/architecture.yml`:

```yaml
name: Architecture

on: [pull_request]

jobs:
  arch-check:
    runs-on: ubuntu-latest
    permissions:
      contents: read
      pull-requests: write # Required for posting comments
    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0 # Required for git diff

      - name: archlint Architecture Gate
        uses: archlinter/action@v1
        with:
          baseline: origin/${{ github.base_ref }}
          fail-on: medium
          comment: true
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

## Inputs

| Input               | Description                                                    | Default               |
| ------------------- | -------------------------------------------------------------- | --------------------- |
| `baseline`          | Git ref or snapshot file to compare against                    | `origin/main`         |
| `fail-on`           | Minimum severity to fail (`low`, `medium`, `high`, `critical`) | `medium`              |
| `comment`           | Post PR comment with architecture report                       | `true`                |
| `annotations`       | Show inline annotations for architectural smells               | `true`                |
| `working-directory` | Directory to analyze                                           | `.`                   |
| `github-token`      | GitHub token for posting comments and annotations              | `${{ github.token }}` |

## Documentation

For more information, see the [official documentation](https://archlinter.github.io/archlint/integrations/github-actions).

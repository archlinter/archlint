# GitHub Actions

Integrate archlint into your GitHub workflow to prevent architectural regressions in every Pull Request.

## Example Workflow

Create `.github/workflows/architecture.yml`:

```yaml
name: Architecture Check

on:
  pull_request:
    branches: [main]

jobs:
  archlint:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0 # Important: Fetch history for diff analysis

      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: '20'

      - name: Install dependencies
        run: npm install

      - name: Check for architectural regressions
        run: npx @archlinter/cli diff origin/${{ github.base_ref }} --fail-on medium --explain
```

## Key Options for CI

- `--fail-on medium`: Fail the build if any regression of medium, high, or critical severity is found.
- `--explain`: Prints a detailed explanation in the CI logs for any found regressions.
- `--quiet`: Disables progress bars for cleaner logs.

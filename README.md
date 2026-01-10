<p align="center">
  <img src="docs/public/logo.svg" height="128" alt="archlint logo" />
</p>

<h1 align="center">archlint</h1>

<p align="center">
  <strong>We don't fix your architecture. We stop it from getting worse.</strong>
</p>

<p align="center">
  <a href="https://opensource.org/licenses/MIT"><img src="https://img.shields.io/badge/License-MIT-yellow.svg" alt="License: MIT"></a>
  <a href="https://www.npmjs.com/package/@archlinter/cli"><img src="https://img.shields.io/npm/v/@archlinter/cli.svg" alt="npm"></a>
  <a href="https://www.rust-lang.org/"><img src="https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white" alt="Rust"></a>
</p>

---

Your codebase has 47 circular dependencies. Everyone knows. Nobody has time to fix them.

**And that's okay.**

What's _not_ okay is adding the 48th one in your next PR.

```bash
npx -y @archlinter/cli diff HEAD~1 --explain
```

```
🔴 REGRESSION: New cycle detected

  src/orders/service.ts → src/payments/processor.ts → src/orders/service.ts

  Why this is bad:
    Circular dependencies create tight coupling between modules.
    Changes in one module can cause unexpected failures in the other.

  How to fix:
    Extract shared logic into a separate module, or use dependency injection.

Exit code: 1
```

**PR blocked. Architecture protected. Zero onboarding required.**

---

## The Problem

Static analysis tools love to dump 500 issues on you and call it a day.

- SonarQube: _"You have 127 code smells"_ → team ignores it
- ESLint: _"379 warnings"_ → `// eslint-disable-next-line`
- You: _"We'll fix it later"_ → you won't

The result? **Technical debt compounds silently** until refactoring becomes a 6-month project.

## The Solution

Archlint doesn't ask you to fix everything.

It asks you to **stop making it worse**.

<div v-pre>

```yaml
# GitHub Actions
- name: archlint
  uses: archlinter/action@v1
  with:
    baseline: origin/${{ github.base_ref }}
    fail-on: medium
    github-token: ${{ github.token }}
```

</div>

That's it. Your PR now fails only if:

- You **introduce** a new architectural smell
- You **worsen** an existing one

Everything else passes. Your legacy code is safe. Your future code is protected.

---

## Quick Start

```bash
# One command. No config. No server.
npx @archlinter/cli scan

# Save a baseline (e.g., on main branch)
npx @archlinter/cli snapshot -o .archlint-baseline.json

# Check for regressions in PR
npx @archlinter/cli diff .archlint-baseline.json
```

## Why Archlint?

|                   | SonarQube            | ESLint       | Archlint                |
| ----------------- | -------------------- | ------------ | ----------------------- |
| **Focus**         | Code smells          | Syntax/style | Architecture            |
| **Setup**         | Server + DB + tokens | Config files | `npx`                   |
| **Diff mode**     | Issues count         | ❌           | Semantic regressions    |
| **Explains why**  | Generic              | ❌           | Per-smell context       |
| **Blocks PRs on** | New issues           | Rules        | **Architectural drift** |

### What Archlint Catches

**Dependency Issues:**

- Circular dependencies (file & type-level)
- Layer violations (domain → infra? blocked)
- Stable Dependencies Principle violations

**Design Smells:**

- God modules (doing too much)
- Hub modules (everything depends on it)
- Low cohesion classes (LCOM4 metric)

**Code Organization:**

- Dead code & unused exports
- Barrel file abuse
- High coupling

[→ See all 30+ detectors](docs/detectors.md)

---

## CI Integration

### GitHub Actions

The recommended way to use archlint on GitHub is via our official [GitHub Action](https://github.com/marketplace/actions/archlint):

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

### GitLab CI

```yaml
architecture:
  script:
    - npx @archlinter/cli diff origin/$CI_MERGE_REQUEST_TARGET_BRANCH_NAME
  rules:
    - if: $CI_PIPELINE_SOURCE == "merge_request_event"
```

### Pre-commit Hook

```bash
#!/bin/bash
npx @archlinter/cli diff HEAD~1 --fail-on high
```

---

## Configuration

Create `.archlint.yaml` for project-specific rules:

```yaml
rules:
  # Layer architecture enforcement
  layer_violation:
    layers:
      - name: domain
        path: '**/domain/**'
        allowed_imports: []

      - name: application
        path: '**/application/**'
        allowed_imports: ['domain']

  god_module:
    severity: error
    fan_in: 15
    fan_out: 15

  high_coupling:
    max_cbo: 20

# Framework-aware analysis
extends: nestjs # knows controllers are entry points
```

---

## ESLint Plugin

Get architectural feedback directly in your editor:

```bash
npm install --save-dev @archlinter/eslint-plugin
```

```javascript
// eslint.config.js
import archlint from '@archlinter/eslint-plugin';

export default [archlint.configs['flat/recommended']];
```

---

## Performance

- **~200 files in <5s** — Rust + oxc parser
- **Instant re-runs** — content-based caching
- **Watch mode** — real-time feedback during development

---

## Philosophy

1. **Ratchet, don't renovate** — lock current state, prevent degradation
2. **Explain, don't just report** — every regression comes with context
3. **Zero friction** — `npx`, no server, no tokens, no config required
4. **Architecture > syntax** — ESLint handles style, we handle structure

---

## Links

- [Official Documentation](https://archlinter.github.io/archlint/)
- [Getting Started](https://archlinter.github.io/archlint/getting-started/)
- [Available Detectors](https://archlinter.github.io/archlint/detectors/)
- [Configuration Guide](https://archlinter.github.io/archlint/configuration/)
- [CLI Reference](https://archlinter.github.io/archlint/cli/)
- [ESLint Plugin](https://archlinter.github.io/archlint/integrations/eslint)
- [MCP Server](https://archlinter.github.io/archlint/integrations/mcp-server)

---

## License

MIT — use it, fork it, make your architecture better.

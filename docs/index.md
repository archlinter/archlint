---
layout: home
title: Stop architecture degradation
description: Fast, AST-based architecture smell detector for TypeScript/JavaScript projects. Stop architecture degradation with 28+ detectors and blazingly fast analysis.

hero:
  name: 'archlint'
  text: "We don't fix your architecture. We stop it from getting worse."
  tagline: Fast, AST-based architecture smell detector for TypeScript/JavaScript projects.
  image:
    src: /logo.svg
    alt: archlint logo
  actions:
    - theme: brand
      text: Get Started
      link: /getting-started/
    - theme: alt
      text: View on GitHub
      link: https://github.com/archlinter/archlint

features:
  - title: 28+ Detectors
    details: From circular dependencies to God modules and layer violations. Built with Rust and oxc for maximum performance.
  - title: Diff Mode
    details: Philosophy of "ratchet, don't renovate". Lock current state and only fail on new architectural regressions.
  - title: Framework-Aware
    details: Built-in presets for NestJS, Next.js, Express, React, Vue, Angular, and more. Knows about your framework's architectural patterns.
  - title: Blazingly Fast
    details: Analyzes 200+ files in under 5 seconds. Parallel processing and smart content-based caching.
  - title: Actionable Insights
    details: Every report includes severity scores, clear explanations, and refactoring recommendations.
  - title: Integration Ready
    details: ESLint plugin, GitHub Actions, GitLab CI, and even an MCP server for your AI coding assistant.
---

## Why archlint?

Modern codebases grow complex fast. archlint helps you detect architectural problems early before they become technical debt.

```bash
# Catch regressions in your PR
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
```

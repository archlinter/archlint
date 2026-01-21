---
title: First Scan
description: "Learn how to run your first architectural scan with archlint, interpret results, and configure the tool for your project."
---

# First Scan

Once installed, running your first scan is simple.

## Run a Basic Scan

Navigate to your project root and run:

```bash
npx @archlinter/cli scan
```

By default, archlint will:

1. Scan all TypeScript and JavaScript files in the current directory.
2. Respect your `.gitignore` file.
3. Use default rules for all 28+ detectors.
4. Output a colored table summary of the detected smells.

## Save a Snapshot

To use the "Ratchet" approach, you first need to capture the current state of your architecture:

```bash
npx @archlinter/cli snapshot -o .archlint-baseline.json
```

This file represents your architectural baseline. You should commit it to your repository.

## Check for Regressions

Now, as you develop, you can check if your changes have introduced any new architectural issues:

```bash
npx @archlinter/cli diff .archlint-baseline.json
```

In a CI environment, you would typically compare against the main branch:

```bash
npx @archlinter/cli diff origin/main --fail-on medium
```

## What's Next?

- [Learn about all Detectors](/detectors/)
- [Configure .archlint.yaml](/configuration/)
- [Integrate into CI/CD](/integrations/github-actions)

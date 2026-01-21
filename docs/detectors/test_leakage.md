---
title: Test Leakage
description: "Detect production code that imports from test files, which can lead to increased bundle size, security risks, and broken builds."
---

# Test Leakage

**ID:** `test_leakage` | **Severity:** High (default)

Identifies production code that imports from test files or mock files.

## Why this is a smell

Production code should never depend on test code. This can lead to increased bundle size, security risks, and broken builds if test files are excluded from the production build.

## How to fix

- Move the shared logic from the test file to a production-safe location.
- Ensure that your import paths are correct.

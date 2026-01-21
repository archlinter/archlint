---
title: Orphan Types
description: "Find types or interfaces that are defined but never used, adding clutter and increasing cognitive load without benefit."
---

# Orphan Types

**ID:** `orphan_types` | **Severity:** Low (default)

Identifies types or interfaces that are defined but never used as a type for a variable, parameter, or return value.

## Why this is a smell

Like dead code, orphan types add clutter and increase the cognitive load for developers without providing any benefit.

## How to fix

Delete the unused types.

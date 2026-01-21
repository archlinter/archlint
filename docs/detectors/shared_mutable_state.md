---
title: Shared Mutable State
description: "Detect exported mutable variables that create unpredictable behavior and are a common source of hard-to-track bugs."
---

# Shared Mutable State

**ID:** `shared_mutable_state` | **Severity:** Medium (default)

Identifies exported variables that are mutable (e.g., `export let ...` or `export var ...`).

## Why this is a smell

Global or shared mutable state is a common source of bugs that are extremely hard to track down. It makes the behavior of a module unpredictable and dependent on the order of execution.

## How to fix

- **Use Const**: Export only constants.
- **Encapsulate**: Use a class or a function to manage the state and provide controlled access via methods.
- **Use a State Manager**: If the state truly needs to be shared, use a proper state management library (Redux, Zustand, etc.).

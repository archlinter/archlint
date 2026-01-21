---
title: Feature Envy
description: "Detect methods that are more interested in another class's data than their own, indicating a violation of encapsulation."
---

# Feature Envy

**ID:** `feature_envy` | **Severity:** Medium (default)

Feature envy occurs when a method in one class seems more interested in the data of another class than in the data of its own class.

## Why this is a smell

It indicates a violation of encapsulation. The logic is likely in the wrong place.

## How to fix

Move the method (or the part of the method that has envy) to the class whose data it is using.

---
title: Shotgun Surgery
description: "Detect files that frequently change together, indicating that a single requirement change forces many small changes across multiple modules."
---

# Shotgun Surgery

**ID:** `shotgun_surgery` | **Severity:** Medium (default)

Shotgun Surgery occurs when a single change in requirements forces you to make many small changes across many different modules. archlint detects this by analyzing git history to find files that frequently change together (high co-change frequency).

## Why this is a smell

- **High Maintenance Cost**: Every feature or bug fix requires touching multiple parts of the system.
- **Error Prone**: It's easy to miss one of the many required changes, leading to bugs.
- **Poor Encapsulation**: It indicates that a single responsibility is fragmented across the codebase rather than being encapsulated in one place.

## How to fix

- **Consolidate Responsibilities**: Use **Move Method** or **Move Field** to bring the related logic into a single module.
- **Introduce Parameter Object**: If multiple modules require the same set of data, group it into a single object.
- **Replace Data Value with Object**: If you have many modules handling the same primitive data, encapsulate that data and its behavior into a new class.

## Configuration

```yaml
rules:
  shotgun_surgery:
    severity: medium
```

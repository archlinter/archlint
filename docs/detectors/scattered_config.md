---
title: Scattered Configuration
description: "Identify configuration spread across many files instead of being centralized, making it harder to manage and understand."
---

# Scattered Configuration

**ID:** `scattered_config` | **Severity:** Low (default)

Identifies configuration (like environment variable access) that is spread across many different files instead of being centralized.

## Why this is a smell

Centralizing configuration makes it easier to:

- See all configuration options in one place.
- Provide default values.
- Validate configuration at startup.
- Change the source of configuration (e.g., from env vars to a file or a secret manager).

## How to fix

Create a central `config` module that reads and validates all environment variables and exports them as a typed object.

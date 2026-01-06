# Abstractness Violation

**ID:** `abstractness_violation` | **Severity:** Low (default)

Based on Robert C. Martin's "Main Sequence" metrics. It measures the balance between stability (I) and abstractness (A). A module should either be stable and abstract, or unstable and concrete.

## Why this is a smell

Modules that are stable and concrete are in the "Zone of Pain" (hard to change, but others depend on them). Modules that are unstable and abstract are in the "Zone of Uselessness" (no one depends on them, but they are abstract).

## How to fix

Adjust the module's abstractness (e.g., by introducing interfaces) or its stability (by changing who depends on it).

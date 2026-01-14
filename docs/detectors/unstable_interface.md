# Unstable Interface

**ID:** `unstable_interface` | **Severity:** Medium (default)

Identifies modules whose public API (exports) changes frequently according to git history, while many other modules depend on it.

## Why this is a smell

An unstable interface causes a ripple effect. Every time the interface changes, all its dependents might need to be updated, leading to a lot of busywork and potential bugs.

## How to fix

- **Stabilize the API**: Spend more time designing the interface before implementation.
- **Use Versioning**: If possible, support multiple versions of the interface simultaneously during a transition.
- **Narrow the Interface**: Export only what is absolutely necessary.

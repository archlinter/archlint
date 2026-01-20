# Unstable Interface

**ID:** `unstable_interface` | **Severity:** Medium (default)

Identifies modules that are a "moving target"—they change their public API constantly while everyone else is trying to build on top of them.

## Why this is a smell

- **The ripple effect**: Every time you change a public export in an unstable module, you're potentially breaking a dozen other files that depend on it.
- **Busywork**: Developers spend more time fixing imports and adjusting to API changes than actually building features.
- **Frustration**: It’s hard to trust a module that breaks its promises every other week.

## How to fix

- **Stabilize the API**: Spend more time designing the interface before implementation.
- **Use Versioning**: If possible, support multiple versions of the interface simultaneously during a transition.
- **Narrow the Interface**: Export only what is absolutely necessary.

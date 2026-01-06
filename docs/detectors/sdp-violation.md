# Stable Dependencies Principle (SDP)

**ID:** `sdp_violation` | **Severity:** Medium (default)

The Stable Dependencies Principle states that "the dependencies between packages should be in the direction of the stability." In other words, stable (hard to change) modules should not depend on unstable (easy to change) modules.

## Why this is a smell

If a stable module (one that many others depend on) depends on an unstable module, the stable module becomes harder to change because any change in the unstable module might affect it, which in turn affects all its dependants.

## How to fix

Ensure that your core, stable modules don't depend on volatile modules. Use interfaces or abstract classes to decouple them.

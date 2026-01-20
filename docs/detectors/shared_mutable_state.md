# Shared Mutable State

**ID:** `shared_mutable_state` | **Severity:** Medium (default)

Identifies exported variables that are mutable. When you see an `export let` or `export var`, any importing module can mutate it, sometimes in ways that are hard to trace.

## Why this is a smell

- **The "Who changed this?" mystery**: Global mutable state is a classic source of bugs that are often hard to track down. You can spend time debugging why a value is `null`, only to find out a distant module changed it moments earlier.
- **Race conditions**: It can make behavior depend on the exact order of events. If one file imports or mutates the state later than expected, subtle bugs can appear.
- **Unpredictability**: Your modules are no longer predictable; they're all secretly sharing a messy, global scratchpad.

## How to fix

- **Use Const**: Export only constants.
- **Encapsulate**: Use a class or a function to manage the state and provide controlled access via methods.
- **Use a State Manager**: If the state truly needs to be shared, use a proper state management library (Redux, Zustand, etc.).

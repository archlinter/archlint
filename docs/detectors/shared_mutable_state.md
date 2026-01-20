# Shared Mutable State

**ID:** `shared_mutable_state` | **Severity:** Medium (default)

Identifies exported variables that are mutable. It’s when you see an `export let` or `export var` that any file can reach in and change at any time.

## Why this is a smell

- **The "Who changed this?" mystery**: Global mutable state is a classic source of bugs that are nearly impossible to track down. You’ll spend hours in your debugger wondering why a value is `null`, only to find out a module on the other side of your app changed it three seconds ago.
- **Race conditions**: It makes your app’s behavior dependent on the exact order in which things happen. If one file imports your state slightly later, everything breaks.
- **Unpredictability**: Your modules are no longer predictable; they’re all secretly sharing a messy, global scratchpad.

## How to fix

- **Use Const**: Export only constants.
- **Encapsulate**: Use a class or a function to manage the state and provide controlled access via methods.
- **Use a State Manager**: If the state truly needs to be shared, use a proper state management library (Redux, Zustand, etc.).

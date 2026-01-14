# Primitive Obsession

**ID:** `primitive_obsession` | **Severity:** Low (default)

Primitive obsession is the overuse of primitive types (strings, numbers, booleans) to represent domain concepts that could be better represented by a specific type or class (e.g., using a `string` for an email address or a `number` for a currency).

## Why this is a smell

Primitives don't have behavior or validation. By using a domain-specific type, you can encapsulate validation logic and make the code more self-documenting.

## How to fix

Create a class or a type alias (in TS) with validation logic for the domain concept.

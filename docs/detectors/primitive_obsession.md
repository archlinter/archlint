# Primitive Obsession

**ID:** `primitive_obsession` | **Severity:** Low (default)

Primitive obsession is the habit of using simple types (strings, numbers, booleans) to represent complex domain concepts. It’s when you treat an email address, a currency value, and a random string of text as if they were all the same thing.

## Why this is a smell

- **No safety**: A "string" can be anything. It can be an empty string, a SQL injection, or a poem. It doesn't know it's supposed to be a valid email.
- **Scattered validation**: If you use a raw string for an email, you’ll end up writing `if (!email.includes('@'))` in ten different files instead of having one place that knows what an email is.
- **Meaningless code**: Does `function process(a: number, b: number)` tell you anything? Probably not. Does `function process(price: Currency, quantity: Amount)`? Absolutely.

## How to fix

Create a class or a type alias (in TS) with validation logic for the domain concept.

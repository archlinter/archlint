# Dead Symbols

**ID:** `dead_symbols` | **Severity:** Low (default)

Identifies functions, variables, or classes that are defined within a file but never used, even locally.

## Why this is a smell

It's just clutter. It makes the file harder to read and maintain without adding any value.

## How to fix

Delete the unused symbols.

# Orphan Types

**ID:** `orphan_types` | **Severity:** Low (default)

Identifies types or interfaces that are defined in your code but are essentially "ghosts"—nobody is actually using them.

## Why this is a smell

Just like dead code, orphan types are just clutter. They take up space, show up in your IDE's auto-complete, and force other developers to wonder, "Wait, where is this `OrderOptionsV3` interface actually used?". If it’s not used, it shouldn't be there.

## How to fix

Delete the unused types.

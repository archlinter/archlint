# Scattered Module

**ID:** `scattered_module` | **Severity:** Medium (default)

Identifies a "module" (often a folder or a logical grouping) where the internal files are not well-connected to each other, indicating that the module is just a random collection of code.

## Why this is a smell

A module should be cohesive. If its internal parts don't interact with each other, it's likely not a real module and should be broken up or restructured.

## How to fix

Re-evaluate the purpose of the module. Group the code into more cohesive modules or move the unrelated parts to where they are actually used.

# Shotgun Surgery

**ID:** `shotgun_surgery` | **Severity:** Medium (default)

Shotgun surgery occurs when a single change in your requirements requires you to make many small changes to many different modules.

## Why this is a smell

It indicates that responsibilities are spread too thin across the codebase. It makes changes difficult, time-consuming, and error-prone.

## How to fix

- **Move Method/Field**: Consolidate the related logic into a single module.
- **Inline Class**: If a class is just a collection of methods that are always used together with another class, combine them.

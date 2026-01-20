# Shotgun Surgery

**ID:** `shotgun_surgery` | **Severity:** Medium (default)

Shotgun Surgery is that annoying situation where a "simple" change requires you to touch 15 different files. It's like trying to fix a leak by patching a hundred tiny holes instead of replacing the pipe.

## Why this is a smell

- **High friction**: Every small requirement change becomes a major operation.
- **Easy to miss a spot**: When logic is scattered everywhere, it’s almost certain you’ll forget to update one of those files, leading to "ghost bugs".
- **Broken encapsulation**: It’s a sign that a single responsibility has escaped its module and is now hiding in every corner of your codebase.

## How to fix

- **Consolidate Responsibilities**: Use **Move Method** or **Move Field** to bring the related logic into a single module.
- **Introduce Parameter Object**: If multiple modules require the same set of data, group it into a single object.
- **Replace Data Value with Object**: If you have many modules handling the same primitive data, encapsulate that data and its behavior into a new class.

## Configuration

```yaml
rules:
  shotgun_surgery:
    severity: medium
```

# Hub Module

**ID:** `hub_module` | **Severity:** Medium (default)

A "Hub Module" is a module that acts as a central point in the dependency graph, having both high Fan-in and high Fan-out.

## Why this is a smell

Hub modules are dangerous "single points of failure" in your architecture. Because so many things depend on them, and they depend on so many other things, they are extremely fragile and hard to change without causing a ripple effect across the entire codebase.

## How to fix

Break the hub! Identify the different paths of data or control passing through the hub and extract them into separate, more focused modules.

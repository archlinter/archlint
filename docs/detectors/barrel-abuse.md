# Barrel Abuse

**ID:** `barrel_file_abuse` | **Severity:** Medium (default)

Barrel files (e.g., `index.ts` files that only re-export other files) can become problematic when they grow too large or include too many unrelated exports.

## Why this is a smell

- **Circular Dependencies**: Large barrel files are a common cause of indirect circular dependencies.
- **Unnecessary Coupling**: Importing one thing from a large barrel file can cause the bundler to pull in many unrelated modules.
- **Performance**: Can slow down both development (IDE indexing) and production (bundle size/loading time).

## How to fix

- Avoid "catch-all" barrel files at the root of large directories.
- Prefer direct imports if a barrel file is causing issues.
- Group exports into smaller, more specific barrel files.

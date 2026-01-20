# Test Leakage

**ID:** `test_leakage` | **Severity:** High (default)

Identifies production code that is secretly importing from your test files or mocks.

## Why this is a smell

- **Shipping your tests**: You don't want your production bundle to include 5MB of mock data or test utilities. It's embarrassing and slows down your users.
- **Security risks**: Test files often contain sample credentials or simplified logic that should never be exposed in a real environment.
- **Broken builds**: Many build tools exclude test files (e.g., `*.test.ts`, `*.test.js`) during production compilation. If your production code depends on them, your app will simply fail to build or crash at runtime.

## How to fix

- Move the shared logic from the test file to a production-safe location.
- Ensure that your import paths are correct.

# Side-Effect Imports

**ID:** `side_effect_import` | **Severity:** Low (default)

Identifies imports that are performed only for their side effects (e.g., `import './globals';`), which often modify global state or prototypes.

## Why this is a smell

Side-effect imports make the dependency graph less explicit and can lead to non-deterministic behavior depending on the import order. They are often "hidden" dependencies that are hard to track.

## Excluded Patterns

archlint automatically ignores the following side-effect imports:

- **CSS/Assets**: `import './styles.css'`, `import './image.png'`, etc.
- **Dynamic Imports**: `import('./module')` or `require('./module')` inside functions (often used for lazy loading or conditional imports).

## How to fix

Try to make the initialization explicit. Instead of relying on a side-effect import, export an `init()` function and call it explicitly.

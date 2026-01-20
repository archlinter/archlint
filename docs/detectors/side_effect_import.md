# Side-Effect Imports

**ID:** `side_effect_import` | **Severity:** Low (default)

Identifies imports that happen purely for their "magic" side effects—like an import that modifies a global prototype or sets up a global state without actually exporting anything.

## Why this is a smell

- **Hidden surprises**: Side-effect imports make your app’s behavior feel like magic. You look at a file and can't figure out where a specific behavior is coming from until you find a "ghost" import in your `main.ts`.
- **Order matters (too much)**: If you accidentally swap the order of two side-effect imports, your app might behave differently or break entirely.
- **Hard to trace**: Because nothing is explicitly called or returned, these dependencies are invisible to many static analysis tools and even harder for human developers to keep in their heads.

## Excluded Patterns

archlint automatically ignores the following side-effect imports:

- **CSS/Assets**: `import './styles.css'`, `import './image.png'`, etc.
- **Dynamic Imports**: `import('./module')` or `require('./module')` inside functions (often used for lazy loading or conditional imports).

## How to fix

Try to make the initialization explicit. Instead of relying on a side-effect import, export an `init()` function and call it explicitly.

# Framework Support

archlint is not just a generic linter; it understands the architectural patterns of popular frameworks and adjusts its analysis accordingly.

## How it works

archlint automatically detects which frameworks are used in your project by looking at `package.json` and file structures. You can also explicitly set or override this in your `archlint.yaml`:

```yaml
frameworks:
  - nestjs
  - react
```

## Benefits of Framework-Awareness

- **Reduced False Positives**: Some patterns that are smells in general (like high coupling) are necessary and expected in certain framework contexts (like NestJS modules).
- **Smart Entry Points**: Automatically identifies controllers, pages, and hooks as entry points for dead code analysis.
- **Relevant Detectors**: Disables detectors that don't make sense for a specific framework (like LCOM for React components).

## Supported Frameworks

- [NestJS](/frameworks/nestjs)
- [Next.js](/frameworks/nextjs)
- [React](/frameworks/react)

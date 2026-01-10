# Next.js Support

Next.js projects have unique file-based routing and bundling patterns that archlint understands.

## Key Features

- **Routing Aware**: Automatically recognizes files in `pages/` and `app/` directories as entry points.
- **Barrel Files**: Relaxes barrel file rules for common Next.js patterns.
- **Client/Server Components**: (Coming Soon) Specialized analysis for server-only vs client-only code leakage.

## Recommended Configuration

```yaml
extends:
  - nextjs

entry_points:
  - 'src/pages/**/*.tsx'
  - 'src/app/**/*.tsx'
```

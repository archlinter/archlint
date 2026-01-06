# Layer Violation

**ID:** `layer_violation` | **Severity:** High (default)

Layer violation occurs when code in one architectural layer imports code from a layer it shouldn't know about (e.g., Domain layer importing from Infrastructure).

## Why this is a smell

- **Breaks Abstraction**: Internal implementation details leak into high-level business logic.
- **Testing Difficulty**: Business logic becomes hard to test without mocks for infrastructure (DB, API, etc.).
- **Rigidity**: Changing a database or external library requires changing the core business logic.

## Configuration

You must define your layers in `.archlint.yaml`:

```yaml
layers:
  - name: domain
    paths: ['**/domain/**']
    can_import: [] # Domain imports nothing

  - name: application
    paths: ['**/application/**']
    can_import: ['domain']

  - name: infrastructure
    paths: ['**/infrastructure/**']
    can_import: ['domain', 'application']
```

## How to fix

1. **Dependency Inversion**: Define an interface in the higher layer (Domain) and implement it in the lower layer (Infrastructure).
2. **Refactor**: Move the misplaced code to the appropriate layer.

# Vendor Coupling

**ID:** `vendor_coupling` | **Severity:** Medium (default)

Identifies modules that have become "married" to a specific external library or framework.

## Why this is a smell

- **Vendor lock-in**: If that library becomes deprecated or you decide to switch to a better alternative, you'll have to rewrite half your codebase.
- **Testing friction**: You can't test your business logic without also pulling in the heavy external library and its mocks.
- **Hard to upgrade**: You're stuck on whatever version the library supports because it's woven into every file.

## How to fix

Use the **Adapter Pattern**. Create an interface in your domain and implement it using the external library. The rest of your code should only depend on your interface.

## Configuration

```yaml
rules:
  vendor_coupling:
    severity: medium
    max_files_per_package: 10
    ignore_packages:
      - 'lodash'
      - 'rxjs'
      - '@nestjs/*'
```

### Options

- `max_files_per_package` (default: 10): The maximum number of files that can import a specific package before a smell is reported.
- `ignore_packages`: A list of package names or glob patterns to ignore.

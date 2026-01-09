# Vendor Coupling

**ID:** `vendor_coupling` | **Severity:** Medium (default)

Identifies modules that are too tightly coupled to a specific external library or framework.

## Why this is a smell

If you decide to switch the library in the future, you'll have to change code in many places. It also makes testing harder because you have to mock the external library everywhere.

## How to fix

Use the **Adapter Pattern**. Create an interface in your domain and implement it using the external library. The rest of your code should only depend on your interface.

## Configuration

```yaml
rules:
  vendor_coupling:
    severity: warn
    max_files_per_package: 10
    ignore_packages:
      - 'lodash'
      - 'rxjs'
      - '@nestjs/*'
```

### Options

- `max_files_per_package` (default: 10): The maximum number of files that can import a specific package before a smell is reported.
- `ignore_packages`: A list of package names or glob patterns to ignore.

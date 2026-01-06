# Vendor Coupling

**ID:** `vendor_coupling` | **Severity:** Medium (default)

Identifies modules that are too tightly coupled to a specific external library or framework.

## Why this is a smell

If you decide to switch the library in the future, you'll have to change code in many places. It also makes testing harder because you have to mock the external library everywhere.

## How to fix

Use the **Adapter Pattern**. Create an interface in your domain and implement it using the external library. The rest of your code should only depend on your interface.

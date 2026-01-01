# Test Fixtures

This directory contains test fixtures used by integration tests.

## Structure

```
test_data/
├── cycles/           # Circular dependency detection tests
│   ├── no_cycle/     # No circular dependencies
│   └── simple_cycle/ # Simple A→B→A cycle
├── dead_code/        # Dead code detection tests
│   ├── main.ts      # Entry point
│   ├── used.ts      # Used module
│   └── dead.ts      # Unused module
└── god_module/       # God module detection tests
    ├── god.ts       # Module with high fan-in/fan-out
    ├── caller1.ts   # Depends on god.ts
    ├── caller2.ts   # Depends on god.ts
    ├── dep1.ts      # Dependency of god.ts
    └── dep2.ts      # Dependency of god.ts
```

## Usage

Tests reference fixtures using the `analyze_fixture()` helper from `tests/common/mod.rs`:

```rust
let ctx = analyze_fixture("cycles/simple_cycle");
```

All paths are relative to this `test_data` directory.

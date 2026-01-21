---
title: Abstractness Violation
description: "Detect modules that are too concrete and stable (hard to change) or too abstract and unstable (overengineering), violating the Stable Abstractions Principle."
---

# Abstractness Violation

**ID:** `abstractness` | **Severity:** Medium (default)

## What this rule detects (TL;DR)

This rule flags modules that are:

- **Too concrete and too stable** — many files depend on a concrete class (hard to change safely).
- **Too abstract and too unstable** — abstractions that nobody depends on (overengineering/YAGNI).

In short:  
👉 **Stable modules** (the foundation) should be **abstract**.  
👉 **Unstable modules** (the leaves) should be **concrete**.

---

## What is a "Module"?

In `archlint`, a **module** is a single source file (`.ts`, `.tsx`, etc.) that exports at least one symbol.

- Each file is analyzed independently.
- Barrel files (`index.ts`) are treated as aggregation modules.
- Re-exports and imports are counted as dependencies between modules.

---

## Metrics & Intuition

### 1. Instability (I)

Measures how prone a module is to change.

`I = Efferent Coupling (Ce) / (Afferent Coupling (Ca) + Efferent Coupling (Ce))`

**Intuition**:

- **Stable (I ≈ 0)**: Many files import you, but you import almost nobody. You are a foundation component.
- **Unstable (I ≈ 1)**: You import many things, but nobody imports you. You are at the edge of the system.

### 2. Abstractness (A)

We use a **Semantic Calculation** based on real usage:

`A = (Clients importing only Interfaces/Types) / (Total Clients)`

**Important difference from classic A**:
We do **NOT** just count keywords or interfaces inside the file. Instead, we measure **how the module is actually used**:

- Importing a concrete `class` → **concrete dependency**.
- Importing an `interface` or `type` → **abstract dependency**.

This reflects _real architectural coupling_, not just syntax. Using `import type` is a strong signal of abstract intent.

### 3. Distance (D)

The distance from the ideal "Main Sequence" line where `A + I = 1`.

`D = |A + I - 1|`

---

## Risk Zones (Interpretation)

Based on the **A** and **I** values, modules fall into specific zones:

### 🧱 Zone of Pain

- **Metrics**: I ≈ 0–0.3 (stable), A ≈ 0–0.3 (concrete).
- **Problem**: Everyone depends on a concrete implementation. Changing it is dangerous because it's both rigid and highly coupled.

**Bad Example (Concrete dependency):**

```typescript
// database.service.ts
export class DatabaseService {
  save(data: any) {
    /* concrete logic */
  }
}

// client.ts (100+ files doing this)
import { DatabaseService } from './database.service'; // Direct class import
const db = new DatabaseService();
```

**Why it's flagged**:

- `Ca` = 100+ (very stable, `I ≈ 0`).
- `A` = 0 (clients import the class directly).
- `D` ≈ 1 → Maximum distance from the main sequence.

### 💨 Zone of Uselessness

- **Metrics**: I ≈ 0.7–1.0 (unstable), A ≈ 0.7–1.0 (abstract).
- **Problem**: Over-engineered abstractions that nobody uses.

**Example:**

```typescript
// complex-plugin.interface.ts
export interface IComplexPlugin {
  execute(context: any): Promise<void>;
}
// 0 implementations and 0 clients using this interface.
```

**Why it's flagged**:

- `I` ≈ 1 (nobody depends on it).
- `A` = 1 (it's purely abstract).
- `D` ≈ 1 → Abstraction exists without a purpose.

---

## Heuristics to Reduce False Positives

Static analysis can be noisy. These heuristics focus the rule on **architectural decisions**, not incidental code:

1.  **Stability Threshold (Fan-in)**: Only modules with at least `fan_in_threshold` (default: 10) dependents are analyzed. If only a few files use a module, its architectural impact is low.
2.  **DTOs & Entities**: Classes with no methods (data-only) are ignored. They are **data carriers**, not behavioral components.
3.  **Errors**: Classes extending `Error` are ignored. They are **always concrete by design**.
4.  **Infrastructure Scripts**: Database migrations (`up`/`down`) are ignored as they are **procedural scripts**, not part of the long-term architecture.

---

## How to Fix (Decision Guide)

1. **Is the module stable (has many dependents)?**
   - **Yes**: Extract an `interface`. Ensure clients use `import type { ... }`. Use Dependency Injection.
   - **No**: Abstractions might be unnecessary. Keep it concrete until stability increases.

2. **Is there more than one implementation?**
   - **No**: If it's unstable, consider removing the interface (YAGNI).
   - **Yes**: The interface is justified, but ensure clients depend on the interface, not the classes.

---

## Configuration

```yaml
rules:
  abstractness:
    severity: medium
    distance_threshold: 0.85 # Trigger threshold for distance D
    fan_in_threshold: 10 # Minimum incoming dependencies (Fan-in) to trigger analysis
```

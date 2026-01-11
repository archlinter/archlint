# Cyclic Dependencies

**ID:** `cycles` | **Severity:** Critical (default)

Circular dependencies occur when two or more modules depend on each other, either directly or indirectly.

## Why this is a smell

- **Tight Coupling**: Modules are inseparable, making it hard to reuse them independently.
- **Initialization Issues**: Can lead to "undefined" imports at runtime if not handled carefully by the bundler.
- **Testing Difficulty**: Hard to mock or isolate one module without bringing in the entire cycle.
- **Cognitive Load**: Harder for developers to understand the flow of data and control.

## Examples

### Bad

```typescript
// orders.ts
import { processPayment } from './payments';
export const createOrder = () => {
  /* ... */
};

// payments.ts
import { createOrder } from './orders';
export const processPayment = () => {
  /* ... */
};
```

### Good

Extract shared logic into a third module.

```typescript
// types.ts
export interface Order {
  /* ... */
}

// orders.ts
import { Order } from './types';
export const createOrder = () => {
  /* ... */
};

// payments.ts
import { Order } from './types';
export const processPayment = (order: Order) => {
  /* ... */
};
```

## Configuration

```yaml
rules:
  cycles:
    severity: high
    exclude: ['**/*.test.ts']
```

## How to fix

1. **Extract shared logic**: Move the common parts to a new module that both existing modules depend on.
2. **Dependency Injection**: Pass dependencies as arguments instead of importing them.
3. **Use Events**: Use an event bus or callbacks to decouple the modules.

## ESLint Rule

This detector is available as an ESLint rule for real-time feedback in your editor.

```javascript
// eslint.config.js
export default [
  {
    rules: {
      '@archlinter/no-cycles': 'error',
    },
  },
];
```

See [ESLint Integration](/integrations/eslint) for setup instructions.

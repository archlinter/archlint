# Cyclic Dependencies

**ID:** `cyclic_dependency` | **Severity:** Critical (default)

Circular dependencies occur when two or more modules depend on each other, either directly or indirectly. It's the "chicken or the egg" problem of software engineering.

## Why this is a smell

- **Inseparable coupling**: You can't just take one module and use it elsewhere; it brings the whole family of dependencies with it.
- **Initialization traps**: Depending on your bundler, you might end up with "undefined" imports at runtime because the cycle couldn't be resolved in time.
- **Testing nightmare**: Good luck mocking one part of the cycle without the whole thing collapsing like a house of cards.
- **Cognitive overload**: Trying to follow the data flow in a cycle is like reading a "choose your own adventure" book where every page leads back to the start.

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
  cyclic_dependency:
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

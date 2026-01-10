# React Support

React components have different architectural characteristics than traditional classes or modules.

## Key Features

- **Component Recognition**: Identifies React components by naming patterns and JSX usage.
- **LCOM Disabled**: Automatically disables the Low Cohesion (LCOM) detector for components, as they are inherently focused on UI state and rendering.
- **Hook Analysis**: Understands that custom hooks are entry points for shared UI logic.

## Recommended Configuration

```yaml
extends:
  - react
```

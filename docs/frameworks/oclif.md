# oclif Support

archlint provides built-in support for [oclif](https://oclif.io/), the Open CLI Framework.

## Features

- **CLI Entry Points**: Automatically recognizes command files as entry points.
- **Hook Detection**: Identifies oclif hooks to prevent false positives in dead code analysis.
- **Architectural Rules**: Provides presets that follow oclif's recommended directory structure.

## Configuration

To enable oclif support, add it to your `extends` list:

```yaml
extends:
  - oclif
```

## Detection Logic

The oclif preset is automatically detected if:

1. `package.json` contains `@oclif/core` in dependencies.
2. The project has an `oclif.manifest.json` file.

# archlint init

The `init` command helps you quickly set up archlint in a new project by generating a configuration file.

## Usage

```bash
archlint init [options]
```

## Options

| Option             | Default | Description                                                             |
| ------------------ | ------- | ----------------------------------------------------------------------- |
| `-f, --force`      | `false` | Overwrite existing `.archlint.yaml` if it exists                        |
| `--no-interactive` | `false` | Skip interactive framework selection                                    |
| `--presets <list>` | `none`  | Explicitly specify framework presets (comma-separated or repeated flag) |

## How it Works

1. **Framework Detection**: archlint analyzes your `package.json` and project structure to detect used frameworks.
2. **Interactive Selection**: Unless `--no-interactive` is used, it prompts you to confirm or select additional framework presets.
3. **Configuration Generation**: Creates a `.archlint.yaml` file with the selected presets and a reference to the JSON schema for IDE support.

## Examples

### Interactive initialization

```bash
archlint init
```

### Non-interactive with specific presets

```bash
# Comma-separated
archlint init --no-interactive --presets nestjs,prisma

# Or repeated flag
archlint init --no-interactive --presets nestjs --presets prisma
```

### Overwrite existing config

```bash
archlint init --force
```

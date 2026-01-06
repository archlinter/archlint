# archlint watch

The `watch` command runs archlint in the background and re-analyzes your project every time a file changes.

## Usage

```bash
archlint watch [options]
```

## Options

| Option            | Default | Description                               |
| ----------------- | ------- | ----------------------------------------- |
| `--debounce <ms>` | `300`   | Wait for more changes before re-running   |
| `--clear`         | `false` | Clear the terminal screen before each run |

## Examples

### Real-time feedback during development

```bash
archlint watch --clear --debounce 500
```

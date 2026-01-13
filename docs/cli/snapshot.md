# archlint snapshot

The `snapshot` command captures the current state of your project's architecture and saves it to a JSON file. This file can then be used with the `diff` command.

## Usage

```bash
archlint snapshot [options]
```

## Options

| Option                | Default                  | Description                      |
| --------------------- | ------------------------ | -------------------------------- |
| `-o, --output <file>` | `archlint-snapshot.json` | The file to save the snapshot to |
| `-p, --path <path>`   | `.`                      | Project path to analyze          |

## Examples

### Create a baseline for the project

```bash
archlint snapshot -o .archlint-baseline.json
```

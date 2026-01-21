---
title: watch
description: "Run archlint in watch mode to automatically re-analyze your project whenever files change, providing continuous feedback."
---

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
| `-c, --config`    | `none`  | Path to configuration file                |

::: tip
The `watch` command also supports all options from the [`scan`](/cli/scan) command.
:::

## Examples

### Real-time feedback during development

```bash
archlint watch --clear --debounce 500
```

# CLI Reference

The archlint CLI is the primary way to interact with the tool.

## General Usage

```bash
archlint [command] [options]
```

## Commands

| Command                     | Description                                  |
| --------------------------- | -------------------------------------------- |
| [`scan`](/cli/scan)         | Run a one-time architectural analysis        |
| [`diff`](/cli/diff)         | Compare the current state against a baseline |
| [`snapshot`](/cli/snapshot) | Save the current state to a JSON file        |
| [`watch`](/cli/watch)       | Run in watch mode for real-time feedback     |

## Global Options

| Option            | Description                         |
| ----------------- | ----------------------------------- |
| `--config <path>` | Path to the configuration file      |
| `--verbose`       | Enable verbose logging              |
| `--quiet`         | CI-friendly mode (no progress bars) |
| `--version`       | Show version information            |
| `--help`          | Show help for a command             |

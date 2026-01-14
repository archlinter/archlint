---
title: CLI Reference
description: Complete reference for archlint CLI commands, including scan, diff, snapshot, and watch.
---

# CLI Reference

The archlint CLI is the primary way to interact with the tool.

## General Usage

```bash
archlint [command] [options]
```

## Commands

| Command                     | Description                                  |
| --------------------------- | -------------------------------------------- |
| [`init`](/cli/init)         | Initialize a new configuration file          |
| [`scan`](/cli/scan)         | Run a one-time architectural analysis        |
| [`diff`](/cli/diff)         | Compare the current state against a baseline |
| [`snapshot`](/cli/snapshot) | Save the current state to a JSON file        |
| [`watch`](/cli/watch)       | Run in watch mode for real-time feedback     |

## Global Options

| Option                | Description                         |
| --------------------- | ----------------------------------- |
| `-c, --config <path>` | Path to the configuration file      |
| `-v, --verbose`       | Enable verbose logging              |
| `-q, --quiet`         | CI-friendly mode (no progress bars) |
| `-V, --version`       | Show version information            |
| `-h, --help`          | Show help for a command             |

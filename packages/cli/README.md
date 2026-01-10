<p align="center">
  <img src="docs/assets/logo.svg" height="128" alt="archlint logo" />
</p>

<h1 align="center">archlint</h1>

<p align="center">
  <strong>Fast, AST-based architecture smell detector for TypeScript/JavaScript projects</strong>
</p>

<p align="center">
  <a href="https://opensource.org/licenses/MIT"><img src="https://img.shields.io/badge/License-MIT-yellow.svg" alt="License: MIT"></a>
  <a href="https://www.rust-lang.org/"><img src="https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white" alt="Rust"></a>
</p>

A powerful CLI tool for detecting 30+ types of architectural smells in TypeScript/JavaScript codebases. Built with Rust and `oxc` parser for maximum performance.

## Why archlint?

Modern codebases grow complex fast. archlint helps you:

- 🔍 **Detect architectural problems early** before they become technical debt
- 🎯 **Framework-aware analysis** with presets for NestJS, Next.js, React, oclif
- ⚡ **Blazingly fast** - analyzes 200+ files in under 5 seconds
- 📊 **Actionable insights** - clear explanations, severity scores, and refactoring recommendations
- 🔄 **Watch mode** for real-time feedback during development
- 🧠 **Smart caching** for instant re-runs

## Features

### 30+ Architectural Smell Detectors

**Dependency & Structure Issues:**

- Cyclic dependencies (file-level and type-level)
- Hub modules (too many connections)
- God modules (doing too much)
- Dead code (unused exports)
- Orphan types (disconnected from codebase)

**Design Quality Issues:**

- Low cohesion (LCOM metric)
- High coupling
- Layer violations (domain/application/infrastructure)
- SDP violations (Stable Dependencies Principle)
- Feature envy, shotgun surgery

**Code Organization Issues:**

- Barrel file abuse
- Scattered modules/configs
- Primitive obsession
- Long parameter lists
- Deep nesting

**Framework-Specific Issues:**

- Test leakage (test code in production)
- Side-effect imports
- Vendor coupling
- Unstable interfaces
- Shared mutable state

### Framework Intelligence

Built-in presets that understand framework patterns:

- **NestJS**: Knows controllers are entry points, modules can have high coupling
- **Next.js**: Understands pages/API routes, relaxes barrel file rules
- **React**: Aware of component patterns, skips irrelevant checks
- **oclif**: Recognizes CLI commands and hooks

### Output Formats

- **Table** (default) - clean terminal output with colors
- **Markdown** - detailed reports with Mermaid dependency graphs
- **JSON** - machine-readable for CI/CD integration

### Developer Experience

- **Watch mode** with debouncing for live feedback
- **Smart caching** using content hashes
- **Severity filtering** (low/medium/high/critical)
- **Selective detector runs** (include/exclude specific checks)
- **Git integration** for churn analysis
- **CI-friendly** quiet mode

## Installation

### Using npm (Recommended)

```bash
npm install -g @archlinter/cli
# or use without installation
npx @archlinter/cli scan
```

### From source

```bash
git clone https://github.com/superprotocol/archlint.git
cd archlint
cargo build --release
cargo install --path crates/archlint
```

## Quick Start

### Basic scan

```bash
archlint scan ./my-project
```

### With configuration

```bash
archlint scan ./my-project --config .archlint.yaml
```

### Watch mode for development

```bash
archlint watch --debounce 500 --clear
```

### Export report

```bash
# Markdown with diagrams
archlint scan ./my-project --format markdown --report architecture-report.md

# JSON for CI/CD
archlint scan ./my-project --format json --report report.json
# Or use the shorter --json flag
archlint scan ./my-project --json --report report.json
```

### Filter by severity

```bash
archlint scan --min-severity high
```

### Run specific detectors

```bash
archlint scan --detectors cycles,god_module,dead_code
```

### Exclude detectors

```bash
archlint scan --exclude-detectors barrel_file,lcom
```

## Configuration

Create `.archlint.yaml` in your project root:

```yaml
# Files/directories to ignore
ignore:
  - '**/node_modules/**'
  - '**/dist/**'

# Path aliases (from tsconfig.json)
aliases:
  '@/*': 'src/*'

# Entry points (won't be marked as dead code)
entry_points:
  - 'src/main.ts'

# Rule-specific configuration
rules:
  cycles: error

  god_module:
    severity: high
    fan_in: 15
    fan_out: 15
    churn: 20

  layer_violation:
    severity: error
    layers:
      - name: domain
        path: '**/domain/**'
        allowed_imports: []
      - name: application
        path: '**/application/**'
        allowed_imports: ['domain']

  complexity:
    max_complexity: 15

# Rule overrides for specific paths
overrides:
  - files: ['**/tests/**']
    rules:
      complexity: off
      god_module: off

# Scoring and grading settings
scoring:
  minimum: warn
  weights:
    critical: 100
    high: 50
    medium: 20
    low: 5

# Framework preset
extends: nestjs
```

## Available Detectors

Run `archlint detectors list` to see all detectors with descriptions.

| ID                | Name                  | Default Enabled |
| ----------------- | --------------------- | --------------- |
| `cycles`          | Circular Dependencies | ✅              |
| `god_module`      | God Module            | ✅              |
| `dead_code`       | Dead Code             | ✅              |
| `layer_violation` | Layer Violation       | ⚠️              |
| `complexity`      | High Complexity       | ✅              |
| ...               | ...                   | ...             |

[→ See full list of detectors](https://archlinter.github.io/archlint/detectors/)

## License

MIT License - see [LICENSE](LICENSE) for details.

## Credits

- Built with [oxc](https://oxc-project.github.io/) - blazingly fast JS/TS parser
- Inspired by [ArchUnit](https://www.archunit.org/), [madge](https://github.com/pahen/madge)

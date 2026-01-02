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

### From source

```bash
git clone https://github.com/yourusername/archlint.git
cd archlint
cargo build --release
cargo install --path .
```

### Using Cargo

```bash
cargo install archlint
```

## Quick Start

### Basic scan

```bash
archlint scan ./my-project
```

### With configuration

```bash
archlint scan ./my-project --config archlint.yaml
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
archlint scan --exclude-detectors barrel_file_abuse,lcom
```

## Configuration

Create `archlint.yaml` in your project root:

```yaml
# Files/directories to ignore
ignore:
  - '**/*.test.ts'
  - '**/*.spec.ts'
  - '**/__tests__/**'
  - '**/__mocks__/**'
  - '**/node_modules/**'
  - '**/dist/**'

# Path aliases (from tsconfig.json)
aliases:
  '@/*': 'src/*'
  '@components/*': 'src/components/*'
  '@utils/*': 'src/utils/*'

# Entry points (won't be marked as dead code)
entry_points:
  - 'src/main.ts'
  - 'src/index.ts'
  - '**/*.e2e.ts'
  - '**/pages/**' # Next.js pages

# Detector thresholds
thresholds:
  god_module:
    fan_in: 10 # Max incoming dependencies
    fan_out: 10 # Max outgoing dependencies
    churn: 20 # Max git commits

  hub_module:
    fan_in: 15
    fan_out: 15

  high_coupling:
    max_dependencies: 15

  lcom:
    threshold: 0.8 # 0-1 scale (higher = worse cohesion)

  deep_nesting:
    max_depth: 4

  long_params:
    max_params: 5

  complexity:
    max_complexity: 15

# Framework detection (auto-detected, can override)
frameworks:
  - nestjs
  - react

# Layer architecture (for layer_violation detector)
layers:
  - name: domain
    paths: ['**/domain/**']
    can_import: []

  - name: application
    paths: ['**/application/**', '**/use-cases/**']
    can_import: ['domain']

  - name: infrastructure
    paths: ['**/infrastructure/**', '**/adapters/**']
    can_import: ['domain', 'application']

  - name: presentation
    paths: ['**/controllers/**', '**/api/**']
    can_import: ['application', 'domain']

# Severity overrides
severity:
  cycles: critical
  god_module: high
  dead_code: low
  barrel_file_abuse: medium
```

## Available Detectors

Run `archlint detectors list` to see all detectors with descriptions.

| Detector                 | Description                                    | Default Enabled |
| ------------------------ | ---------------------------------------------- | --------------- |
| `cycles`                 | Circular dependencies between files            | ✅              |
| `circular_type_deps`     | Type-level circular dependencies               | ✅              |
| `god_module`             | Modules with too many responsibilities         | ✅              |
| `hub_module`             | Hub-like modules with high connectivity        | ✅              |
| `dead_code`              | Unused exports                                 | ✅              |
| `dead_symbols`           | Unused local functions and variables           | ✅              |
| `orphan_types`           | Types not connected to codebase                | ✅              |
| `lcom`                   | Low cohesion in classes (LCOM4 metric)         | ✅              |
| `high_coupling`          | Files with too many dependencies               | ✅              |
| `layer_violation`        | Architectural layer violations                 | ⚠️ Framework    |
| `sdp_violation`          | Stable Dependencies Principle violations       | ⚠️ Framework    |
| `barrel_file_abuse`      | Barrel files causing unnecessary coupling      | ✅              |
| `scattered_module`       | Related functionality scattered across files   | ✅              |
| `scattered_config`       | Configuration spread across codebase           | ✅              |
| `feature_envy`           | Methods using more of another class            | ✅              |
| `shotgun_surgery`        | Changes requiring modifications in many places | ✅              |
| `primitive_obsession`    | Overuse of primitives vs domain objects        | ✅              |
| `long_params`            | Functions with too many parameters             | ✅              |
| `deep_nesting`           | Deeply nested code blocks                      | ✅              |
| `complexity`             | High cyclomatic complexity                     | ✅              |
| `side_effect_import`     | Imports with side effects                      | ✅              |
| `test_leakage`           | Test code leaking into production              | ✅              |
| `vendor_coupling`        | Tight coupling to external libraries           | ✅              |
| `unstable_interface`     | Frequently changing public interfaces          | ✅              |
| `shared_mutable_state`   | Shared mutable state between modules           | ✅              |
| `abstractness_violation` | Pain/Useless zones (I+A metric)                | ✅              |
| `package_cycle`          | Cyclic dependencies between packages           | ✅              |

## Example Output

### Table Format (Default)

```
╭─────────────────────────────────────────────────────────────────────────╮
│                     Architecture Smell Report                          │
├─────────────────────────────────────────────────────────────────────────┤
│ Files analyzed: 247                                                     │
│ Smells detected: 12                                                     │
│ Total issues: 18                                                        │
╰─────────────────────────────────────────────────────────────────────────╯

╭───────────────┬──────────────────────────────┬──────────┬─────────────╮
│ Severity      │ Smell                        │ File     │ Score       │
├───────────────┼──────────────────────────────┼──────────┼─────────────┤
│ 🔴 CRITICAL   │ Cyclic Dependency            │ a.ts     │ 95          │
│               │                              │ b.ts     │             │
│               │                              │ c.ts     │             │
├───────────────┼──────────────────────────────┼──────────┼─────────────┤
│ 🟠 HIGH       │ God Module                   │ utils.ts │ 87          │
│               │ Fan-in: 23, Fan-out: 15      │          │             │
├───────────────┼──────────────────────────────┼──────────┼─────────────┤
│ 🟡 MEDIUM     │ Low Cohesion (LCOM)          │ user.ts  │ 65          │
│               │ LCOM4: 0.85                  │          │             │
╰───────────────┴──────────────────────────────┴──────────┴─────────────╯
```

### Markdown Format

### Markdown Format

See full example with Mermaid diagrams in generated reports.

## CLI Reference

```bash
# Scan commands
archlint scan [PATH]                    # Scan project
  --config <FILE>                       # Config file path
  --format <table|markdown|json>        # Output format
  --json                                # Shortcut for --format json (automatically enables --quiet)
  --report <FILE>                       # Save to file (default: stdout)
  --no-diagram                          # Disable Mermaid diagrams
  --all                                 # Run all detectors (including disabled)
  --detectors <IDS>                     # Only run specific detectors
  --exclude-detectors <IDS>             # Exclude specific detectors
  --min-severity <low|medium|high|critical> # Filter by severity
  --min-score <SCORE>                   # Filter by score (0-100)
  --quiet                               # CI-friendly mode (no progress bars)
  --verbose                             # Detailed output
  --no-cache                            # Disable caching

# Watch mode
archlint watch                          # Watch for changes
  --debounce <MS>                       # Debounce time (default: 300ms)
  --clear                               # Clear screen on re-run
  --ignore <PATTERN>                    # Additional ignore patterns
  [all scan options]                    # Accepts all scan options

# Detector management
archlint detectors list                 # List all available detectors

# Cache management
archlint cache clear                    # Clear analysis cache

# Shell completions
archlint completions <shell>            # Generate completion script for bash, zsh, fish, powershell
```

## Shell Completions

To enable command-line completions for `archlint`, you can generate a script for your shell:

### Bash

```bash
archlint completions bash > ~/.local/share/bash-completion/completions/archlint
```

### Zsh

```bash
archlint completions zsh > ~/.zfunc/_archlint
```

Then add `fpath+=~/.zfunc` to your `.zshrc` before `autoload -Uz compinit && compinit`.

### Fish

```bash
archlint completions fish > ~/.config/fish/completions/archlint.fish
```

### PowerShell

```powershell
archlint completions powershell | Out-String | Invoke-Expression
```

## Performance

- **Fast**: ~200 files analyzed in under 5 seconds
- **Parallel**: Uses Rayon for multi-threaded analysis
- **Cached**: Content-based caching (SHA256) for instant re-runs
- **Efficient**: AST parsing with `oxc` (faster than Babel/SWC)
- **Low memory**: Streaming analysis, no full AST kept in memory

## Framework Support

### NestJS

- Controllers/Modules recognized as entry points
- Relaxed coupling rules for modules/repositories
- SDP and layer violation checks enabled
- Ignores NestJS-specific decorators in LCOM

### Next.js

- Pages and API routes as entry points
- Barrel file rules relaxed (common pattern)
- Layer violation checks disabled

### React

- Components recognized by naming patterns
- LCOM checks disabled (not applicable)
- Scattered module checks relaxed

### oclif

- CLI commands/hooks as entry points
- Appropriate checks for CLI patterns

## Use Cases

### 🚀 CI/CD Integration

```yaml
# .github/workflows/architecture.yml
- name: Architecture Check
  run: |
    archlint scan --format json --report arch-report.json --min-severity high
    if [ $(jq '.summary.total_issues' arch-report.json) -gt 0 ]; then
      exit 1
    fi
```

### 🔄 Pre-commit Hook

```bash
# .git/hooks/pre-commit
#!/bin/bash
archlint scan --quiet --min-severity critical
```

### 📊 Regular Architecture Audits

```bash
# Generate weekly reports
archlint scan --format markdown --report "reports/$(date +%Y-%m-%d).md"
```

### 🧹 Refactoring Guidance

```bash
# Find high-priority issues
archlint scan --min-score 80 --format table

# Focus on specific problems
archlint scan --detectors cycles,god_module,high_coupling
```

## Comparison with Other Tools

| Feature          | archlint | madge     | dependency-cruiser | ts-unused-exports | ArchUnit (Java) |
| ---------------- | -------- | --------- | ------------------ | ----------------- | --------------- |
| Language         | TS/JS    | TS/JS     | TS/JS              | TS/JS             | Java            |
| Circular deps    | ✅       | ✅        | ✅                 | ❌                | ✅              |
| Dead code        | ✅       | ❌        | ❌                 | ✅                | ❌              |
| God modules      | ✅       | ❌        | ❌                 | ❌                | ⚠️ Custom rules |
| LCOM/Cohesion    | ✅       | ❌        | ❌                 | ❌                | ⚠️ Custom rules |
| Layer violations | ✅       | ❌        | ✅                 | ❌                | ✅              |
| Framework-aware  | ✅       | ❌        | ⚠️ Limited         | ❌                | ✅              |
| Watch mode       | ✅       | ✅        | ✅                 | ❌                | ❌              |
| AST-based        | ✅ (oxc) | ⚠️ (slow) | ✅                 | ✅                | ✅              |
| Performance      | ⚡ <5s   | 🐌 ~30s   | 🚀 ~10s            | ⚡ ~5s            | N/A             |
| 30+ detectors    | ✅       | ❌        | ❌                 | ❌                | ⚠️ Custom       |

**Key differentiators:**

- **Comprehensive**: 30+ architectural smell detectors vs competitors' narrow focus
- **Fast**: Rust + oxc parser = 5-10x faster than JS-based tools
- **Smart**: Framework-aware analysis (NestJS, Next.js, React, oclif)
- **Actionable**: Severity scores, explanations, and refactoring recommendations
- **Modern**: Type-level cycle detection, abstractness metrics, SDP violations

## Architecture

```
archlint
├── scanner      # File discovery with .gitignore support
├── parser       # AST parsing (oxc) + dependency extraction
├── resolver     # Path resolution (aliases, node_modules)
├── graph        # Dependency graph (petgraph)
├── detectors    # 30+ smell detectors
├── framework    # Framework detection & presets
├── engine       # Analysis orchestration
├── metrics      # Code metrics (LCOM, complexity, etc.)
├── cache        # Content-based caching
└── report       # Output formatters (table, markdown, json)
```

## Contributing

Contributions welcome! Areas for improvement:

- [ ] More detectors (data clumps, speculative generality, etc.)
- [ ] Support for other languages (Python, Go, Java)
- [ ] HTML interactive reports
- [ ] VS Code extension
- [ ] Auto-fix suggestions
- [ ] More framework presets (Angular, Vue, Svelte)

### Adding a Detector

```rust
use crate::framework::detector::{Detector, DetectorMetadata, Smell};
use inventory::submit;

pub struct MyDetector;

impl Detector for MyDetector {
    fn metadata(&self) -> DetectorMetadata {
        DetectorMetadata {
            id: "my_detector",
            name: "My Detector",
            description: "Detects something bad",
            severity: crate::framework::detector::Severity::Medium,
            enabled_by_default: true,
        }
    }

    fn detect(&self, context: &crate::engine::context::Context) -> Vec<Smell> {
        // Your detection logic here
        vec![]
    }
}

submit! { &MyDetector as &dyn Detector }
```

## Roadmap

- [x] Core detectors (cycles, god modules, dead code)
- [x] Framework detection (NestJS, Next.js, React)
- [x] Watch mode
- [x] Caching
- [x] 30+ detectors
- [ ] Auto-fix suggestions
- [ ] VS Code extension
- [ ] HTML reports with interactive graphs
- [ ] Python/Go support
- [ ] Trend analysis (track metrics over time)
- [ ] Integration with SonarQube/CodeClimate

## Troubleshooting

### Parser errors

```bash
# Enable verbose logging
RUST_LOG=debug archlint scan --verbose
```

### Slow analysis

```bash
# Check cache status
ls -lh .smell-lens-cache/

# Clear cache if stale
archlint cache clear

# Use --quiet to disable progress bars
archlint scan --quiet
```

### False positives

```yaml
# Adjust thresholds in archlint.yaml
thresholds:
  god_module:
    fan_in: 20 # Increase threshold
```

## License

MIT License - see [LICENSE](LICENSE) for details.

## Credits

- Built with [oxc](https://oxc-project.github.io/) - blazingly fast JS/TS parser
- Inspired by [ArchUnit](https://www.archunit.org/), [madge](https://github.com/pahen/madge), [dependency-cruiser](https://github.com/sverweij/dependency-cruiser)
- Academic foundations: Robert C. Martin's stability metrics, LCOM4 cohesion metric

## Contact

- Issues: [GitHub Issues](https://github.com/yourusername/archlint/issues)
- Discussions: [GitHub Discussions](https://github.com/yourusername/archlint/discussions)

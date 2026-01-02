# Roadmap

This document outlines planned features, improvements, and milestones for archlint.

## Status Legend

- ✅ **Completed**
- 🚧 **In Progress**
- 📋 **Planned**
- 💡 **Under Consideration**

---

## Core Features

### Detection & Analysis

- ✅ 30+ architectural smell detectors
- ✅ AST-based analysis with oxc parser
- ✅ Type-level cycle detection
- ✅ Framework-aware analysis (NestJS, Next.js, React, oclif)
- ✅ Layer violation detection
- ✅ Stable Dependencies Principle (SDP) checks
- ✅ LCOM4 cohesion metrics
- 📋 Data clumps detection
- 📋 Speculative generality detection
- 📋 Middle man detection
- 📋 Temporary field detection
- 💡 Inappropriate intimacy detection
- 💡 Message chains detection

### Performance & Caching

- ✅ Multi-threaded analysis with Rayon
- ✅ Content-based caching (SHA256)
- ✅ Parallel file processing
- 📋 Incremental analysis (analyze only changed files)
- 📋 Memory-optimized analysis for large codebases (>10k files)
- 💡 Distributed caching for monorepos

### Output & Reporting

- ✅ Table output format
- ✅ Markdown reports with Mermaid diagrams
- ✅ JSON output for CI/CD
- ✅ Severity filtering
- 📋 HTML interactive reports with:
  - Interactive dependency graphs
  - Drill-down capabilities
  - Code snippets
  - Trend visualization
- 📋 PDF export
- 📋 Sarif format for GitHub Code Scanning
- 💡 Integration with SonarQube
- 💡 Integration with CodeClimate

---

## Distribution & Publishing

### Package Distribution

- ✅ npm packages for CLI and core
- ✅ Platform-specific binaries (darwin-arm64, darwin-x64, linux-x64, linux-arm64, win32-x64)
- ✅ Automated versioning and releases
- 📋 **Publish Rust crate `archlint` with bindings to crates.io**
- 📋 **Publish CLI binary as Rust crate**
- 📋 Homebrew formula
- 📋 Debian/RPM packages
- 📋 Docker image
- 💡 Snap package
- 💡 Chocolatey package (Windows)

### Installation Methods

- ✅ `npx @archlinter/cli`
- 📋 `cargo install archlint`
- 📋 `brew install archlint`
- 📋 `apt install archlint`
- 📋 `docker pull archlinter/archlint`

---

## Developer Experience

### CLI & Tools

- ✅ Watch mode with debouncing
- ✅ Shell completions (bash, zsh, fish, powershell)
- ✅ Configuration file support (YAML)
- ✅ Path alias resolution
- ✅ Quiet mode for CI
- 📋 Auto-fix suggestions for common issues
- 📋 Explain command with detailed analysis
- 📋 Diff mode (compare before/after)
- 💡 Interactive mode for configuration setup
- 💡 Git hooks integration (husky, simple-git-hooks)

### IDE Integration

- 📋 VS Code extension with:
  - Inline diagnostics
  - Quick fixes
  - Code actions
  - Architecture view panel
  - Dependency graph visualization
- 💡 IntelliJ IDEA plugin
- 💡 Neovim LSP integration
- 💡 Language Server Protocol (LSP) implementation

### Configuration & Presets

- ✅ Framework presets (NestJS, Next.js, React, oclif)
- ✅ Custom threshold configuration
- ✅ Layer architecture definition
- ✅ Entry points configuration
- 📋 Shareable presets (similar to ESLint configs)
- 📋 More framework presets:
  - Angular
  - Vue
  - Svelte
  - Express
  - Fastify
  - tRPC
- 💡 Auto-detect and suggest framework presets
- 💡 Configuration migration tool

---

## Language Support

### Current Support

- ✅ TypeScript
- ✅ JavaScript (ES modules, CommonJS)
- ✅ TSX/JSX

### Planned Support

- 📋 Python
- 📋 Go
- 📋 Java
- 💡 Rust
- 💡 C#/.NET
- 💡 Ruby
- 💡 PHP

---

## Analytics & Insights

### Metrics & Trends

- ✅ Basic metrics (fan-in, fan-out, complexity)
- ✅ Git churn analysis
- 📋 Trend analysis (track metrics over time)
- 📋 Historical comparison
- 📋 Technical debt estimation
- 📋 Hotspot analysis (files with most issues)
- 💡 Predictive analysis (risk of future issues)
- 💡 Team metrics (contributor impact)

### Visualization

- ✅ Mermaid dependency graphs in Markdown
- 📋 Interactive web-based dependency explorer
- 📋 Heatmap of architectural smells
- 📋 Architecture evolution timeline
- 💡 3D dependency visualization
- 💡 Real-time dashboard

---

## Integration & Ecosystem

### CI/CD Integration

- ✅ JSON output for automation
- ✅ Exit codes for CI failures
- 📋 GitHub Actions official action
- 📋 GitLab CI template
- 📋 Bitbucket Pipelines template
- 📋 CircleCI orb
- 💡 Jenkins plugin

### Code Quality Platforms

- 📋 SonarQube plugin
- 📋 CodeClimate integration
- 📋 Codacy integration
- 💡 DeepSource integration

### Development Tools

- 📋 Pre-commit hook templates
- 📋 Danger.js integration
- 📋 Webhooks for notifications (Slack, Discord, Teams)
- 💡 Jira integration for issue creation

---

## Testing & Quality

### Robustness

- ✅ Comprehensive test suite
- ✅ Integration tests
- 📋 Fuzzing for parser
- 📋 Performance benchmarks
- 📋 Regression test suite
- 💡 Mutation testing

### Documentation

- ✅ CLI documentation
- ✅ Configuration guide
- ✅ Detector reference
- 📋 API documentation
- 📋 Architecture decision records (ADRs)
- 📋 Video tutorials
- 📋 Interactive playground
- 💡 Best practices guide per framework

---

## Community & Ecosystem

### Open Source

- ✅ MIT License
- ✅ Public GitHub repository
- 📋 Contribution guidelines
- 📋 Code of conduct
- 📋 Issue templates
- 📋 PR templates
- 💡 Good first issue labels
- 💡 Bounty program

### Community Support

- 📋 GitHub Discussions
- 📋 Discord server
- 📋 Blog with architecture tips
- 📋 Newsletter
- 💡 Annual conference/meetup
- 💡 Certification program

---

## Performance Goals

### Current Performance

- ✅ ~200 files in <5 seconds
- ✅ Cached re-runs: <1 second

### Target Performance

- 📋 1000 files in <10 seconds
- 📋 10,000 files in <60 seconds
- 📋 Memory usage <500MB for large codebases
- 💡 100,000+ files support for monorepos

---

## Milestones

### v0.4.0 - Enhanced Distribution 📋

**Target: Q1 2026**

- Publish `archlint` crate to crates.io
- Cargo install support
- Improved documentation
- API stability guarantees

### v0.5.0 - IDE Integration 📋

**Target: Q2 2026**

- VS Code extension beta
- Auto-fix suggestions
- Interactive HTML reports
- Incremental analysis

### v0.6.0 - Multi-language Support 📋

**Target: Q3 2026**

- Python support
- Go support
- Language-agnostic architecture rules

### v1.0.0 - Stable Release 📋

**Target: Q4 2026**

- API stability
- Comprehensive documentation
- Production-ready performance
- Enterprise support options

---

## Contributing

We welcome contributions! See areas marked with 📋 for planned features that need implementation.

Priority areas:

1. Auto-fix suggestions
2. VS Code extension
3. HTML interactive reports
4. Python language support
5. Publish Rust crates

For questions or suggestions, open an issue or discussion on GitHub.

---

## Feedback

Have ideas or requests? Open a [GitHub Discussion](https://github.com/archlinter/archlint/discussions) or submit an issue!

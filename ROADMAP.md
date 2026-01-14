# Roadmap

This roadmap is intentionally short. If something is not listed here, it is not currently planned. We stay focused on high-impact features, but we're open to suggestions and contributions that align with our core goals.

## Scope

archlint focuses on **JavaScript/TypeScript (including JSX/TSX)** codebases. The goal is fast, deterministic architectural analysis with CI-friendly output.

## Exists Today

- **Detectors**: 30+ architectural smell detectors (including layer violations, SDP, LCOM4)
- **Parsing**: AST-based analysis using `oxc`
- **Performance**: parallel analysis + content-based caching
- **Reporting**: table / JSON / Markdown (with Mermaid diagrams) / SARIF
- **DX**: watch mode, YAML config with `extends`, path alias resolution, framework presets
- **Distribution**: npm packages + platform-specific binaries
- **Baseline & Diff**: snapshot command + diff against git refs or files
- **Suppressions**: inline comments (`archlint-disable`, `archlint-disable-line`, etc.)
- **CI Integration**: GitHub Action with PR comments, GitLab CI support

## Next (High ROI)

- **Crates.io**: publish a Rust crate and enable `cargo install` for the CLI
- **Monorepo Boundaries**: workspace-aware analysis for Nx, Turborepo, pnpm workspaces (package-level dependency rules)
- **Plugin SDK (JS/TS)**: public API via `@archlinter/plugin-api` for custom detectors without Rust

## Maybe (Only If Needed)

- **LSP Server**: universal IDE support via Language Server Protocol (more flexible than editor-specific plugins)
- **Metrics Dashboard**: interactive visualization of architectural trends over time
- **Auto-fix**: automated refactoring for simple smells (barrel re-exports, import reorganization)

## Contributing

PRs are welcome, but this project is optimized for maintainability by a small team.

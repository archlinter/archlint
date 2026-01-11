# Roadmap

This roadmap is intentionally short. If something is not listed here, it is not planned.

## Scope

archlint focuses on **JavaScript/TypeScript (including JSX/TSX)** codebases. The goal is fast, deterministic architectural analysis with CI-friendly output.

## Exists Today

- **Detectors**: 30+ architectural smell detectors (including layer violations, SDP, LCOM4)
- **Parsing**: AST-based analysis using `oxc`
- **Performance**: parallel analysis + content-based caching
- **Reporting**: table / JSON / Markdown (with Mermaid diagrams)
- **DX**: watch mode, YAML config, path alias resolution, framework presets
- **Distribution**: npm packages + platform-specific binaries
- **Tooling**: diff mode, MCP server support

## Next (High ROI)

- **SARIF output**: GitHub/GitLab code scanning compatibility
- **Baseline mode**: ignore existing issues, catch only new ones
- **Crates.io**: publish a Rust crate and enable `cargo install` for the CLI

## Maybe (Only If Needed)

- **Interactive HTML report** (if Markdown/Mermaid is not enough)
- **More shareable presets** (if ecosystem demand appears)
- **VS Code integration** (only after output formats and rule stability are solid)

## Contributing

PRs are welcome, but this project is optimized for maintainability by a small team.

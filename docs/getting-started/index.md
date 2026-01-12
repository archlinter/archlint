---
title: Getting Started
description: Learn the philosophy and key features of archlint, an AST-based architecture smell detector for TypeScript and JavaScript.
---

# Introduction

archlint is an AST-based architecture smell detector for TypeScript and JavaScript projects. It's designed to help teams maintain a healthy codebase by preventing architectural regressions.

## Philosophy

### Ratchet, don't renovate

The biggest challenge with architectural debt is its volume. If a tool reports 500 circular dependencies on day one, the team will likely ignore it. archlint focuses on the **diff**. It locks the current state and only fails your CI if you introduce a _new_ smell or make an existing one worse.

### Explain, don't just report

Knowing that you have a "God Module" is only half the battle. archlint provides context: why it's considered a smell, how it impacts your codebase, and suggestions for refactoring.

### Zero friction

No servers to set up, no databases to maintain. It's a CLI tool that runs in seconds, works with your existing `.gitignore`, and can be integrated into any CI/CD pipeline with a single command.

## Key Features

- **28+ Detectors**: Covering dependencies, module design, complexity, and design patterns.
- **Fast**: Built with Rust and the `oxc` parser.
- **Framework-Aware**: Built-in intelligence for NestJS, Next.js, Express, React, Vue, Angular, and more.
- **Visual**: Generates reports with Mermaid diagrams for circular dependencies.
- **Integration**: ESLint plugin for real-time feedback and an MCP server for AI-assisted refactoring.

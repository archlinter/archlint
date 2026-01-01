# Release Flow

This document describes the automated release process for archlint.

## Overview

archlint uses **semantic-release** for fully automated versioning and publishing. Version numbers are determined automatically from commit messages following the [Conventional Commits](https://www.conventionalcommits.org/) specification.

## Commit Message Format

All commits **must** follow the Conventional Commits format. This is enforced by commitlint in CI.

### Format

```
<type>(<scope>): <subject>

<body>

<footer>
```

### Types and Version Impact

| Type | Description | Version Bump | Example |
|------|-------------|--------------|---------|
| `feat` | New feature | **Minor** (0.x.0) | `feat: add barrel file detector` |
| `fix` | Bug fix | **Patch** (0.0.x) | `fix: resolve false positive in cycles` |
| `perf` | Performance improvement | **Patch** (0.0.x) | `perf: optimize graph traversal` |
| `refactor` | Code refactoring | None | `refactor: simplify detector logic` |
| `docs` | Documentation | None | `docs: update README` |
| `test` | Tests | None | `test: add coverage for god module` |
| `chore` | Maintenance | None | `chore: update dependencies` |
| `ci` | CI/CD changes | None | `ci: add caching to workflows` |
| `build` | Build system | None | `build: update Cargo.toml` |

### Breaking Changes

Add `!` after the type or `BREAKING CHANGE:` in the footer to trigger a **major** version bump:

```bash
# Major version bump (1.0.0)
git commit -m "feat!: change API signature"

# Or
git commit -m "feat: new feature

BREAKING CHANGE: This changes the public API"
```

## Release Process

### 1. Development

Develop features in feature branches:

```bash
git checkout -b feat/new-detector
# Make changes
git commit -m "feat: add new max-exports detector"
git push origin feat/new-detector
```

### 2. Pull Request

Create a PR to `main`. CI will:
- ✅ Validate commit messages (commitlint)
- ✅ Run linting (rustfmt, clippy)
- ✅ Run tests
- ✅ Build binaries

### 3. Merge to Main

When PR is merged, the `version.yml` workflow automatically:

1. **Analyzes commits** since the last release
2. **Determines version** based on commit types:
   - `feat` → minor bump (0.2.0)
   - `fix`/`perf` → patch bump (0.1.1)
   - `feat!` or `BREAKING CHANGE` → major bump (1.0.0)
3. **Updates version** in:
   - `Cargo.toml`
   - All `packages/*/package.json`
4. **Generates CHANGELOG.md** with all changes
5. **Creates commit** with message `chore(release): vX.Y.Z [skip ci]`
6. **Creates Git tag** `vX.Y.Z`
7. **Pushes** commit and tag

### 4. Automatic Release

Tag push triggers `release.yml` workflow:

1. **Verify**: Run all tests and linting
2. **Build**: Compile binaries for all platforms:
   - macOS (ARM64, x64)
   - Linux (x64, x64-musl, ARM64)
   - Windows (x64)
3. **Publish npm**:
   - Platform-specific packages (`@archlinter/cli-darwin-arm64`, etc.)
   - Main CLI package (`@archlinter/cli`)
4. **GitHub Release**: Create release with standalone binaries

## Version Numbers

All packages share the same version (unified versioning):
- `@archlinter/cli@0.2.0`
- `@archlinter/cli-darwin-arm64@0.2.0`
- `@archlinter/cli-linux-x64@0.2.0`
- etc.

**Source of truth**: Both `Cargo.toml` and `package.json` are synchronized automatically.

## Examples

### Feature Release (Minor)

```bash
# Develop
git commit -m "feat: add cyclomatic complexity threshold"

# After merge to main
# → Version: 0.1.0 → 0.2.0
```

### Bug Fix (Patch)

```bash
git commit -m "fix: correct false positive in dead code detection"

# After merge to main
# → Version: 0.2.0 → 0.2.1
```

### Breaking Change (Major)

```bash
git commit -m "feat!: redesign configuration API

BREAKING CHANGE: Config structure changed from flat to nested"

# After merge to main
# → Version: 0.2.1 → 1.0.0
```

### Multiple Commits

If PR has multiple commits, the highest version bump wins:

```bash
git commit -m "fix: minor bug"      # patch
git commit -m "feat: new detector"  # minor
git commit -m "docs: update README" # none

# After merge to main
# → Version bumps by minor (highest)
```

## Manual Release (Emergency Only)

For critical hotfixes that can't wait:

```bash
# Create and push tag manually
git tag v0.2.2
git push origin v0.2.2

# This triggers the release workflow
```

⚠️ **Warning**: Manual tags should be rare. Prefer the automatic flow.

## Pre-release Versions

Tag with pre-release suffix for alpha/beta/rc:

```bash
git tag v1.0.0-alpha.1
git tag v1.0.0-beta.1
git tag v1.0.0-rc.1
```

GitHub Release will be marked as "pre-release" automatically.

## Checking Release Status

### View Workflow Status

https://github.com/archlinter/archlint/actions

### Verify npm Publication

```bash
npm view @archlinter/cli
npm view @archlinter/cli-darwin-arm64
```

### Test Installation

```bash
npx @archlinter/cli@latest --version
```

## Troubleshooting

### Commit Rejected by commitlint

```
✖   subject may not be empty [subject-empty]
✖   type may not be empty [type-empty]
```

**Fix**: Follow conventional commits format:
```bash
git commit --amend -m "feat: correct commit message"
```

### Release Workflow Failed

Check:
1. All tests passing?
2. NPM_TOKEN secret configured?
3. Version conflicts?

View logs: GitHub Actions → Release workflow

### Version Not Bumped

Possible reasons:
1. Commit message doesn't trigger version (e.g., `docs:`, `chore:`)
2. Commit contains `[skip ci]`
3. No commits since last release

## Reference

- [Conventional Commits](https://www.conventionalcommits.org/)
- [Semantic Versioning](https://semver.org/)
- [semantic-release](https://github.com/semantic-release/semantic-release)
- [Plan 6: CI/CD](.cursor/docs/plans/plan-06-ci-cd.md)
- [Plan 11: Release Process](.cursor/docs/plans/plan-11-release-process.md)

# Release Flow

This document describes the release process for archlint.

## Overview

archlint uses **Release Drafter** to automatically create draft releases from pull requests. Version numbers and release notes are managed through PR labels and titles.

## Commit Message Format

All commits **must** follow the Conventional Commits format. This is enforced by commitlint in CI.

### Format

```
<type>(<scope>): <subject>

<body>

<footer>
```

### Types

| Type       | Description             | Label           | Version Bump      |
| ---------- | ----------------------- | --------------- | ----------------- |
| `feat`     | New feature             | `feature`       | **Minor** (0.x.0) |
| `fix`      | Bug fix                 | `fix`           | **Patch** (0.0.x) |
| `perf`     | Performance improvement | `performance`   | **Patch** (0.0.x) |
| `refactor` | Code refactoring        | `chore`         | None              |
| `docs`     | Documentation           | `documentation` | None              |
| `test`     | Tests                   | `chore`         | None              |
| `chore`    | Maintenance             | `chore`         | None              |
| `ci`       | CI/CD changes           | `chore`         | None              |
| `build`    | Build system            | `chore`         | None              |

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

Create a PR to `main`. The PR will be automatically labeled based on the commit message:

- `feat:` → `feature` label
- `fix:` → `fix` label
- `perf:` → `performance` label
- `docs:` → `documentation` label
- `feat!:` or `BREAKING CHANGE` → `breaking` label

CI will:

- ✅ Validate commit messages (commitlint)
- ✅ Run linting (rustfmt, clippy)
- ✅ Run tests
- ✅ Build binaries

### 3. Merge to Main

When PR is merged, **Release Drafter** automatically:

1. Creates or updates a **draft release** on GitHub
2. Adds the PR to the release notes under the appropriate category
3. Calculates the next version based on PR labels

### 4. Review Draft Release

Go to [GitHub Releases](https://github.com/archlinter/archlint/releases) to see the draft:

- Review the generated release notes
- Edit the version tag if needed (e.g., `v0.2.0`)
- Make any adjustments to the description

### 5. Publish Release

Click **Publish release** to trigger the full release workflow:

1. **Update Versions**: Updates `Cargo.toml`, `package.json`, and `CHANGELOG.md`
2. **Commit**: Creates a commit `chore(release): X.Y.Z [skip ci]`
3. **Verify**: Runs all tests and linting
4. **Build**: Compiles binaries for all platforms:
   - macOS (ARM64, x64)
   - Linux (x64, x64-musl, ARM64)
   - Windows (x64)
5. **Publish npm**:
   - Platform-specific packages (`@archlinter/cli-darwin-arm64`, etc.)
   - Main CLI package (`@archlinter/cli`)
6. **Attach Binaries**: Uploads standalone binaries to the GitHub release

## Version Numbers

All packages share the same version (unified versioning):

- `@archlinter/cli@0.2.0`
- `@archlinter/cli-darwin-arm64@0.2.0`
- `@archlinter/cli-linux-x64@0.2.0`
- etc.

**Source of truth**: The Git tag determines the version. Files are updated during the release workflow.

## Examples

### Feature Release (Minor)

```bash
# Create PR with feat commit
git commit -m "feat: add cyclomatic complexity threshold"
# PR gets 'feature' label automatically
# After merge → draft release updated
# Publish release → version 0.1.0 → 0.2.0
```

### Bug Fix (Patch)

```bash
git commit -m "fix: correct false positive in dead code detection"
# PR gets 'fix' label automatically
# After merge → draft release updated
# Publish release → version 0.2.0 → 0.2.1
```

### Breaking Change (Major)

```bash
git commit -m "feat!: redesign configuration API

BREAKING CHANGE: Config structure changed from flat to nested"
# PR gets 'breaking' label automatically
# After merge → draft release updated
# Publish release → version 0.2.1 → 1.0.0
```

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
3. GH_PAT secret configured?
4. Version conflicts?

View logs: GitHub Actions → Release workflow

### Draft Release Not Updated

Possible reasons:

1. PR was not merged (only closed)
2. Release Drafter workflow failed
3. No changes since last release

## Reference

- [Conventional Commits](https://www.conventionalcommits.org/)
- [Semantic Versioning](https://semver.org/)
- [Release Drafter](https://github.com/release-drafter/release-drafter)

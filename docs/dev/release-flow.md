# Release Flow

This document describes the release process for archlint.

## Overview

archlint uses **semantic-release** to automate the entire release workflow. Version numbers are calculated based on commit messages following the Conventional Commits format.

## Commit Message Format

All commits **must** follow the Conventional Commits format. This is enforced by commitlint in CI.

### Format

```
<type>(<scope>): <subject>

<body>

<footer>
```

### Types

| Type       | Description             | Version Bump      |
| ---------- | ----------------------- | ----------------- |
| `feat`     | New feature             | **Minor** (0.x.0) |
| `fix`      | Bug fix                 | **Patch** (0.0.x) |
| `perf`     | Performance improvement | **Patch** (0.0.x) |
| `refactor` | Code refactoring        | None              |
| `docs`     | Documentation           | None              |
| `test`     | Tests                   | None              |
| `chore`    | Maintenance             | None              |
| `ci`       | CI/CD changes           | None              |
| `build`    | Build system            | None              |

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

Develop features in feature branches and merge them into `main`.

### Prerelease Branches

The `.releaserc.json` file contains static branch configurations for `beta` and `alpha` channels. However, **prerelease branches are configured dynamically by CI** during the release workflow. The workflow automatically creates branch configurations based on the selected channel and current branch name, so the static entries in `.releaserc.json` are not used during actual releases.

### 2. Trigger Release

When you are ready to release, manually trigger the Release workflow:

1. Go to **Actions** -> **Release** workflow.
2. Click **Run workflow**.
3. (Optional) Set `dry_run` to `true` to see what would happen without actually publishing.

### 3. Automatic Steps

The workflow will:

1. **Calculate Version**: `semantic-release` analyzes commits since the last release.
2. **Update Files**: Automatically updates `Cargo.toml`, `package.json`, and `CHANGELOG.md`.
3. **Commit & Tag**: Creates a new commit and Git tag for the release.
4. **Trigger CI**: The tag push triggers the CI workflow, which builds all binaries.
5. **Publish to npm**: CI publishes all packages to the npm registry (only on tags).
6. **Attach Binaries**: CI uploads standalone binaries to the GitHub Release.

## Version Numbers

All packages share the same version (unified versioning):

- `@archlinter/cli@0.2.0`
- `@archlinter/cli-darwin-arm64@0.2.0`
- `@archlinter/cli-linux-x64@0.2.0`
- etc.

## Checking Release Status

### View Workflow Status

https://github.com/archlinter/archlint/actions

### Verify npm Publication

```bash
npm view @archlinter/cli
```

### Test Installation

```bash
npx @archlinter/cli@latest --version
```

## Troubleshooting

### Commit Rejected by commitlint

**Fix**: Follow conventional commits format:

```bash
git commit --amend -m "feat: correct commit message"
```

### Release Workflow Failed

Check:

1. NPM_TOKEN secret configured?
2. GH_PAT secret configured?
3. Did the CI build fail?

## Reference

- [Conventional Commits](https://www.conventionalcommits.org/)
- [Semantic Versioning](https://semver.org/)
- [semantic-release](https://github.com/semantic-release/semantic-release)

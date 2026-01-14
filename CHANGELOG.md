## [0.13.0](https://github.com/archlinter/archlint/compare/v0.12.0...v0.13.0) (2026-01-14)

### Features

- **cli:** add init command, dynamic presets architecture and enhanced TS resolution ([#26](https://github.com/archlinter/archlint/issues/26)) ([e0df14b](https://github.com/archlinter/archlint/commit/e0df14b723e1263275b1c1b51f016d6978fd9e1f))

### ⚠️ Breaking Changes

- Snapshot format has been changed
  → Existing snapshots must be regenerated

## [0.12.0](https://github.com/archlinter/archlint/compare/v0.11.0...v0.12.0) (2026-01-11)

### Features

- implement inline ignoring via archlint-disable comments ([#21](https://github.com/archlinter/archlint/issues/21)) ([97b50bb](https://github.com/archlinter/archlint/commit/97b50bbdb0b7454c1afc52ae9f9dac4e22b1b86f))

### Bug Fixes

- **archlint:** resolve duplicate smell ids and side-effect false positives (fixes [#23](https://github.com/archlinter/archlint/issues/23)) ([#24](https://github.com/archlinter/archlint/issues/24)) ([1ed2c5a](https://github.com/archlinter/archlint/commit/1ed2c5ae7b26b01818b2ac06e8812a4c371ab0dd))

## [0.11.0](https://github.com/archlinter/archlint/compare/v0.10.0...v0.11.0) (2026-01-11)

### Features

- add Trivy security scanning to CI and fix dependency vulnerability ([#18](https://github.com/archlinter/archlint/issues/18)) ([03ab184](https://github.com/archlinter/archlint/commit/03ab184d0755f44943b8a258213616714feca32f))
- refactor framework preset system to YAML-based unified configuration ([#17](https://github.com/archlinter/archlint/issues/17)) ([bc8b86f](https://github.com/archlinter/archlint/commit/bc8b86f55bc4d728026cea6fb26b2c58add9127d))
- unify severity levels and implement tsconfig integration ([#20](https://github.com/archlinter/archlint/issues/20)) ([82ae522](https://github.com/archlinter/archlint/commit/82ae522e934ed018302730d3d3c6dd0da7bbd49e))

## [0.10.0](https://github.com/archlinter/archlint/compare/v0.9.0...v0.10.0) (2026-01-08)

### Features

- implement deterministic code clone detector ([#16](https://github.com/archlinter/archlint/issues/16)) ([2cef2bc](https://github.com/archlinter/archlint/commit/2cef2bc77e92ab0ed2bd3899fe3bd3fa37e5c573))

### Documentation

- add favicon and optimize SEO for documentation ([88b1b3a](https://github.com/archlinter/archlint/commit/88b1b3ae7f4a889fb01d198de7440b180cf9d1fc))
- add internationalization support for 5 languages ([c1dc15d](https://github.com/archlinter/archlint/commit/c1dc15d75d43132fafacd0ec89b2310c63cedf1d))

## [0.9.0](https://github.com/archlinter/archlint/compare/v0.8.2...v0.9.0) (2026-01-07)

### Features

- add locations to snapshot smell for line number reporting ([2be6a95](https://github.com/archlinter/archlint/commit/2be6a95c9fc831ed43497a2f62b8751d28ac4847))

## [0.8.2](https://github.com/archlinter/archlint/compare/v0.8.1...v0.8.2) (2026-01-07)

### Bug Fixes

- **cli:** attempt to fetch git ref if not found locally ([#14](https://github.com/archlinter/archlint/issues/14)) ([9d4df7c](https://github.com/archlinter/archlint/commit/9d4df7c4f07d2f73eca602d89f1308f9693ff11e))

### Documentation

- add VitePress documentation and setup GitHub Pages deployment ([2e119be](https://github.com/archlinter/archlint/commit/2e119be48e1bff265c86cc16e87b519323bcd8c3))
- promote official GitHub Action in README and installation guide ([#13](https://github.com/archlinter/archlint/issues/13)) ([87a10c9](https://github.com/archlinter/archlint/commit/87a10c9de8b91a6a1dd0e16bb50b509f834719e9))

## [0.8.1](https://github.com/archlinter/archlint/compare/v0.8.0...v0.8.1) (2026-01-06)

### Bug Fixes

- **core:** improve dead symbols detection with polymorphism and inher… ([#11](https://github.com/archlinter/archlint/issues/11)) ([aeceeb4](https://github.com/archlinter/archlint/commit/aeceeb418a571f8c4c18a3b864e72748edd6114a))

## [0.8.0](https://github.com/archlinter/archlint/compare/v0.7.0...v0.8.0) (2026-01-06)

### Features

- **core:** implement architectural diff mode and ESLint regression rule ([#10](https://github.com/archlinter/archlint/issues/10)) ([553b6ea](https://github.com/archlinter/archlint/commit/553b6ea7e40c85e4562c43042379bed0250c538a)), closes [#8](https://github.com/archlinter/archlint/issues/8)

## [0.7.0](https://github.com/archlinter/archlint/compare/v0.6.0...v0.7.0) (2026-01-06)

### Features

- add support for git history analysis period ([#7](https://github.com/archlinter/archlint/issues/7)) ([0a20d2b](https://github.com/archlinter/archlint/commit/0a20d2b0b910afc565e935c559db6fb43411f50c))
- **core:** implement persistent git history cache using redb ([#5](https://github.com/archlinter/archlint/issues/5)) ([b2f3555](https://github.com/archlinter/archlint/commit/b2f35554f18f904fa916ba6fbb8b13bfad515b58))

### Performance Improvements

- **core:** migrate analysis cache to binary format (bincode) ([#6](https://github.com/archlinter/archlint/issues/6)) ([2543a52](https://github.com/archlinter/archlint/commit/2543a52043b370f58fd4695c55214a6a204778d1))

## [0.6.0](https://github.com/archlinter/archlint/compare/v0.5.0...v0.6.0) (2026-01-04)

### Features

- **eslint-plugin:** implement architectural smell detection rules ([#4](https://github.com/archlinter/archlint/issues/4)) ([117d401](https://github.com/archlinter/archlint/commit/117d401b4b268eb8da8dd7dda44a67fc6dac7bad))

## [0.6.0-alpha.1](https://github.com/archlinter/archlint/compare/v0.5.0...v0.6.0-alpha.1) (2026-01-04)

### Features

- add incremental scan with overlays support ([cc53e14](https://github.com/archlinter/archlint/commit/cc53e14be7675c2d7d5ca8291c8ab4d872a1203d))
- **eslint-plugin:** implement architectural smell detection rules ([672aa37](https://github.com/archlinter/archlint/commit/672aa37d3ac1fdef1ecf5398ecefead8f5d274f5))

### Bug Fixes

- **ci:** ignore npm publish errors for existing versions ([4dbeb70](https://github.com/archlinter/archlint/commit/4dbeb707d46acca27eedc6d967cd49e3207974cd))
- **eslint-plugin:** prevent hash algorithm desync ([c3466fb](https://github.com/archlinter/archlint/commit/c3466fb7894ec4e85fcfa2de7ed4c805e5b85f07))
- **release:** add channel config and fetch-tags for prerelease detection ([fcbf2fd](https://github.com/archlinter/archlint/commit/fcbf2fd27a7eed84b84cb25a729e6ae451bab502))
- **release:** add debug step and improve prerelease tag detection ([2a809d0](https://github.com/archlinter/archlint/commit/2a809d025f45566a77d64b7ac644376cd1b34c49))
- **release:** clean up prerelease state before semantic-release ([0d307cf](https://github.com/archlinter/archlint/commit/0d307cf40cf8f00456caf4614f49e27a4fdce23e))

### Performance Improvements

- **eslint-plugin:** optimize file hashing with xxhash-wasm and size checks ([ab1ef50](https://github.com/archlinter/archlint/commit/ab1ef5094d89bbe4bcf216d09129e56b8a421036))

## [0.6.0-alpha.1](https://github.com/archlinter/archlint/compare/v0.5.0...v0.6.0-alpha.1) (2026-01-03)

### Features

- **eslint-plugin:** implement architectural smell detection rules ([672aa37](https://github.com/archlinter/archlint/commit/672aa37d3ac1fdef1ecf5398ecefead8f5d274f5))

### Bug Fixes

- **ci:** ignore npm publish errors for existing versions ([4dbeb70](https://github.com/archlinter/archlint/commit/4dbeb707d46acca27eedc6d967cd49e3207974cd))
- **release:** add channel config and fetch-tags for prerelease detection ([fcbf2fd](https://github.com/archlinter/archlint/commit/fcbf2fd27a7eed84b84cb25a729e6ae451bab502))
- **release:** add debug step and improve prerelease tag detection ([2a809d0](https://github.com/archlinter/archlint/commit/2a809d025f45566a77d64b7ac644376cd1b34c49))
- **release:** clean up prerelease state before semantic-release ([0d307cf](https://github.com/archlinter/archlint/commit/0d307cf40cf8f00456caf4614f49e27a4fdce23e))

## [0.6.0-alpha.1](https://github.com/archlinter/archlint/compare/v0.5.0...v0.6.0-alpha.1) (2026-01-03)

### Features

- **eslint-plugin:** implement architectural smell detection rules ([672aa37](https://github.com/archlinter/archlint/commit/672aa37d3ac1fdef1ecf5398ecefead8f5d274f5))

### Bug Fixes

- **release:** add channel config and fetch-tags for prerelease detection ([fcbf2fd](https://github.com/archlinter/archlint/commit/fcbf2fd27a7eed84b84cb25a729e6ae451bab502))

## [0.6.0-alpha.1](https://github.com/archlinter/archlint/compare/v0.5.0...v0.6.0-alpha.1) (2026-01-03)

### Features

- **eslint-plugin:** implement architectural smell detection rules ([672aa37](https://github.com/archlinter/archlint/commit/672aa37d3ac1fdef1ecf5398ecefead8f5d274f5))

### Bug Fixes

- **release:** add channel config and fetch-tags for prerelease detection ([fcbf2fd](https://github.com/archlinter/archlint/commit/fcbf2fd27a7eed84b84cb25a729e6ae451bab502))

## [0.5.0](https://github.com/archlinter/archlint/compare/v0.4.1...v0.5.0) (2026-01-02)

## [0.5.0](https://github.com/archlinter/archlint/compare/v0.4.1...v0.5.0) (2026-01-02)

### Features

- **mcp:** implement model context protocol server ([#3](https://github.com/archlinter/archlint/issues/3)) ([8face1e](https://github.com/archlinter/archlint/commit/8face1eea265a26e0996e3d0df24d0c54631a5ca))

## [0.4.1](https://github.com/archlinter/archlint/compare/v0.4.0...v0.4.1) (2026-01-02)

### Bug Fixes

- add repository field to core package for npm provenance ([a9b5d66](https://github.com/archlinter/archlint/commit/a9b5d6660a93180fa2a446e884505862f258b273))

## [0.4.0](https://github.com/archlinter/archlint/compare/v0.3.1...v0.4.0) (2026-01-02)

### Features

- **cli:** add shell completions command ([05198d5](https://github.com/archlinter/archlint/commit/05198d535a99c578d7bfd5bcdeef93665ec6b9ce))
- **core:** add platform packages and update CI for napi modules ([9cbc22e](https://github.com/archlinter/archlint/commit/9cbc22edf6613be80b3a0c29f49eaaac404b7ccd))
- **core:** implement napi-rs bindings and initial tests ([1ace4de](https://github.com/archlinter/archlint/commit/1ace4dee4c3b3dfb3a0d64e1b21cd1496df8096d))

# [0.4.0](https://github.com/archlinter/archlint/compare/v0.3.1...v0.4.0) (2026-01-02)

### Features

- **cli:** add shell completions command ([05198d5](https://github.com/archlinter/archlint/commit/05198d535a99c578d7bfd5bcdeef93665ec6b9ce))
- **core:** add platform packages and update CI for napi modules ([9cbc22e](https://github.com/archlinter/archlint/commit/9cbc22edf6613be80b3a0c29f49eaaac404b7ccd))
- **core:** implement napi-rs bindings and initial tests ([1ace4de](https://github.com/archlinter/archlint/commit/1ace4dee4c3b3dfb3a0d64e1b21cd1496df8096d))

# [0.3.0](https://github.com/archlinter/archlint/compare/v0.2.0...v0.3.0) (2026-01-02)

### Features

- **archlint:** implement public Rust API for programmatic use ([75e73a8](https://github.com/archlinter/archlint/commit/75e73a89ffd5a29880d9057efc049afba9a4339e))

# [0.2.0](https://github.com/archlinter/archlint/compare/v0.1.3...v0.2.0) (2026-01-02)

### Features

- add glob expansion and project root detection ([0f6122e](https://github.com/archlinter/archlint/commit/0f6122e3fed148c6e4f3d4e319ed95fa417d1c79))

## [0.1.3](https://github.com/archlinter/archlint/compare/v0.1.2...v0.1.3) (2026-01-02)

### Bug Fixes

- remove changeset ([1904811](https://github.com/archlinter/archlint/commit/1904811dde7b0c48776693c0dc171a1c3f83484b))

## [0.1.2](https://github.com/archlinter/archlint/compare/v0.1.1...v0.1.2) (2026-01-02)

### Bug Fixes

- include README in npm package ([2116344](https://github.com/archlinter/archlint/commit/211634405d337290be5debae86230de85a37bf51))

## [0.1.1](https://github.com/archlinter/archlint/compare/v0.1.0...v0.1.1) (2026-01-02)

### Bug Fixes

- replace workspace protocol with actual versions for npm compatibility ([2dfc1f9](https://github.com/archlinter/archlint/commit/2dfc1f9c87a7eaf8f9199fee17b83deda855bf30))
- restore workspace protocol for development, use --publish flag for releases ([df85b29](https://github.com/archlinter/archlint/commit/df85b29f0b811a95ef3974b6a190c71bdce6c96a))

# 1.0.0 (2026-01-02)

### Features

- initial release with CI/CD and automated versioning ([ac627ea](https://github.com/archlinter/archlint/commit/ac627ea08f0e1ad57f2d30aeab726ce3c553cc28))

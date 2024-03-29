# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- next-header -->

## [Unreleased] <!-- release-date -->

### Changed

- Refuse to clone if repository is not empty.
- Do not trust the cloned repository.

## [0.1.2] - 2024-03-29

### Added

- Add support for 2014-01-11 and 2014-01-18, which use a non-standard "Quotes of the Week" heading.

## [0.1.1] - 2024-03-29

### Added

- Add a ChangeLog.
- Add `cargo release` configuration.

### Changed

- Change release tag schema from `X.Y.Z` to `vX.Y.Z` in accordance with `cargo release`.
- Set package to `publish = false`, preventing accidental crates.io spam.

## [0.1.0] - 2024-03-29

### Added

- Initial implementation in Berlin Rust Hack and Learn 2024-03-28.

<!-- next-url -->
[Unreleased]: https://github.com/LeoniePhiline/twir-qotw-scraper/compare/v0.1.2...HEAD

[0.1.2]: https://github.com/LeoniePhiline/twir-qotw-scraper/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/LeoniePhiline/twir-qotw-scraper/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/LeoniePhiline/twir-qotw-scraper/releases/tag/v0.1.0

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.0] - 2023-07-29

### Changed

- Type of struct members (all but year) to `u8` to allocate less memory
- Clarified possible values of struct member year
- Format of as_string(): 4 digit year is ISO 8601 spec
- Style changes in Rust docs

## [0.2.2] - 2023-07-27

### Added

- Run also doc tests in CI
- Run clippy also for `main.rs` in CI
- Badges to README
- PR template

### Changed

- Improved examples
- Added links to CHANGELOG
- Updated link to Keep a Changelog 1.1.0

## [0.2.1] - 2023-07-25

### Fixed

- Typos in README and docs (hopefully all of them)

## [0.2.0] - 2023-07-25

### Added

- Binary `uts2ts SECONDS`
- CHANGELOG.md

### Fixed

- Typos in README and docs

## [0.1.0] - 2023-07-25

Initial release.

[0.3.0]: https://github.com/tessus/uts2ts/compare/v0.2.2...v0.3.0
[0.2.2]: https://github.com/tessus/uts2ts/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/tessus/uts2ts/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/tessus/uts2ts/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/tessus/uts2ts/releases/tag/v0.1.0

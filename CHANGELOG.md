# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [v0.4.0] - 2022-09-11

### Changed
- Update [msp430] crate from `0.3.0` to `0.4.0`. This crate remains
  API-compatible with [v0.3.0]. Updating [msp430] is _transitively_ a breaking
  change due to previously always-enabled functionality now being gated behind
  the default-disabled `critical-section-single-core` feature.

## [v0.3.0] - 2022-01-25

### Changed
- Update [msp430] crate from `0.2.0` to `0.3.0`. This crate remains
  API-compatible with [v0.2.0]. This is being treated as a breaking change for
  consistency with version bumps of other crates tailored to msp430.

## [v0.2.0] - 2020-01-02

### Changed
- Update [msp430] crate from `0.1.0` to `0.2.0`. This crate remains
  API-compatible with [v0.1.0]. This is being treated as a breaking change for
  consistency with version bumps of other crates tailored to msp430.

## [v0.1.0] - 2019-08-26

Initial release.

[msp430]: https://github.com/rust-embedded/msp430

[Unreleased]: https://github.com/YuhanLiin/panic-msp430/compare/v0.4.0...HEAD
[v0.4.0]: https://github.com/YuhanLiin/panic-msp430/compare/v0.3.0...v0.4.0
[v0.3.0]: https://github.com/YuhanLiin/panic-msp430/compare/v0.2.0...v0.3.0
[v0.2.0]: https://github.com/YuhanLiin/panic-msp430/compare/v0.1.0...v0.2.0
[v0.1.0]: https://github.com/YuhanLiin/panic-msp430/tree/v0.1.0

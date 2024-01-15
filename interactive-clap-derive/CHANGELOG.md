# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.8](https://github.com/near-cli-rs/interactive-clap/compare/interactive-clap-derive-v0.2.7...interactive-clap-derive-v0.2.8) - 2024-01-15

### Added
- Added possibility to process optional fields ([#13](https://github.com/near-cli-rs/interactive-clap/pull/13))

## [0.2.7](https://github.com/near-cli-rs/interactive-clap/compare/interactive-clap-derive-v0.2.6...interactive-clap-derive-v0.2.7) - 2023-10-13

### Added
- Add support for "flatten" ([#11](https://github.com/near-cli-rs/interactive-clap/pull/11))

## [0.2.6](https://github.com/near-cli-rs/interactive-clap/compare/interactive-clap-derive-v0.2.5...interactive-clap-derive-v0.2.6) - 2023-10-05

### Fixed
- named_args/unnamed_args/args_without_attrs conflict ([#9](https://github.com/near-cli-rs/interactive-clap/pull/9))

## [0.2.5](https://github.com/near-cli-rs/interactive-clap/compare/interactive-clap-derive-v0.2.4...interactive-clap-derive-v0.2.5) - 2023-09-21

### Fixed
- fixed unnamed_args/args_without_attrs conflict

### Other
- added fn try_parse_from()
- Merge branch 'master' of https://github.com/FroVolod/interactive-clap

## [0.2.4](https://github.com/near-cli-rs/interactive-clap/compare/interactive-clap-derive-v0.2.3...interactive-clap-derive-v0.2.4) - 2023-06-02

### Added
- Add support for boolean flags (e.g. --offline) ([#6](https://github.com/near-cli-rs/interactive-clap/pull/6))

## [0.2.3](https://github.com/near-cli-rs/interactive-clap/compare/interactive-clap-derive-v0.2.2...interactive-clap-derive-v0.2.3) - 2023-05-30

### Fixed
- Trim unnecessary spaces in inquire prompts (fix it again after recent refactoring that reverted the previous fix)

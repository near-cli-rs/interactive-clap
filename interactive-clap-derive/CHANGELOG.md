# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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

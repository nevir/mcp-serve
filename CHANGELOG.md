# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0](https://github.com/nevir/mcp-serve/releases/tag/v0.1.0) - 2025-09-13

### Added

- implement tool definition data structures ([#59](https://github.com/nevir/mcp-serve/pull/59))
- *(tool-discovery)* add external dependencies ([#58](https://github.com/nevir/mcp-serve/pull/58))
- implement release binary upload mechanism ([#42](https://github.com/nevir/mcp-serve/pull/42))
- add Dependabot configuration for GitHub Actions updates ([#39](https://github.com/nevir/mcp-serve/pull/39))
- integrate release-plz for automated releases ([#33](https://github.com/nevir/mcp-serve/pull/33))
- implement cross-compilation build matrix for releases ([#32](https://github.com/nevir/mcp-serve/pull/32))
- create GitHub Actions workflow structure ([#30](https://github.com/nevir/mcp-serve/pull/30))

### Fixed

- *(ci)* improve Conventional Commits validation for PR titles ([#31](https://github.com/nevir/mcp-serve/pull/31))

### Other

- Pin the claude workflow too
- Correctly pin all actions
- Extra mise commands
- More generic command templates
- Drop CONTIRBUTING for now
- Streamline README.md with terse, engaging format
- Add README.md and CONTRIBUTING.md documentation
- Formatting
- Change --tools flag to optional positional parameter
- Update --tool flag to --tools with directory parameter
- Implement CLI structure with clap derive API
- Setup Cargo.toml with clap dependency and binary target
- enable mcp github
- Project Skeleton

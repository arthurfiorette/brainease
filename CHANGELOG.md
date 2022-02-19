# Changelog

All notable changes to this project will be documented in this file.

## [unreleased]

### Bug Fixes

- Wrap pointer if value is the same as memory length
- Fixed goto capturing group
- Use @ for pointer as # is a comment :p
- Handled pointer overflow
- Fixed comment detection
- Fixed exponential line_index bug
- Fixed match_indentation function
- Fixed infinite loop after indentation ends
- Fixed multiple indentations bugs

### Documentation

- Added changelog
- Fixed site url
- More documentation
- Added initial docs
- Crates.io badge

### Features

- Accept # as pointer
- Goto instruction
- [**breaking**] Integrated `if_cell` into `if`
- [**breaking**] Require `*` for cell reference

### Miscellaneous Tasks

- Updated debugger
- Bump clap from 3.0.14 to 3.1.0 (#1)
- Added debug config for vscode

### Refactor

- Trace cli arguments
- Split io_handler into multiple files
- IoHandler for different IO modes
- Runtime supports different IOs
- Better error messages
- Clippy
- Updated cli

### Testing

- Added integration tests

### Ci

- Dependabot.yml
- Added docs action

## [1.0.3] - 2022-02-07

### Ci

- Add sleep between publishes

### Tag

- V1.0.3

## [1.0.2] - 2022-02-07

### Ci

- Fix publish steps

### Tag

- V1.0.2

## [1.0.1] - 2022-02-07

### Documentation

- Added readme
- Updated license

### Features

- Cli ready
- Runtime ready
- Lexer ready
- Lexer almost ready
- Added lexer syntax

### Miscellaneous Tasks

- Updated repo structure
- Updated cargo.toml version
- Updated hello example and gitignore
- Some config files

### Refactor

- Lexer parser
- Impl FromStr for IfLogic

### Testing

- Fixed tests

### Ci

- Fix publish steps
- Cache target directory
- Renamed actions
- Fix publish event
- Unique CI for all packages and publish action
- Added github ci

### Init

- Init cargo workspace

### Tag

- V1.0.1


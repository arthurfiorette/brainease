# Changelog

All notable changes to this project will be documented in this file.

## [1.0.5] - 2022-03-05

### Bug Fixes

- Clippy
- Flush output after every instruction loop
- Extra chars warns even if its only comments

### Documentation

- Added examples

### Miscellaneous Tasks

- Use cargo-make for common tasks
- Add funding.yml
- Bump clap from 3.1.3 to 3.1.5 (#6)
- Bump cli to v1.0.4

### Refactor

- Change -f to argument
- Use None when GotoDirection is 1

### Ci

- Fixed ci and updated changelog

### Deps

- Updated autocfg

### Tag

- V1.0.5

## [1.0.4] - 2022-03-01

### Bug Fixes

- Use 30.000 as default memory length
- Update pointer every loop check
- Fixed unknown indentation after double indentation block
- Goto index out of bounds (#2)
- Write and print output (#2)
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

- Updated hello world example
- Fixed changelog
- Added changelog
- Fixed site url
- More documentation
- Added initial docs
- Crates.io badge

### Features

- Removed run option after transpilation
- Cli using new transpiler
- Transpile cli and subcommands
- Transpile from brainfuck code
- Use .brain as file extension (#2)
- Accept # as pointer
- Goto instruction
- [**breaking**] Integrated `if_cell` into `if`
- [**breaking**] Require `*` for cell reference

### Miscellaneous Tasks

- Bump clap from 3.1.2 to 3.1.3 (#5)
- Clippy code
- Bump clap from 3.1.1 to 3.1.2 (#4)
- Bump clap from 3.1.0 to 3.1.1 (#3)
- Updated debugger
- Bump clap from 3.0.14 to 3.1.0 (#1)
- Added debug config for vscode

### Refactor

- Smaller and faster transpiler
- Cli paths normalization and result handling (#2)
- Idiomatic rust code
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

- Attach binaries on latest release
- Updated ci to test all packages
- Dependabot.yml
- Added docs action

### Tag

- Bump all brainease crates

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


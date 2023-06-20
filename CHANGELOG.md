# Changelog

All notable changes to this project will be documented in this file.

## [unreleased]

### Refactor

- Removed pub mod tests

## [1.0.7] - 2023-06-20

### Bug Fixes

- Include correct binary name on releasing
- Remove deprecated warnings
- Flush output before reading any input
- Added break all type
- Fixed ci

### Documentation

- Fixed cname
- Added pointer movement page
- Updated documentation page
- Assignment operators page
- Updated docs site url
- Fixed link url
- Removed margin top
- Learning section
- Added install page
- Updated readme png
- Formatted license and readme
- Updated docs sketch
- Updated readme example
- Added web server example

### Features

- Tag v1.0.7
- Added arm version
- Updated dependencies
- [**breaking**] Rename binary into brainz
- Exit, break and continue
- Return and exit instructions
- Allow if with chars
- Interpret escape characters in print, write and save
- Allow direct print without saving it in memory

### Miscellaneous Tasks

- Bump env_logger from 0.9.0 to 0.9.1 (#23)
- Bump clap from 3.1.18 to 3.2.2 (#22)
- Gitattributes
- Bump clap-verbosity-flag from 1.0.0 to 1.0.1 (#21)
- Bump regex from 1.5.4 to 1.5.6 (#20)
- Updated sponsors
- Bump clap from 3.1.17 to 3.1.18 (#19)
- Bump clap from 3.1.15 to 3.1.17 (#18)
- Bump clap from 3.1.14 to 3.1.15 (#15)
- Bump log from 0.4.16 to 0.4.17 (#16)
- Bump clap from 3.1.12 to 3.1.14 (#14)
- Bump clap from 3.1.10 to 3.1.12 (#13)
- Bump clap from 3.1.9 to 3.1.10 (#12)
- Bump clap from 3.1.8 to 3.1.9 (#11)
- Bump clap from 3.1.6 to 3.1.8 (#10)
- Bump log from 0.4.14 to 0.4.16 (#9)
- Bump clap from 3.1.5 to 3.1.6 (#8)
- Bump lazy-regex from 2.2.2 to 2.3.0 (#7)
- Allow print! on tests
- Gitignore
- Fixed launch config

### Refactor

- Clippy and Eq imports
- Minor things
- Added cli descriptions

### Testing

- Refactored some tests
- If char tests
- Added transpiler initial unit tests

### Ci

- Fixed docs ci
- Updated docs CI

## [1.0.6] - 2022-03-05

### Bug Fixes

- Fixed integration tests
- Exit early when parse_line return error
- Prevent panic when file is not found
- Clippy
- Flush output after every instruction loop
- Extra chars warns even if its only comments

### Documentation

- Updated readme symlinks
- Fixed readme example
- Added examples

### Miscellaneous Tasks

- Use cargo-make for common tasks
- Add funding.yml
- Bump clap from 3.1.3 to 3.1.5 (#6)
- Bump cli to v1.0.4

### Refactor

- Dedicated struct for instructions
- Rename binary to `brain`
- Move crates under packages/**
- Change -f to argument
- Use None when GotoDirection is 1

### Ci

- Fixed Rust CI syntax
- Groupped CI into 3 per-os jobs
- Fixed ci and updated changelog

### Deps

- Updated autocfg

### Tag

- V1.0.6
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


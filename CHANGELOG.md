# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.4.0] - 2025-12-04

### Added
- GitHub Actions CI workflow for automated testing and quality checks
- Documentation badges in README
- Workspace package configuration for shared metadata
- Documentation warnings for better code documentation

### Changed
- **BREAKING**: Updated to Leptos 0.8.14 compatibility (view! macro syntax changed from `class =` to `class:`)
- Updated Rust toolchain to latest nightly (from nightly-2023-07-28)
- Updated all dependencies to latest versions using version ranges
- Improved error handling in build script with better error messages
- Improved error handling in proc macros (return compile errors instead of panicking)
- Improved string formatting efficiency in class name generation
- Updated workspace configuration to use shared package metadata

### Fixed
- Fixed duplicate condition bug in `rand_class_from_seed` method
- Fixed typos: "_componet_name" → "_component_name", "statment" → "statement"
- Fixed typo: "This create" → "This crate" in documentation
- Fixed README grammar and spelling issues (11 fixes)
- Fixed dependency configuration to use path dependencies for workspace members
- Fixed deprecated `proc_macro` to `proc-macro` in Cargo.toml

### Documentation
- Added comprehensive CHANGELOG.md
- Added CONTRIBUTING.md with development guidelines
- Added Rust nightly requirement documentation
- Improved README with badges and better structure
- Updated all examples to use Leptos 0.8.14 syntax

## [Unreleased]

## [1.0.0-alpha] - 2024

### Added
- Initial alpha release
- `style!` macro for inline CSS
- `style_sheet!` macro for external CSS files
- `style_str!` macro returning tuple (class_name, style_val)
- `style_sheet_str!` macro returning tuple (class_name, style_val)
- Scoped CSS support
- Custom `:deep()` pseudo-class for deep DOM tree styling
- Support for CSS at-rules (@media, @keyframes, etc.)

### Compatibility
- Compatible with Leptos 0.8.14
- Requires nightly Rust due to `proc_macro_span` feature


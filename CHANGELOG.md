# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [7.0.0] 2025-08-11
### Changed
- [#59](https://github.com/rambler-digital-solutions/actix-web-validator/pull/59): Update validator dependency to 0.20

## [6.0.0] 2024-07-11
### Changed
- [#55](https://github.com/rambler-digital-solutions/actix-web-validator/pull/55): Update validator dependency to 0.18
- [#53](https://github.com/rambler-digital-solutions/actix-web-validator/pull/53): Update serde_qs to 0.13

## [5.0.1] 2022-08-25
### Fixed
- Update README

## [5.0.0] 2022-08-25
### Changed
- [#37](https://github.com/rambler-digital-solutions/actix-web-validator/pull/37): Update validator dependency to 0.16

## [4.0.0] 2022-07-18
### Added
- [#36](https://github.com/rambler-digital-solutions/actix-web-validator/pull/36): Support for nested errors in default output

### Changed
- [#34](https://github.com/rambler-digital-solutions/actix-web-validator/pull/34): Update validator dependency to 0.15

## [3.0.0] 2022-03-15
### Added 
- [#29](https://github.com/rambler-digital-solutions/actix-web-validator/pull/29): Add actix-web 4.x support.

### Changed
- Rust edition changed to 2021.

### Removed
- [#29](https://github.com/rambler-digital-solutions/actix-web-validator/pull/29): ValidatedJson ValidatedQuery and ValidatedPath now removed.

## [2.2.0] 2021-10-04
### Added
- [#27](https://github.com/rambler-digital-solutions/actix-web-validator/pull/27): Add validation support for Form extractor.

## [2.1.1] 2021-06-07
### Fixed
- Fix QsQuery and QsQueryConfig documentation.

## [2.1.0] 2021-06-07
### Added
- [#23](https://github.com/rambler-digital-solutions/actix-web-validator/pull/23): Add serde_qs support query deserialization.
- [#20](https://github.com/rambler-digital-solutions/actix-web-validator/issues/20): Adds some additional information about validation errors into default HTTP response.

### Deprecated
- Deprecate of reexporting `validator::Validate` trait.

## [2.0.3] 2021-01-12
### Added
- [#17](https://github.com/rambler-digital-solutions/actix-web-validator/issues/17): Add reexport of Validate trait from validator.

## [2.0.2] 2020-12-28
### Fixed
- [#16](https://github.com/rambler-digital-solutions/actix-web-validator/issues/16): Update validator dependency to 0.12.

## [2.0.1] 2020-09-27
### Fixed 
- [#15](https://github.com/rambler-digital-solutions/actix-web-validator/issues/15): Disable default features for actix-web dependency. 

## [2.0.0] 2020-09-18
### Added
- [#13](https://github.com/rambler-digital-solutions/actix-web-validator/issues/13): Add actix-web 3.x.x support.

### Deprecated
- `ValidatedJson`, `ValidatedQuery` and `ValidatedPath` are deprecated in favor of same names from actix (`Json`, `Query` and `Path`).

## [1.0.0] 2020-06-06
- [#11](https://github.com/rambler-digital-solutions/actix-web-validator/issues/11): Add actix-web 2.0.0 support.

## [0.2.1] 2020-06-05
### Added
- Add documentation link and Readme into crates.io page.

## [0.2.0] 2020-06-05
### Added
- [#6](https://github.com/rambler-digital-solutions/actix-web-validator/issues/6): Add Deref trait implementation for VaidatedQuery.
- [#5](https://github.com/rambler-digital-solutions/actix-web-validator/issues/5): Add ValidatedJson implementation.
- [#3](https://github.com/rambler-digital-solutions/actix-web-validator/issues/3): Add ValidatedPath implementation.
- [#2](https://github.com/rambler-digital-solutions/actix-web-validator/issues/2): Add tests.
- [#1](https://github.com/rambler-digital-solutions/actix-web-validator/issues/1): Add documentation.

## [0.1.2] 2020-02-28
### Added
- Add implementation of `ValidatedQuery`

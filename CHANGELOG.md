# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
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
- `ValidatedJson`, `ValidatedQuery` and `ValidatedPath` are depricated in favor of same names from actix (`Json`, `Query` and `Path`).

## [1.0.0] 2020-06-06
- [#11](https://github.com/rambler-digital-solutions/actix-web-validator/issues/11): Add actix-web 2.0.0 support.

## [0.2.1] 2020-06-05
### Added
- Add documnetation link and Readme into crates.io page.

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

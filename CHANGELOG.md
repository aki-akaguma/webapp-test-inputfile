# Changelog: test-inputfile

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Changed
* update crate: dioxus(0.7.7)


## [0.1.4] (2026-04-27)
### Changed
* I reorganized the dependencies in `Cargo.toml`.
* update crate: dioxus(0.7.6)

### Fixed
* `dx bundle --desktop --release --package-types appimage` on `Makefile`

## [0.1.3] (2026-04-05)
### Changed
* `load_url` = "https://appassets.androidplatform.net/assets/pre.html"
* profile.release: opt-level = "s"

## [0.1.2] (2026-04-03)
### Changed
* updated crates: dioxus(0.7.4)

## [0.1.1] (2026-02-26)
### Added
* desktop download
* `android-versionCode-inc` into Makefile
* download link: blob
* download link: data
* download link: asset file
* resources and scripts

### Changed
* android-webview-assets template

### Removed
* android-webview-params.toml

### Fixed
* too many arguments
* javascript: `setDataUrlToBlobLink()`

## [0.1.0] (2026-02-19)
### Added
* first commit

[Unreleased]: https://github.com/aki-akaguma/webapp-test-inputfile/compare/v0.1.4..HEAD
[0.1.4]: https://github.com/aki-akaguma/webapp-test-inputfile/compare/v0.1.3..v0.1.4
[0.1.3]: https://github.com/aki-akaguma/webapp-test-inputfile/compare/v0.1.2..v0.1.3
[0.1.2]: https://github.com/aki-akaguma/webapp-test-inputfile/compare/v0.1.1..v0.1.2
[0.1.1]: https://github.com/aki-akaguma/webapp-test-inputfile/compare/v0.1.0..v0.1.1
[0.1.0]: https://github.com/aki-akaguma/webapp-test-inputfile/releases/tag/v0.1.0

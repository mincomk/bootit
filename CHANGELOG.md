# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-02-05

### Added
- Initial feature set.

## [0.1.1] - 2026-02-06

### Fixed
- Fix `allow-non-admin` flag not making the owner of file root.

## [0.1.2] - 2026-02-06

### Fixed
- Fix Windows build issue

## [0.2.0] - 2026-02-06

### Added
- Add Windows support.
- Add `BOOTIT_CONFIG_PATH` environment variable to specify custom config file path.

### Changed
- Change default config file path in Linux to `/etc/bootit.yaml`.
- Change default config file path in Windows to `C:\ProgramData\bootit\config.yaml`.

## [0.2.1] - 2026-02-06

### Fixed
- Fix Windows installer build issue.

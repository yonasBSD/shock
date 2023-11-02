# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.4](https://github.com/ipetkov/shock/compare/v0.1.3...v0.1.4) - 2023-11-02

### Fixed
- *(systemd)* don't start service on startup
- *(systemd)* don't restart service when changed

### Other
- *(deps)* bump clap from 4.4.6 to 4.4.7 ([#16](https://github.com/ipetkov/shock/pull/16))
- *(deps)* bump toml from 0.8.2 to 0.8.6 ([#18](https://github.com/ipetkov/shock/pull/18))
- *(deps)* bump serde from 1.0.188 to 1.0.190 ([#17](https://github.com/ipetkov/shock/pull/17))
- *(deps)* bump bstr from 1.6.2 to 1.7.0 ([#15](https://github.com/ipetkov/shock/pull/15))
- *(deps)* bump DeterminateSystems/nix-installer-action from 4 to 6 ([#19](https://github.com/ipetkov/shock/pull/19))

## [0.1.3](https://github.com/ipetkov/shock/compare/v0.1.2...v0.1.3) - 2023-10-09

### Added
- print out how many snapshots were deleted on success ([#12](https://github.com/ipetkov/shock/pull/12))

### Other
- *(deps)* update all dependencies ([#13](https://github.com/ipetkov/shock/pull/13))
- *(flake)* Update flake.lock ([#10](https://github.com/ipetkov/shock/pull/10))

## [0.1.2](https://github.com/ipetkov/shock/compare/v0.1.1...v0.1.2) - 2023-10-01

### Other
- *(deps)* bump clap from 4.4.3 to 4.4.6 ([#9](https://github.com/ipetkov/shock/pull/9))
- *(deps)* bump toml from 0.8.0 to 0.8.1 ([#8](https://github.com/ipetkov/shock/pull/8))
- update gh cli flag
- also delete branch after merging flake updates
- enable auto approve of flake updates if they pass tests

## [0.1.1](https://github.com/ipetkov/shock/compare/v0.1.0...v0.1.1) - 2023-09-17

### Other
- Update README
- fix token for flake updates
- *(deps)* bump toml from 0.7.8 to 0.8.0 ([#5](https://github.com/ipetkov/shock/pull/5))
- *(deps)* bump clap from 4.4.2 to 4.4.3 ([#4](https://github.com/ipetkov/shock/pull/4))
- add workflow to update flake.lock
- enable dependabot for cargo
- Run release-plz only after tests have passed
- release v0.1.0 ([#2](https://github.com/ipetkov/shock/pull/2))

## [0.1.0](https://github.com/ipetkov/shock/releases/tag/v0.1.0) - 2023-09-17

### Other
- Initial release

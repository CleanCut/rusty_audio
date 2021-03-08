<!-- next-header -->

## [Unreleased] - ReleaseDate

## [1.2.0] - 2021-03-07

### Changed

- Moved `rusty_audio` back out of `rusty_engine` into its own standalone project again
- Fixed adding clips being broken
- Updated `rodio` to 0.13.0
- Added a `play` example. Run it with `cargo run --example play`

## 1.1.5 - 2021-02-26

### Changed

- Updated `rodio` to 0.11.0 to get the downstream `cpal` dependency up to 0.11.0

## 1.1.4 - 2020-03-24

### Changed

- Removed the `prelude` module

## 1.1.3 - 2020-03-14

### Changed

- `rusty_audio` became part of [`rusty_engine`] as of this release (reverted in after 1.1.5)
- Added option to use Apache 2.0 license

[`rusty_engine`]: https://github.com/cleancut/rusty_engine

## 1.1.2 - 2020-03-14

### Borked

- Failed release -- Yanked

## 1.1.1 - 2019-11-19

### Changed

- Updated `rodio` to 0.10.0

## 1.1.0 - 2019-10-10 (later in the day)

### Added

- Added the `.wait()` method to `Audio` to block until no sounds are playing

## 1.0.0 - 2019-10-10

### Added

- Initial release, ported from [rusty_sword_arena](https://github.com/cleancut/rusty_sword_arena)
  so it can be reused in other projects such as 
  [rusty_sword](https://github.com/cleancut/rusty_sword)

<!-- next-url -->
[Unreleased]: https://github.com/cleancut/rusty_audio/compare/v1.2.0...HEAD
[1.2.0]: https://github.com/cleancut/rusty_audio/compare/v1.1.5...v1.2.0

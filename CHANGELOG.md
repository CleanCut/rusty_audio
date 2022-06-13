<!-- next-header -->

## [Unreleased] - ReleaseDate

### Changed

- Even better than the change in `1.3.0`, instead of accepting `Into<_>` for `Audio::add` and `Audio::play`, lets accept `AsRef<str>` and `AsRef<Path>`, since that's what we'll be dealing with a lot of the time. And there shouldn't be any situations which force folks to clone something to pass to these methods. 🤞🏻

## [1.3.0] - 2022-06-13

### Changed

- Instead of accepting only `&'static str`, which was difficult to use with anything other than a string literal, `Audio::add` and `Audio::play` are now generic over anything that implements `Into<String>` for the name and `Into<PathBuf>` for the filename.
- Documented the release process & moved the release config into a dedicated file
- Updated to `rodio 0.14.0`.

## [1.2.1] - 2021-03-14

### Changed

- Fixed clippy warnings
- Improved CI & Readme

## [1.2.0] - 2021-03-07

### Changed

- Moved `rusty_audio` back out of `rusty_engine` into its own standalone project again
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
- Moved `rusty_audio` to the `rusty_engine` workspace and re-exported it as `audio`

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
[Unreleased]: https://github.com/cleancut/rusty_audio/compare/v1.3.0...HEAD
[1.3.0]: https://github.com/cleancut/rusty_audio/compare/v1.2.1...v1.3.0
[1.2.1]: https://github.com/cleancut/rusty_audio/compare/v1.2.0...v1.2.1
[1.2.0]: https://github.com/cleancut/rusty_audio/compare/v1.1.5...v1.2.0

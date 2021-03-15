[![Crates.io Version](https://img.shields.io/crates/v/rusty_audio.svg)](https://crates.io/crates/rusty_audio)
[![Crates.io Downloads](https://img.shields.io/crates/d/rusty_audio.svg)](https://crates.io/crates/rusty_audio)
[![Build Status](https://github.com/cleancut/rusty_audio/workflows/CI/badge.svg)](https://github.com/cleancut/rusty_audio/actions)


# Rusty Audio Playback Library

`rusty_audio` is a fun and easy audio playback library that provides a 4-track audio system to
load/decode audio files and play them, perfect for small projects.  It is also well-suited for
training purposes, and is featured in the [Ultimate Rust Crash Course] on Udemy.

- Formats: MP3, WAV, Vorbis and Flac.
- Platforms: Supports macOS, Windows, and iOS out of the box. Linux requires installation of
  [extra dependencies](#dependencies-on-linux).

This library uses the [rodio] audio playback library under the hood, which you should
consider using directly if your needs are more complex.

### Example

```toml
# Add this to your [dependencies] section in Cargo.toml
rusty_audio = "1.2.0"
```

```rust
// main.rs
use rusty_audio::Audio;

fn main() {
    let mut audio = Audio::new();
    audio.add("startup", "my_sound_file.mp3"); // Load the sound, give it a name
    audio.play("startup"); // Execution continues while playback occurs in another thread.
    audio.wait(); // Block until sounds finish playing
}
```

### Built-in Example

You can run the built-in example by cloning this repository, and then running:

```shell
$ cargo run --example play
```

### Dependencies on Linux

For Linux, the [CPAL] package that is used under the hood
requires the *alsa* development libraries to be installed.

**CentOS**

```bash
sudo yum install -y alsa-lib-devel
```

**Debian/Ubuntu**

```bash
sudo apt install libasound2-dev
```

## Contribution

All contributions are assumed to be dual-licensed under MIT/Apache-2.

## License

Distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See [license/APACHE](license/APACHE) and [license/MIT](license/MIT).

## Sponsor

If you like Rusty Audio, please consider [sponsoring me] on GitHub. 💖

[CPAL]: https://github.com/RustAudio/cpal
[rodio]: https://github.com/RustAudio/rodio
[sponsoring me]: https://github.com/sponsors/CleanCut
[Ultimate Rust Crash Course]: https://agileperception.com/ultimate_rust_crash_course

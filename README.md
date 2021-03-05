# `rusty_audio`

`rusty_audio` is a fun and easy audio library that provides a 4-track audio system to load/decode
audio files and play them. Supported formats are: MP3, WAV, Vorbis and Flac.

This library uses the very powerful [rodio] audio playback library under the hood, which you should
consider using directly if your needs are more complex.

[`rusty_engine`]: https://github.com/cleancut/rusty_engine
[rodio]: https://github.com/RustAudio/rodio

### Example

```toml
# Add this to your [dependencies] section in Cargo.toml
rusty_audio = "1.0"
```

```rust
// main.rs
use rusty_audio::Audio;

fn main() {
    let mut audio = Audio::new();
    audio.add("startup", "audio_subsystem_initialized.mp3"); // Load the sound, give it a name
    audio.play("startup"); // Execution continues while playback occurs in another thread.
    audio.wait(); // Block until sounds finish playing
}
```

### Dependencies on Linux

`rusty_audio` should work out-of-the-box on macOS, Windows, and iOS.  For Linux, the
([CPAL](https://github.com/RustAudio/cpal) package that is used under the hood requires
the *alsa* development libraries to be installed.

**CentOS**

```bash
sudo yum install -y alsa-lib-devel
```

**Debian/Ubuntu**

```bash
sudo apt install libasound2-dev
```
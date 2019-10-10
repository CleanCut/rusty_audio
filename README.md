# Rusty Audio (`rusty_audio`)

Rusty Audio is a very simple audio library created for other educational `rusty_*` projects that I
use for courses I teach online, at conferences, or in-person.  It uses the very powerful
[rodio](https://github.com/tomaka/rodio) audio playback library under the hood, which you should use
directly if your needs are more complex.

### Dependencies on Linux

This library should work out-of-the-box on macOS, Windows, iOS, and emscripten.  For Linux, the
downstream package for actually _playing_ sound ([CPAL](https://github.com/RustAudio/cpal) requires
the *alsa* development libraries to be installed.

**CentOS**

```bash
sudo yum install -y alsa-lib-devel
```

**Debian/Ubuntu**

```bash
sudo apt install libasound2-dev
```
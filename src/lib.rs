//! `rusty_audio` is a convenient sound library for small projects and educational purposes.  For
//! more elaborate needs, please use [rodio](https://github.com/tomaka/rodio), which is the much
//! more powerful audio library that this one uses.
//!
//! Example
//! =======
//! ```
//! use rusty_audio::Audio;
//! let mut audio = Audio::new();
//! audio.add("startup", "audio_subsystem_initialized.mp3"); // Load the sound, give it a name
//! audio.play("startup"); // Execution continues while playback occurs in another thread.
//! audio.wait(); // Block until sounds finish playing
//! ```

use rodio::{
    source::{Buffered, Source},
    Decoder, OutputStream, OutputStreamHandle, Sink,
};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Cursor, Read};

pub mod prelude {
    pub use crate::Audio;
}

/// A simple 4-track audio system to load/decode audio files from disk to play later. Supported
/// formats are: MP3, WAV, Vorbis and Flac.
#[derive(Default)]
pub struct Audio {
    clips: HashMap<&'static str, Buffered<Decoder<Cursor<Vec<u8>>>>>,
    channels: Vec<Sink>,
    current_channel: usize,
    output: Option<(OutputStream, OutputStreamHandle)>,
}

impl Audio {
    /// Create a new sound subsystem.  You only need one of these -- you can use it to load and play
    /// any number of audio clips.
    pub fn new() -> Self {
        if let Ok(output) = OutputStream::try_default() {
            let clips = HashMap::new();
            let mut channels: Vec<Sink> = Vec::new();
            for i in 0..4 {
                let sink = Sink::try_new(&output.1)
                    .unwrap_or_else(|_| panic!("Failed to create sound channel {}", i));
                channels.push(sink);
            }
            Self {
                clips,
                channels,
                current_channel: 0,
                output: Some(output),
            }
        } else {
            Self {
                clips: HashMap::new(),
                channels: Vec::new(),
                current_channel: 0,
                output: None,
            }
        }
    }
    /// If no sound device was detected, the audio subsystem will run in a disabled mode that
    /// doesn't actually do anything. This method indicates whether audio is disabled.
    pub fn disabled(&self) -> bool {
        self.output.is_none()
    }
    /// Add an audio clip to play.  Audio clips will be decoded and buffered during this call so
    /// the first call to `.play()` is not staticky if you compile in debug mode.  `name` is what
    /// you will refer to this clip as when you need to play it.  Files known to be supported by the
    /// underlying library (rodio) at the time of this writing are MP3, WAV, Vorbis and Flac.
    pub fn add(&mut self, name: &'static str, path: &str) {
        if self.disabled() {
            return;
        }
        let mut file_vec: Vec<u8> = Vec::new();
        File::open(path)
            .expect("Couldn't find audio file to add.")
            .read_to_end(&mut file_vec)
            .expect("Failed reading in opened audio file.");
        let cursor = Cursor::new(file_vec);
        let decoder = Decoder::new(cursor).unwrap();
        let buffered = decoder.buffered();
        // Buffers are lazily decoded, which often leads to static on first play on low-end systems
        // or when you compile in debug mode.  Since this library is intended for educational
        // projects, those are going to be common conditions.  So, to optimize for our use-case, we
        // will pre-warm all of our audio buffers by forcing things to be decoded and cached right
        // now when we first load the file.  I would like to find a cleaner way to do this, but the
        // following scheme (iterating through a clone and discarding the decoded frames) works
        // since clones of a Buffered share the actual decoded data buffer cache by means of Arc and
        // Mutex.
        let warm = buffered.clone();
        for i in warm {
            #[allow(clippy::drop_copy)]
            drop(i);
        }
        self.clips.insert(name, buffered);
    }
    /// Play an audio clip that has already been loaded.  `name` is the name you chose when you
    /// added the clip to the `Audio` system. If you forgot to load the clip first, this will crash.
    pub fn play(&mut self, name: &str) {
        if self.disabled() {
            return;
        }
        let buffer = self.clips.get(name).expect("No clip by that name.").clone();
        self.channels[self.current_channel].append(buffer);
        self.current_channel += 1;
        if self.current_channel >= self.channels.len() {
            self.current_channel = 0;
        }
    }
    /// Block until no sounds are playing. Convenient for keeping a thread alive until all sounds
    /// have played.
    pub fn wait(&self) {
        if self.disabled() {
            return;
        }
        loop {
            if self.channels.iter().any(|x| !x.empty()) {
                std::thread::sleep(std::time::Duration::from_millis(50));
                continue;
            }
            break;
        }
    }
}

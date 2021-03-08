use rusty_audio::Audio;

fn main() {
    // Create a 4-channel audio subsystem.
    let mut audio = Audio::new();
    // On systems without an audio card, Audio will run in disabled mode, which doesn't do anything,
    // but at least doesn't crash.  Helpful for CI.
    if audio.disabled() {
        println!("Sorry, we couldn't find an audio device, so no sound will play for you. :'-(");
    }
    audio.add("startup", "audio_subsystem_initialized.mp3"); // Load the sound, give it a name
    audio.play("startup"); // Execution continues while playback occurs in another thread.
    audio.wait(); // Block until sounds finish playing
}

# To-Do List

This file tracks the immediate development tasks.

-   **[ ] Fix Audio Playback (cpal implementation):**
    -   **[ ] Correct Trait Bounds:** Fix the compilation errors in `music_visualizer/src/playback.rs` by applying the correct trait bounds for `cpal::Sample`.
    -   **[ ] Verify Playback:** Load a WAV file and confirm that it plays correctly through the system's default audio device.
    -   **[ ] Verify Visualization:** Confirm that the ProjectM visualizer is receiving the audio data from `cpal` and is reacting to the music.

-   **[ ] Add Support for More Audio Formats:**
    -   **[ ] MP3 Support:** Add a crate like `minimp3` to handle MP3 decoding.
    -   **[ ] (Optional - MP3 is the only file the user uses currently with occasional WAV) FLAC Support:** Add a crate like `claxon` to handle FLAC decoding.
    -   **[ ] (Optional - OGG is less commonly used by people. Can be moved to the end of the long term roadmap alongside FLAC) OGG Support:** Add a crate like `lewton` to handle OGG decoding.

-   **[ ] Implement Playback Controls:**
    -   **[ ] Play/Pause/Stop:** Add UI buttons and logic to control the `cpal` audio stream.
    -   **[ ] Volume Control:** Add a volume slider to the UI.  USER EDIT NOTE: This is primarily not a music player but rather a music video recorded with projectm visaulizing audio playback as a means to "advertise" the users songs on social media with the animated textual elements over the projectm visualizer background. If recording mode is activated then the users settings should be honored and only the users favorite (if any have been favorited) visualizers should be used in record mode)

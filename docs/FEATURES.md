# Feature Breakdown

This document provides a detailed breakdown of the features required for the Aurora Visualizer RS project.

## Audio Playback

-   **Supported Formats:** MP3, FLAC, WAV, OGG.
-   **Playback Controls:**
    -   Play / Pause Toggle
    -   Stop
    -   Next Track
    -   Previous Track
    -   Volume Slider
-   **Playlist / Queue:**
    -   Add files via a file dialog.
    -   Remove files from the queue.
    -   Reorder files in the queue.
    -   Display the current queue in the UI.
    -   Automatic playback of the next track when the current one finishes.
-   **Shuffle Mode:** Toggle random playback of the queue.

## ProjectM Integration

-   **Preset Switching:**
    -   "Next (Random) Visualizer" button/hotkey.
    -   "Previous Visualizer" button/hotkey.
-   **Favorites System:**
    -   Button/hotkey to toggle the "favorite" status of the current preset.
    -   A "Favorites Only" mode for playback, where only favorite presets are shuffled.
    -   Favorites list is persisted in `$HOME/.config/aurora-visualizer-rs/favorites.txt`.
-   **Broken Preset Detection (Blocklist):**
    -   Automatically detect presets that fail to load by monitoring stdout for specific error messages.
    -   Add the file paths of broken presets to a blocklist.
    -   The blocklist is persisted in `$HOME/.config/aurora-visualizer-rs/blocklist.txt`.
    -   Blocked presets are excluded from the random preset pool.

## Recording Mode

-   **Toggle Recording:** A button/hotkey to start and stop recording.
-   **Temporary File Storage:** Recordings are temporarily stored in `/tmp` to avoid filling up user storage.
-   **Output:**
    -   The final output is a video file (e.g., MP4).
    -   The video combines the visualizer output with the audio from the currently playing track.
    -   The video includes the rendered text overlays.
-   **Configuration:**
    -   Option to specify the output directory for saved videos.
    -   Option to configure the recording duration (e.g., full track, percentage of track).

## Text Rendering & Animation

-   **Configuration:**
    -   A dedicated settings window for all text-related options.
    -   All settings are persisted in `$HOME/.config/aurora-visualizer-rs/`.
-   **Text Elements:**
    -   **Song Title:**
        -   Source: ID3 tags or sanitized filename.
        -   Automatic wrapping based on viewport width.
    -   **Artist Name:**
        -   Source: ID3 tags or a custom, user-defined string.
    -   **Custom URL:**
        -   A user-defined URL or social media handle.
-   **Styling:**
    -   Independent control over font, size, and RGBA color for each text element.
    -   Font chooser with a preview.
-   **Layout:**
    -   Option to join the song title and artist into a single string.
    -   The custom URL is displayed in a corner of the viewport.
-   **Animation:**
    -   Initial animation style: bouncing off the edges of the viewport.
    -   Configurable animation speed and duration.
    -   The custom URL is not animated.

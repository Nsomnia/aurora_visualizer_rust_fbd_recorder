# Project Roadmap

This document outlines the major development phases for the Aurora Visualizer RS project.

## Phase 1: Core Functionality (In Progress)

-   **[ ] Implement Robust Audio Playback:**
    -   Support for common audio formats (MP3, FLAC, WAV).
    -   Stable playback with standard controls (Play, Pause, Stop, Volume).
    -   A functional audio queue/playlist.
-   **[ ] Integrate ProjectM Visualizer:**
    -   Ensure the visualizer correctly receives and reacts to the audio stream.
    -   Implement preset switching (Next/Previous/Random).
-   **[ ] Basic UI:**
    -   A functional UI for loading audio files and controlling playback.
    -   Visible controls for preset switching.

## Phase 2: Advanced Features

-   **[ ] Favorites & Blocklist System:**
    -   Allow users to mark presets as "favorites".
    -   Automatically detect and blocklist broken presets.
    -   Persist these lists to the user's config directory.
-   **[ ] Hotkey Implementation:**
    -   Implement keyboard shortcuts for all major actions (playback control, preset switching, etc.).
-   **[ ] Recording Mode:**
    -   Implement a feature to record the visualizer output as a video file.
    -   Combine the recorded video with the playing audio.

## Phase 3: Text Rendering & Animation

-   **[ ] Advanced Text Overlay:**
    -   Render song title, artist, and a custom URL on the visualizer.
    -   Allow customization of fonts, colors, and sizes.
    -   Parse metadata from audio files or use custom strings.
-   **[ ] Text Animation:**
    -   Implement basic animation styles for the text overlays (e.g., bouncing off the viewport edges).
-   **[ ] Advanced Configuration UI:**
    -   Create a dedicated settings panel for customizing text rendering and animation options.

## Phase 4: Refinement & Polish

-   **[ ] Cross-Platform Testing & Compatibility:**
    -   Ensure the application runs smoothly on Linux, Windows, and macOS.
-   **[ ] Performance Optimization:**
    -   Profile and optimize the application for efficient resource usage.
-   **[ ] Code Cleanup & Documentation:**
    -   Refactor and document the codebase for long-term maintainability.

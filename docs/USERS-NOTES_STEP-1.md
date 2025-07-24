> API limit reached before finishing the last task.                                                                              │
│                                                                                                                                   │
│    Incase this happens again start your work by taking the following users notes about features and goals and organize them into  │
│    either existing or new documents in an appropriate folder(s) formatted to best be parsed by you and only you in agentic coding │
│     mode. This may require touching a blank file in the project root to remind yourself and future agentic calls to you to both   │
│    follow these, and keep them updated.                                                                                           │
│                                                                                                                                   │
│    Heres the notes I wrote down now so that we have a better grasp of the overall goals, features, and functionality:             │
│                                                                                                                                   │
│    ---                                                                                                                            │
│                                                                                                                                   │
│                                                                                                                                   │
│    We are trying to get this rust program to be a very basic barebones audio (mp3, flac, wav, etc) player with ability to toggle  │
│    play/pause, shuffle, a single audio file basic playlist that allows adding removing and organizing files in a list, the        │
│    ability to switch projectm audio visualizers presets (core implementation working enough for testing) to a random .milk shader │
│     file from the projectm directory /usr/share/projectM/presets/ when a keyboard key or UI button is triggered for "next         │
│    (random) visaulizer", detection of any broken or otherwise inoperatable .milk visualizer shader filders by looking for errors  │
│    in stdout that follow a predictable format and adding those to a persistant ignore list/db saved in the users                  │
│    $HOME/.config/aurora-visualizer-rs/<some_file_name> file, ability to press a keyboard key to toggle the currentyly used .milk  │
│    projectm visualizer shader file by adding and removing it from the list                                                        │
│                                                                                                                                   │
│    - ability to both playback the audio and view the projectm visualizer viewport as normal operations but also allow toggling a  │
│    recording mode where:                                                                                                          │
│    - recording is done to a file or folder in /tmp to keep resources safe and efficient                                           │
│    - the audio file should play back fully (or optionally a configurable percentage of the entire audio file) to then take the    │
│    recorded visualizer viewport video with the audio playing and text rendering (see below) recorded temporary file and save it   │
│    somewhere for the user to watch or post on social media                                                                        │
│                                                                                                                                   │
│    ---                                                                                                                            │
│                                                                                                                                   │
│    **TEXT RENDERING:**                                                                                                            │
│    - in the top menu bar "options" -> "advanced settings" window when opened there should be a settings page or tertiary window   │
│    where the user can set strings and toggle all elements and features on/off and customize things such as colors (RGBA values),  │
│    fonts, animation speeds, animations durations, toggling textual elements on/off, and again all saved persistantly (a highly    │
│    structured set of files of the appropriate filetype and structure will likely need to be held within the project holding       │
│    defaults that are overridden by an identical file the user can customize in the users home config folder where we are storing  │
│    nother data as previously mentioned.                                                                                           │
│    - Initally the textual elements are the songs title and artist name with an option to join them into one single string, or to  │
│    allow them to be separated so that the songs title and artist are separate elements in te viewport for animations etcetera, as │
│     well as a social media link such as a youtube or facebook page.                                                               │
│    - The url should default to being in one corner in a smaller font size than the other elements with no effects applied to it   │
│    other than a configurable RGBA value.                                                                                          │
│    - The artist name string should be able to have an option to be gotten from the songs ID tags or set to a persistant custom    │
│    string by the end user in which case the "Artist" ID3/ID4 tag is ignored. It should be able to have a font, font size, and     │
│    RGBA value, configured and saved between sessions.                                                                             │
│    - The audio name (song title) should be able to be configured by the user selecting from doing text sanitization on the file   │
│    name to convert non standard characters like underscores/asterisks/square brackets into a cleaner to display and parse string  │
│    and have a length factor that will split this title into multiple rows vertically if it ends up being bigger than the currrent │
│     width of the visualizer viewport, i.e. if one line or any individual strings line takes up more than 33% of the current       │
│    viewport width then it will be split into 2 strings as close to similer size as possible. The user should also be able to      │
│    choose having the song title name parsed from ID3/ID4 tags instead.                                                            │
│    - An option to have the song name and artist name strings joined as one string with a clear unicode large circle or other      │
│    character separating them, or to have both strings separate elements inside the viewport independant of them.                  │
│    - The artist name and song title name strings should each be able to have different RGBA values and fonts selected, especially │
│     important if acted on separatly for the animations.                                                                           │
│    - Animations: The artist name and song title strings should have multiple animation styles that can be toggled on and off to   │
│    change on a certain timeframe. For simplicity we will start with just having the strings reflect or bounce off the projectm    │
│    visualizer viewport. The users custom smaller text URL is not animated, though maybe in the future we will have it change      │
│    viewport corners at a certain period of time.                                                                                  │
│    - Custom fonts should be allowed for all elements using a font choose with preview, whether a system dialog, qt library, or    │
│    rust crate.                                                                                                                    │
│                                                                                                                                   │
│    ---                                                                                                                            │
│                                                                                                                                   │
│    Did you include all of these in the planning, todo, and future goals, docs?                                                    │
│                                                                                                                                   │
│    If you answered yes to the previous inquiery after then we are currently working on getting the audio playback basics          │
│    implemented so that its like a basic audio player package that has the projectm viewport reacting to the music as the inital   │
│    step. Having the default projectm visualizer be displayed as the logic currently has occur is the right approach because then  │
│    we can debug the biggest problem: loading audio files and playing them results in no sound output nor projectm visualizer      │
│    reacting to any audio stream even if it was silent.                                                                            │
│                                                                                                                                   │
│    You are free to both search github.com to find libraries to use or projects to borrow logic from, sources of rust projects and │
│     libraries such as crates.io website especially with the search string as this https://crates.io/search?q=alsa&sort=downloads  │
│    with a downloads sort tag appended to get more likely easy to use means of certain logic, or your own implementations.         │
│                                                                                                                                   │
│    You write documentation that is laid out to be easily parsed and updated by only yourself as the gemini-2.5-pro model and you  │
│    write code that is laid out best for you to parse and fix issues in or to expand and refactor. Be as mindful as possible as    │
│    you can about token usage to keep API usage to a minimum, especially output tokens as we only have thousands (4 to 24 thouand) │
│     to work with each day. Meaning do not give any notes or findings to the user unless you need them to know or respond to       │
│    something instead only using what you need to operate in agentic mode. This also means do not get running away completing      │
│    tasks and side projects of your own imagination/creation but stay on one task and check that you've completed it and the user  │
│    has responded that it is complete.                                                                                             │
│                                                                                                                                   │
│    You may want to write a second GEMINI.md with only the most important of these ideas in it and move the old on to the backups/ │
│     directory. This reminded the user to never run `rm` or `rm -r` nor any other destructive commands and this project isnt under │
│     source control until we have the barebone features operating making this more important.                                      │
│                                                                                                                                   │
│    Did you both assess all of the users ideas, thoughts, needs, concerns, and what steps to take to complete the core step after  │
│    documenting this projects immediate road-map?                                                                  
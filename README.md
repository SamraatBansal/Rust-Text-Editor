# Yet Another Text Editor

This is a Terminal based App implemented using Rust

Using CrossTerm crate to get easy access of the Raw Terminal

#Features

- MVP
    Cursor Movment
    Enter/Edit Text in file by Reading user input
    Prompt Message Bar


- Control Functions
    CTRL+Q -> Quit the file, If you have unsaved changes - Repeat 3 times (Configurable in QUIT_TIMES constant)
    CTRL+S -> Save the file, If its a new file - Implements Save As, and ESC to come out of the prompt.
    CTRL+N -> Create new instance of the terminal with new file

- Utility
    PageUp
    PageDn
    home - Line Beginning
    end - Line End


To use, clone the repository and execute
```
cargo run --release
```
You can also pass a file name as an argument. For instance
```
cargo run --release /src/main.rs


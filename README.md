![Tetrs Demo](demos/tetrs_demo.gif)
# Tetrs (tetrs-tui)
- A modern Tetris clone for your terminal, written in Rust with a responsive TUI and optional soundtrack.
- Check out the [crates.io page](https://crates.io/crates/tetrs-tui).
- Uses the [Cursive TUI library](https://github.com/gyscos/cursive) and [Rodio](https://github.com/RustAudio/rodio) for audio.
- Music synthesized from MIDI from [bitmidi.com](https://bitmidi.com/).

## Features
- Runs on Linux, MacOS, and Windows
- Beautiful TUI graphics
- Difficulty scaling
- Piece holding 
- Toggleable music
- Toggleable ghost piece
- Persistent highscore

## Install
```bash
cargo install tetrs-tui
```
## Run 
```bash
cargo run --package tetrs-tui
```
or, if your Cargo binaries are on your path: 
```bash
tetrs
```

## License
- Distributed under the [MIT License](LICENSE).

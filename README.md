# Stopwatch TUI

A terminal-based stopwatch application built with Rust and Ratatui, featuring lap timing, pause/resume functionality, and an intuitive keyboard interface.

## Features

- ⏱️ **Precise timing** with millisecond accuracy
- 🏃 **Lap recording** with individual and cumulative times
- ⏸️ **Pause/Resume** functionality
- 🔄 **Reset** to start over
- ⌨️ **Scrollable lap history** with arrow key navigation
- 🎨 **Color-coded timer** that changes based on elapsed time
- 📱 **Clean TUI interface** that works in any terminal

## Installation

### Prerequisites

- [Rust](https://rustup.rs/) (latest stable version)

### Build from source

```bash
git clone https://github.com/andrewdkennedy1/stopwatch-rs.git
cd stopwatch-rs
cargo build --release
```

### Run directly

```bash
cargo run
```

## Usage

### Controls

| Key | Action |
|-----|--------|
| `SPACE` | Record a lap |
| `P` | Pause/Resume the timer |
| `R` | Reset the stopwatch |
| `↑` / `↓` | Scroll through lap history |
| `Q` / `ESC` | Quit the application |

### Interface

The application displays:

1. **Title bar** - Application name
2. **Main timer** - Current elapsed time with status indicator
3. **Controls** - Available keyboard shortcuts
4. **Lap list** - Scrollable history of recorded laps

### Timer Colors

The main timer changes color based on elapsed time:
- 🟢 **Green**: 0-10 seconds
- 🟡 **Yellow**: 10-60 seconds  
- 🔵 **Cyan**: 1-5 minutes
- 🟣 **Magenta**: 5+ minutes

## Examples

### Basic Usage

1. Start the application: `cargo run`
2. The timer begins automatically
3. Press `SPACE` to record laps
4. Use `↑`/`↓` to scroll through your lap history
5. Press `P` to pause/resume
6. Press `R` to reset everything
7. Press `Q` to quit

### Lap Display Format

```
Lap  1: 12.34s  (Total: 12.34s)
Lap  2: 15.67s  (Total: 28.01s)
Lap  3: 11.23s  (Total: 39.24s)
```

Each lap shows:
- **Lap number**: Sequential numbering
- **Lap time**: Time since the last lap
- **Total time**: Cumulative time from start

## Dependencies

- [ratatui](https://github.com/ratatui-org/ratatui) - Terminal UI framework
- [crossterm](https://github.com/crossterm-rs/crossterm) - Cross-platform terminal manipulation

## Development

### Project Structure

```
src/
├── main.rs          # Main application logic and UI
└── ...

Cargo.toml           # Project dependencies and metadata
README.md           # This file
```

### Building

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Run with cargo
cargo run

# Run tests
cargo test
```

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is open source and available under the [MIT License](LICENSE).

## Acknowledgments

- Built with [Ratatui](https://ratatui.rs/) - An excellent Rust TUI framework
- Inspired by classic stopwatch applications with modern terminal UI design
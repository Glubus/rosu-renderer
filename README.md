# rosu-renderer

A Rust library for rendering osu! beatmaps using egui. Currently supports mania mode with customizable note styles and real-time playback.

## Features

### 🎵 Mania Mode Support
- **Real-time beatmap rendering** with customizable playback speed
- **Multiple note shapes**: Circle, Rectangle, Arrow, and custom images
- **Hold note support** with customizable body and cap colors
- **Adjustable column width and note size**
- **Scroll speed control** for different gameplay preferences

### 🎨 Customization
- **Note styling**: Choose from predefined shapes or load custom images
- **Color customization**: Set note colors, hold body colors, and hold cap colors
- **Layout control**: Adjust column width and note size to match your preferences
- **Playback controls**: Control playback speed and scroll timing

### 🖥️ UI Framework
- Built with **egui** for immediate mode GUI
- Cross-platform support
- Real-time rendering with smooth performance

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
rosu-renderer = "1.1.1"
eframe = "0.32.0"
egui = "0.32.0"
egui_extras = { version = "0.32.0", features = ["image", "file"] }
rosu-map = "0.2.1"
```

## Quick Start

### Basic Usage

```rust
use rosu_renderer::{Player, layout::mania::{NoteStyle, NoteShape}};
use rosu_map::Beatmap;

// Load your beatmap
let beatmap = Beatmap::from_path("path/to/your/map.osu").unwrap();

// Create a player with custom dimensions
let mut player = Player::new(beatmap, 80.0, 60.0, 800.0).unwrap();

// Customize note style
let style = NoteStyle {
    shape: NoteShape::Circle,
    color: egui::Color32::from_rgb(0, 174, 255),
    hold_body_color: egui::Color32::from_rgb(200, 200, 200),
    hold_cap_color: egui::Color32::from_rgb(0, 174, 255),
};
player.set_note_style(style);

// Set playback speed
player.set_speed(1.5);

// Set scroll speed (in milliseconds)
player.set_scroll_time(1000.0);

// In your egui app update loop:
player.render(ui);
```

### Running the Example

```bash
cargo run --example mania
```

The example provides a full-featured mania mode viewer with:
- File selection for beatmaps
- Real-time playback controls
- Note style selection
- Custom image loading
- Adjustable layout parameters

## API Reference

### Player

The main struct for rendering beatmaps.

```rust
pub struct Player {
    // ... internal fields
}

impl Player {
    // Create a new player instance
    pub fn new(beatmap: Beatmap, column_width: f32, note_size: f32, height: f32) -> Option<Self>
    
    // Set the note style
    pub fn set_note_style(&mut self, style: NoteStyle)
    
    // Get required window size
    pub fn get_required_size(&self) -> [f32; 2]
    
    // Set playback speed multiplier
    pub fn set_speed(&mut self, speed: f64)
    
    // Set scroll time in milliseconds
    pub fn set_scroll_time(&mut self, ms: f32)
    
    // Render the beatmap
    pub fn render(&mut self, ui: &mut egui::Ui)
    
    // Reset playback time
    pub fn reset_time(&mut self)
    
    // Set current playback time
    pub fn set_current_time(&mut self, time_ms: f64)
    
    // Get current playback time
    pub fn current_time(&self) -> f64
}
```

### NoteStyle

Customize the appearance of notes.

```rust
pub struct NoteStyle {
    pub shape: NoteShape,
    pub color: Color32,
    pub hold_body_color: Color32,
    pub hold_cap_color: Color32,
}

pub enum NoteShape {
    Circle,
    Rectangle { width: f32, height: f32 },
    Arrow { width: f32, height: f32 },
    Image(egui::Image<'static>),
}
```

## Supported Game Modes

Currently, only **osu!mania** mode is supported. Support for other modes (Standard, Taiko, Catch) is planned for future releases.

## Dependencies

- **eframe**: egui application framework
- **egui**: Immediate mode GUI library
- **egui_extras**: Additional egui features (image loading, file dialogs)
- **rosu-map**: osu! beatmap parsing library
- **image**: Image processing for custom note textures

## Development

### Building

```bash
cargo build
```

### Running Tests

```bash
cargo test
```

### Running Examples

```bash
# Run the mania example
cargo run --example mania
```

## Contributing

Contributions are welcome! Areas that need work:

- [ ] Standard mode support
- [ ] Taiko mode support  
- [ ] Catch mode support
- [ ] Additional note styles
- [ ] Audio synchronization
- [ ] Performance optimizations

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with [egui](https://github.com/emilk/egui) for the UI framework
- Uses [rosu-map](https://github.com/rosu-rs/rosu-map) for beatmap parsing
- Inspired by the osu! game and community

## Roadmap

- [ ] Add support for Standard mode
- [ ] Add support for Taiko mode
- [ ] Add support for Catch mode
- [ ] Implement audio synchronization
- [ ] Add more note styles and effects
- [ ] Performance optimizations for large beatmaps
- [ ] Export functionality for rendered beatmaps 
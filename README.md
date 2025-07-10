# Windows Terminal Profiles Rust Library

This is a Rust library for parsing and managing Windows Terminal configuration files. It provides comprehensive serde structs to serialize and deserialize the Windows Terminal `profiles.json` configuration file.

[Windows Terminal Profiles Schema](https://aka.ms/terminal-profiles-schema)

## Installation

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
terminal-profiles = "0.1.0"
```

## Quick Start

### Basic Usage

```rust
use terminal_profiles::WindowsTerminalSettings;
let s = std::fs::read_to_string("settings.json").unwrap();
let profiles: WindowsTerminalSettings = serde_json::from_str(&s).unwrap();
```

## Main Structs

### WindowsTerminalSettings

The primary configuration struct, containing all Windows Terminal settings:

- `default_profile`: GUID of the default profile
- `profiles`: List of profiles
- `schemes`: List of color schemes
- `actions`: Custom actions
- `keybindings`: Keybinding mappings
- `themes`: Theme configurations

### Profile

Struct for a single profile:

- `guid`: Unique identifier
- `name`: Profile name
- `commandline`: Startup command
- `starting_directory`: Starting directory
- `color_scheme`: Color scheme
- `font`: Font configuration
- `background`: Background settings
- `cursor_shape`: Cursor shape

### ColorScheme

Color scheme struct:

- `name`: Scheme name
- `background`: Background color
- `foreground`: Foreground color
- `black`, `red`, `green`, `blue`, `yellow`, `purple`, `cyan`, `white`: Base colors
- `bright_*`: Bright colors

## Helper Methods

### WindowsTerminalSettings Methods

- `from_json(json: &str)`: Create configuration from JSON string
- `to_json() -> Result<String, serde_json::Error>`: Convert to JSON string
- `get_default_profile() -> Option<&Profile>`: Get the default profile
- `get_all_profiles() -> &[Profile]`: Get all profiles
- `find_profile_by_guid(guid: &str) -> Option<&Profile>`: Find profile by GUID
- `find_profile_by_name(name: &str) -> Option<&Profile>`: Find profile by name
- `find_scheme_by_name(name: &str) -> Option<&ColorScheme>`: Find color scheme by name

### Profile Methods

- `is_hidden() -> bool`: Check if the profile is hidden
- `get_font_face() -> Option<&str>`: Get font face
- `get_font_size() -> Option<f64>`: Get font size
- `get_font_weight() -> Option<&FontWeight>`: Get font weight

### ColorScheme Methods

- `get_all_colors() -> HashMap<&str, &str>`: Get all colors

## Enum Types

The library provides rich enum types for type safety:

- `LaunchMode`: Launch mode (fullscreen, maximized, default, etc.)
- `CursorShape`: Cursor shape (bar, block, underline, etc.)
- `FontWeight`: Font weight
- `BackgroundImageAlignment`: Background image alignment
- `CloseOnExit`: Close behavior
- `TabWidthMode`: Tab width mode
- `ShortcutActionName`: Shortcut action name

## Examples

Run the example program:

```bash
cargo run
```

Run tests:

```bash
cargo test
```

## License

MIT License

## Contributing

Issues and Pull Requests are welcome!

## Related Links

- [Windows Terminal Official Documentation](https://docs.microsoft.com/en-us/windows/terminal/)
- [Windows Terminal Configuration Documentation](https://docs.microsoft.com/en-us/windows/terminal/customize-settings/overview)
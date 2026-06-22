pub mod action;
pub mod color_scheme;
pub mod enums;
pub(crate) mod macros;
pub mod new_tab_menu;
pub mod profile;
pub mod theme;

// Re-export commonly used types
pub use action::{Action, Keybinding, SubCommand};
pub use color_scheme::{ColorScheme, ColorSchemeValue, SchemePair};
pub use enums::*;
pub use new_tab_menu::{
    ActionEntry, FolderEntry, MatchProfilesEntry, NewTabMenuEntry, ProfileEntry,
    RemainingProfilesEntry, SeparatorEntry,
};
pub use profile::{AppearanceConfig, FontConfig, Profile, ProfilesObject, ProfilesValue};
pub use theme::{TabRowTheme, TabTheme, Theme, ThemeColor, ThemePair, ThemeValue, WindowTheme};

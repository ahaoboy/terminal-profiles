use serde::{Deserialize, Serialize};

pub mod types;

use types::{
    Action, ColorScheme, CopyFormat, DynamicProfileSource, FirstWindowPreference, Keybinding,
    LaunchMode, NewTabMenuEntry, NewTabPosition, ProfilesValue, TabSwitcherMode, TabWidthMode,
    Theme, ThemeValue, WindowingBehavior,
};

/// Root settings object for Windows Terminal
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WindowsTerminalSettings {
    #[serde(rename = "$help")]
    pub help: String,
    #[serde(rename = "$schema")]
    pub schema: String,

    pub default_profile: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub always_on_top: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub always_show_tabs: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_on_select: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focus_follow_mouse: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_formatting: Option<CopyFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_animations: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub large_paste_warning: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_line_paste_warning: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub startup_actions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_cols: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_rows: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_position: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_mode: Option<LaunchMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_window_preference: Option<FirstWindowPreference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_tabs_in_titlebar: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_terminal_title_in_titlebar: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tab_width_mode: Option<TabWidthMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub word_delimiters: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirm_close_all_tabs: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tab_switcher_mode: Option<TabSwitcherMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub windowing_behavior: Option<WindowingBehavior>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_tab_position: Option<NewTabPosition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_hide_window: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trim_block_selection: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trim_paste: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub center_on_launch: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimize_to_notification_area: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub always_show_notification_icon: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snap_to_grid_on_resize: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_admin_shield: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_acrylic_in_tab_row: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_profile_sources: Option<Vec<DynamicProfileSource>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme: Option<ThemeValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub themes: Option<Vec<Theme>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<Action>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keybindings: Option<Vec<Keybinding>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_tab_menu: Option<Vec<NewTabMenuEntry>>,
    pub profiles: ProfilesValue,
    pub schemes: Vec<ColorScheme>,
}

#[cfg(test)]
mod test {
    use serde_json::Value;

    use crate::WindowsTerminalSettings;

    #[test]
    fn roundtrip_serialization() {
        let s = include_str!("../assets/settings.json");
        let json: WindowsTerminalSettings = serde_json::from_str(s).unwrap();
        let s2 = serde_json::to_string(&json).unwrap();
        let v1: Value = serde_json::from_str(s).unwrap();
        let v2: Value = serde_json::from_str(&s2).unwrap();
        assert_eq!(v1, v2);
    }
}

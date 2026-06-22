use serde::{Deserialize, Serialize};

use super::enums::{ApplicationTheme, IconStyle, ShowCloseButton, SpecialThemeColor};
use super::macros::impl_untagged_default;

/// UI theme definition
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Theme {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tab: Option<TabTheme>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tab_row: Option<TabRowTheme>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window: Option<WindowTheme>,
}

/// Tab appearance theme
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TabTheme {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<ThemeColor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unfocused_background: Option<ThemeColor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_close_button: Option<ShowCloseButton>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_style: Option<IconStyle>,
}

/// Tab row (background bar) theme
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TabRowTheme {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<ThemeColor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unfocused_background: Option<ThemeColor>,
}

/// Window chrome theme
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct WindowTheme {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_theme: Option<ApplicationTheme>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_mica: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame: Option<ThemeColor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unfocused_frame: Option<ThemeColor>,
}

/// Theme color: either a hex/name string or a special semantic value
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ThemeColor {
    String(String),
    Special(SpecialThemeColor),
}

impl_untagged_default!(ThemeColor, String, String::new());

/// Global `theme` field value: either a theme name string or a light/dark pair
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ThemeValue {
    String(String),
    ThemePair(ThemePair),
}

impl_untagged_default!(ThemeValue, String, String::new());

/// Light / dark theme pair
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ThemePair {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub light: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dark: Option<String>,
}

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::color_scheme::ColorSchemeValue;
use super::enums::{
    AdjustIndistinguishableColors, AntialiasingMode, BackgroundImageAlignment,
    BackgroundImageStretchMode, BellSound, BellStyle, CloseOnExit, CursorShape, FontWeight,
    IntenseTextStyle, PathTranslationStyle, ScrollbarState,
};
use super::macros::impl_untagged_default;

/// A single Windows Terminal profile entry (inside the profiles object/list)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Profile {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commandline: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_directory: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tab_title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suppress_application_title: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tab_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_on_exit: Option<CloseOnExit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_image_alignment: Option<BackgroundImageAlignment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_image_opacity: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_image_stretch_mode: Option<BackgroundImageStretchMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor_shape: Option<CursorShape>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor_height: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foreground: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_background: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_scheme: Option<ColorSchemeValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opacity: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_acrylic: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font: Option<FontConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_face: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_size: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_weight: Option<FontWeight>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub antialiasing_mode: Option<AntialiasingMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub history_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snap_on_input: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt_gr_aliasing: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bell_style: Option<BellStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bell_sound: Option<BellSound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intense_text_style: Option<IntenseTextStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjust_indistinguishable_colors: Option<AdjustIndistinguishableColors>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scrollbar_state: Option<ScrollbarState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_translation_style: Option<PathTranslationStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elevate: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_mark_prompts: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_marks_on_scrollbar: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unfocused_appearance: Option<AppearanceConfig>,
}

/// The `profiles` field: either a plain array (legacy) or an object with defaults + list
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ProfilesValue {
    List(Vec<Profile>),
    Object(Box<ProfilesObject>),
}

impl_untagged_default!(ProfilesValue, List, Vec::new());

/// Object-style profiles container with optional defaults
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ProfilesObject {
    pub list: Vec<Profile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defaults: Option<Profile>,
}

/// Font face, size, weight and OpenType feature/axis configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct FontConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<FontWeight>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<HashMap<String, i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub axes: Option<HashMap<String, f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cell_width: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cell_height: Option<String>,
}

/// Appearance overrides applied when the terminal window is unfocused
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct AppearanceConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_scheme: Option<ColorSchemeValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foreground: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_background: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor_shape: Option<CursorShape>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor_height: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_image_opacity: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_image_stretch_mode: Option<BackgroundImageStretchMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_image_alignment: Option<BackgroundImageAlignment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intense_text_style: Option<IntenseTextStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjust_indistinguishable_colors: Option<AdjustIndistinguishableColors>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_acrylic: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opacity: Option<f64>,
}

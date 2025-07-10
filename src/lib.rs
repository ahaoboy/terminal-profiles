use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ProfilesValue {
    List(Vec<Profile>),
    // TODO: reduce enum size
    Object(Box<ProfilesObject>),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProfilesObject {
    pub list: Vec<Profile>,
    pub defaults: Option<Profile>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ColorScheme {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub black: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blue: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bright_black: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bright_blue: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bright_cyan: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bright_green: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bright_purple: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bright_red: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bright_white: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bright_yellow: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cyan: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foreground: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub green: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purple: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub red: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_background: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub white: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yellow: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ColorSchemeValue {
    String(String),
    SchemePair(SchemePair),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SchemePair {
    pub light: Option<String>,
    pub dark: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Theme {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tab: Option<TabTheme>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tab_row: Option<TabRowTheme>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window: Option<WindowTheme>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TabRowTheme {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<ThemeColor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unfocused_background: Option<ThemeColor>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ThemeColor {
    String(String),
    Special(SpecialThemeColor),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Action {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<CommandValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<CommandName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iterate_on: Option<IterateOn>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commands: Option<Vec<SubCommand>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SubCommand {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<CommandValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<CommandName>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Keybinding {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub keys: KeyChordValue,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum NewTabMenuEntry {
    Folder(FolderEntry),
    Separator(SeparatorEntry),
    Profile(ProfileEntry),
    MatchProfiles(MatchProfilesEntry),
    RemainingProfiles(RemainingProfilesEntry),
    Action(ActionEntry),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FolderEntry {
    #[serde(rename = "type")]
    pub entry_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    pub entries: Vec<NewTabMenuEntry>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline: Option<FolderEntryInlining>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_empty: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SeparatorEntry {
    #[serde(rename = "type")]
    pub entry_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProfileEntry {
    #[serde(rename = "type")]
    pub entry_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MatchProfilesEntry {
    #[serde(rename = "type")]
    pub entry_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commandline: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RemainingProfilesEntry {
    #[serde(rename = "type")]
    pub entry_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ActionEntry {
    #[serde(rename = "type")]
    pub entry_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

// ========== 枚举类型 ==========

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum CopyFormat {
    Boolean(bool),
    Array(Vec<CopyFormatType>),
    String(CopyFormatType),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum LaunchMode {
    Fullscreen,
    Maximized,
    Default,
    Focus,
    MaximizedFocus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum FirstWindowPreference {
    DefaultProfile,
    PersistedWindowLayout,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum TabWidthMode {
    Equal,
    TitleLength,
    Compact,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum TabSwitcherMode {
    Mru,
    InOrder,
    Disabled,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum WindowingBehavior {
    UseNew,
    UseExisting,
    UseAnyExisting,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum NewTabPosition {
    AfterLastTab,
    AfterCurrentTab,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum CloseOnExit {
    Never,
    Graceful,
    Always,
    Automatic,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum BackgroundImageAlignment {
    Bottom,
    BottomLeft,
    BottomRight,
    Center,
    Left,
    Right,
    Top,
    TopLeft,
    TopRight,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum BackgroundImageStretchMode {
    Fill,
    None,
    Uniform,
    UniformToFill,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum CursorShape {
    Bar,
    DoubleUnderscore,
    EmptyBox,
    FilledBox,
    Underscore,
    Vintage,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum FontWeight {
    String(FontWeightString),
    Integer(i32),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum AntialiasingMode {
    Grayscale,
    Cleartype,
    Aliased,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum BellStyle {
    Boolean(bool),
    Array(Vec<BellStyleType>),
    String(BellStyleType),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum BellSound {
    String(String),
    Array(Vec<String>),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum IntenseTextStyle {
    None,
    Bold,
    Bright,
    All,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum AdjustIndistinguishableColors {
    Never,
    Indexed,
    Always,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum ScrollbarState {
    Visible,
    Hidden,
    Always,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum PathTranslationStyle {
    None,
    Wsl,
    Cygwin,
    Msys2,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ThemeValue {
    String(String),
    ThemePair(ThemePair),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ThemePair {
    pub light: Option<String>,
    pub dark: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum CommandValue {
    String(ShortcutActionName),
    Object(serde_json::Value), // 简化为通用对象
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum CommandName {
    String(String),
    Object(CommandNameObject),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CommandNameObject {
    pub key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum KeyChordValue {
    Single(String),
    Multiple(Vec<String>),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum FolderEntryInlining {
    Never,
    Auto,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum SpecialThemeColor {
    Accent,
    TerminalBackground,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum ShowCloseButton {
    Always,
    Hover,
    Never,
    ActiveOnly,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum IconStyle {
    Default,
    Hidden,
    Monochrome,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum ApplicationTheme {
    Light,
    Dark,
    System,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum IterateOn {
    Profiles,
    Schemes,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum CopyFormatType {
    Html,
    Rtf,
    All,
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum FontWeightString {
    Thin,
    ExtraLight,
    Light,
    SemiLight,
    Normal,
    Medium,
    SemiBold,
    Bold,
    ExtraBold,
    Black,
    ExtraBlack,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum BellStyleType {
    Audible,
    Window,
    Taskbar,
    All,
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum ShortcutActionName {
    AddMark,
    AdjustFontSize,
    AdjustOpacity,
    ClearAllMarks,
    ClearBuffer,
    ClearMark,
    CloseOtherPanes,
    CloseOtherTabs,
    ClosePane,
    CloseTab,
    CloseTabsAfter,
    CloseWindow,
    CommandPalette,
    Copy,
    DuplicateTab,
    ExpandSelectionToWord,
    #[serde(rename = "experimental.colorSelection")]
    ExperimentalColorSelection,
    ExportBuffer,
    Find,
    FindMatch,
    FocusPane,
    GlobalSummon,
    IdentifyWindow,
    IdentifyWindows,
    MarkMode,
    MoveFocus,
    MovePane,
    MoveTab,
    MultipleActions,
    NewTab,
    NewWindow,
    NextTab,
    OpenAbout,
    OpenNewTabDropdown,
    OpenSettings,
    OpenSystemMenu,
    OpenTabColorPicker,
    OpenTabRenamer,
    OpenWindowRenamer,
    Paste,
    PrevTab,
    QuakeMode,
    Quit,
    RenameTab,
    RenameWindow,
    ResetFontSize,
    ResizePane,
    RestoreLastClosed,
    ScrollDown,
    ScrollDownPage,
    ScrollToBottom,
    ScrollToMark,
    ScrollToTop,
    ScrollUp,
    ScrollUpPage,
    SearchWeb,
    SelectAll,
    SendInput,
    SetColorScheme,
    SetFocusMode,
    SetFullScreen,
    SetMaximized,
    SetTabColor,
    ShowSuggestions,
    SplitPane,
    SwapPane,
    SwitchSelectionEndpoint,
    SwitchToTab,
    TabSearch,
    ToggleAlwaysOnTop,
    ToggleBlockSelection,
    ToggleFocusMode,
    ToggleFullscreen,
    TogglePaneZoom,
    ToggleReadOnlyMode,
    ToggleShaderEffects,
    ToggleSplitOrientation,
    Wt,
    Unbound,
}

#[cfg(test)]
mod test {
    use crate::WindowsTerminalSettings;

    #[test]
    fn test() {
        let s = include_str!("../assets/settings.json");
        let json: WindowsTerminalSettings = serde_json::from_str(s).unwrap();
        let s2 = serde_json::to_string(&json).unwrap();
        let json2: WindowsTerminalSettings = serde_json::from_str(&s2).unwrap();
        assert_eq!(json, json2);
    }
}

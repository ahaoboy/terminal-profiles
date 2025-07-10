use serde::{Deserialize, Serialize};
use std::collections::HashMap;


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowsTerminalSettings {
    pub default_profile: String,
    pub always_on_top: Option<bool>,
    pub always_show_tabs: Option<bool>,
    pub copy_on_select: Option<bool>,
    pub focus_follow_mouse: Option<bool>,
    pub copy_formatting: Option<CopyFormat>,
    pub disable_animations: Option<bool>,
    pub large_paste_warning: Option<bool>,
    pub multi_line_paste_warning: Option<bool>,
    pub startup_actions: Option<String>,
    pub initial_cols: Option<i32>,
    pub initial_rows: Option<i32>,
    pub initial_position: Option<String>,
    pub launch_mode: Option<LaunchMode>,
    pub first_window_preference: Option<FirstWindowPreference>,
    pub show_tabs_in_titlebar: Option<bool>,
    pub show_terminal_title_in_titlebar: Option<bool>,
    pub tab_width_mode: Option<TabWidthMode>,
    pub word_delimiters: Option<String>,
    pub confirm_close_all_tabs: Option<bool>,
    pub tab_switcher_mode: Option<TabSwitcherMode>,
    pub windowing_behavior: Option<WindowingBehavior>,
    pub new_tab_position: Option<NewTabPosition>,
    pub auto_hide_window: Option<bool>,

    pub theme: Option<ThemeValue>,
    pub themes: Option<Vec<Theme>>,

    pub actions: Option<Vec<Action>>,
    pub keybindings: Option<Vec<Keybinding>>,

    pub new_tab_menu: Option<Vec<NewTabMenuEntry>>,

    pub profiles: ProfilesValue,

    pub schemes: Vec<ColorScheme>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ProfilesValue {
    List(Vec<Profile>),
    // TODO: reduce enum size
    Object(Box<ProfilesObject>),
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfilesObject {
    pub list: Vec<Profile>,
    pub defaults: Option<Profile>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
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


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FontConfig {
    pub face: Option<String>,
    pub size: Option<f64>,
    pub weight: Option<FontWeight>,
    pub features: Option<HashMap<String, i32>>,
    pub axes: Option<HashMap<String, f64>>,
    pub cell_width: Option<String>,
    pub cell_height: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppearanceConfig {
    pub color_scheme: Option<ColorSchemeValue>,
    pub foreground: Option<String>,
    pub background: Option<String>,
    pub selection_background: Option<String>,
    pub cursor_color: Option<String>,
    pub cursor_shape: Option<CursorShape>,
    pub cursor_height: Option<i32>,
    pub background_image: Option<String>,
    pub background_image_opacity: Option<f64>,
    pub background_image_stretch_mode: Option<BackgroundImageStretchMode>,
    pub background_image_alignment: Option<BackgroundImageAlignment>,
    pub intense_text_style: Option<IntenseTextStyle>,
    pub adjust_indistinguishable_colors: Option<AdjustIndistinguishableColors>,
    pub use_acrylic: Option<bool>,
    pub opacity: Option<f64>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorScheme {
    pub name: String,
    pub background: Option<String>,
    pub black: Option<String>,
    pub blue: Option<String>,
    pub bright_black: Option<String>,
    pub bright_blue: Option<String>,
    pub bright_cyan: Option<String>,
    pub bright_green: Option<String>,
    pub bright_purple: Option<String>,
    pub bright_red: Option<String>,
    pub bright_white: Option<String>,
    pub bright_yellow: Option<String>,
    pub cursor_color: Option<String>,
    pub cyan: Option<String>,
    pub foreground: Option<String>,
    pub green: Option<String>,
    pub purple: Option<String>,
    pub red: Option<String>,
    pub selection_background: Option<String>,
    pub white: Option<String>,
    pub yellow: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ColorSchemeValue {
    String(String),
    SchemePair(SchemePair),
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemePair {
    pub light: Option<String>,
    pub dark: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub name: String,
    pub tab: Option<TabTheme>,
    pub tab_row: Option<TabRowTheme>,
    pub window: Option<WindowTheme>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TabTheme {
    pub background: Option<ThemeColor>,
    pub unfocused_background: Option<ThemeColor>,
    pub show_close_button: Option<ShowCloseButton>,
    pub icon_style: Option<IconStyle>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TabRowTheme {
    pub background: Option<ThemeColor>,
    pub unfocused_background: Option<ThemeColor>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowTheme {
    pub application_theme: Option<ApplicationTheme>,
    pub use_mica: Option<bool>,
    pub frame: Option<ThemeColor>,
    pub unfocused_frame: Option<ThemeColor>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ThemeColor {
    String(String),
    Special(SpecialThemeColor),
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    pub command: Option<CommandValue>,
    pub icon: Option<String>,
    pub name: Option<CommandName>,
    pub id: Option<String>,
    pub iterate_on: Option<IterateOn>,
    pub commands: Option<Vec<SubCommand>>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubCommand {
    pub command: Option<CommandValue>,
    pub name: Option<CommandName>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Keybinding {
    pub id: Option<String>,
    pub keys: KeyChordValue,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NewTabMenuEntry {
    Folder(FolderEntry),
    Separator(SeparatorEntry),
    Profile(ProfileEntry),
    MatchProfiles(MatchProfilesEntry),
    RemainingProfiles(RemainingProfilesEntry),
    Action(ActionEntry),
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolderEntry {
    #[serde(rename = "type")]
    pub entry_type: String,
    pub name: Option<String>,
    pub icon: Option<String>,
    pub entries: Vec<NewTabMenuEntry>,
    pub inline: Option<FolderEntryInlining>,
    pub allow_empty: Option<bool>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeparatorEntry {
    #[serde(rename = "type")]
    pub entry_type: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileEntry {
    #[serde(rename = "type")]
    pub entry_type: String,
    pub profile: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchProfilesEntry {
    #[serde(rename = "type")]
    pub entry_type: String,
    pub name: Option<String>,
    pub source: Option<String>,
    pub commandline: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemainingProfilesEntry {
    #[serde(rename = "type")]
    pub entry_type: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionEntry {
    #[serde(rename = "type")]
    pub entry_type: String,
    pub id: Option<String>,
}

// ========== 枚举类型 ==========


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CopyFormat {
    Boolean(bool),
    Array(Vec<CopyFormatType>),
    String(CopyFormatType),
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum LaunchMode {
    Fullscreen,
    Maximized,
    Default,
    Focus,
    MaximizedFocus,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum FirstWindowPreference {
    DefaultProfile,
    PersistedWindowLayout,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TabWidthMode {
    Equal,
    TitleLength,
    Compact,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TabSwitcherMode {
    Mru,
    InOrder,
    Disabled,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum WindowingBehavior {
    UseNew,
    UseExisting,
    UseAnyExisting,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum NewTabPosition {
    AfterLastTab,
    AfterCurrentTab,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CloseOnExit {
    Never,
    Graceful,
    Always,
    Automatic,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
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


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BackgroundImageStretchMode {
    Fill,
    None,
    Uniform,
    UniformToFill,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CursorShape {
    Bar,
    DoubleUnderscore,
    EmptyBox,
    FilledBox,
    Underscore,
    Vintage,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FontWeight {
    String(FontWeightString),
    Integer(i32),
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AntialiasingMode {
    Grayscale,
    Cleartype,
    Aliased,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BellStyle {
    Boolean(bool),
    Array(Vec<BellStyleType>),
    String(BellStyleType),
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BellSound {
    String(String),
    Array(Vec<String>),
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum IntenseTextStyle {
    None,
    Bold,
    Bright,
    All,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AdjustIndistinguishableColors {
    Never,
    Indexed,
    Always,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ScrollbarState {
    Visible,
    Hidden,
    Always,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PathTranslationStyle {
    None,
    Wsl,
    Cygwin,
    Msys2,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ThemeValue {
    String(String),
    ThemePair(ThemePair),
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemePair {
    pub light: Option<String>,
    pub dark: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CommandValue {
    String(ShortcutActionName),
    Object(serde_json::Value), // 简化为通用对象
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CommandName {
    String(String),
    Object(CommandNameObject),
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandNameObject {
    pub key: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum KeyChordValue {
    Single(String),
    Multiple(Vec<String>),
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum FolderEntryInlining {
    Never,
    Auto,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SpecialThemeColor {
    Accent,
    TerminalBackground,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ShowCloseButton {
    Always,
    Hover,
    Never,
    ActiveOnly,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum IconStyle {
    Default,
    Hidden,
    Monochrome,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ApplicationTheme {
    Light,
    Dark,
    System,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum IterateOn {
    Profiles,
    Schemes,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CopyFormatType {
    Html,
    Rtf,
    All,
    None,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
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


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BellStyleType {
    Audible,
    Window,
    Taskbar,
    All,
    None,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
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

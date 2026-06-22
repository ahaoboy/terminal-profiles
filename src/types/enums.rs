use serde::{Deserialize, Serialize};

use super::macros::impl_untagged_default;

// ========== Standalone Enum Types ==========

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "camelCase")]
pub enum LaunchMode {
    Fullscreen,
    Maximized,
    #[default]
    Default,
    Focus,
    MaximizedFocus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "camelCase")]
pub enum FirstWindowPreference {
    #[default]
    DefaultProfile,
    PersistedWindowLayout,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "camelCase")]
pub enum TabWidthMode {
    #[default]
    Equal,
    TitleLength,
    Compact,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "camelCase")]
pub enum TabSwitcherMode {
    #[default]
    Mru,
    InOrder,
    Disabled,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "camelCase")]
pub enum WindowingBehavior {
    #[default]
    UseNew,
    UseExisting,
    UseAnyExisting,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "camelCase")]
pub enum NewTabPosition {
    #[default]
    AfterLastTab,
    AfterCurrentTab,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "camelCase")]
pub enum CloseOnExit {
    Never,
    Graceful,
    Always,
    #[default]
    Automatic,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "camelCase")]
pub enum BackgroundImageAlignment {
    Bottom,
    BottomLeft,
    BottomRight,
    #[default]
    Center,
    Left,
    Right,
    Top,
    TopLeft,
    TopRight,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "camelCase")]
pub enum BackgroundImageStretchMode {
    Fill,
    None,
    #[default]
    Uniform,
    UniformToFill,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "camelCase")]
pub enum CursorShape {
    Bar,
    DoubleUnderscore,
    EmptyBox,
    FilledBox,
    Underscore,
    #[default]
    Vintage,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum FontWeight {
    String(FontWeightString),
    Integer(i32),
}

impl_untagged_default!(FontWeight, String, FontWeightString::Normal);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "camelCase")]
pub enum AntialiasingMode {
    #[default]
    Grayscale,
    Cleartype,
    Aliased,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum BellStyle {
    Boolean(bool),
    Array(Vec<BellStyleType>),
    String(BellStyleType),
}

impl_untagged_default!(BellStyle, String, BellStyleType::Audible);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum BellSound {
    String(String),
    Array(Vec<String>),
}

impl_untagged_default!(BellSound, Array, Vec::new());

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "camelCase")]
pub enum IntenseTextStyle {
    None,
    Bold,
    #[default]
    Bright,
    All,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "camelCase")]
pub enum AdjustIndistinguishableColors {
    Never,
    #[default]
    Indexed,
    Always,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "camelCase")]
pub enum ScrollbarState {
    #[default]
    Visible,
    Hidden,
    Always,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "camelCase")]
pub enum PathTranslationStyle {
    #[default]
    None,
    Wsl,
    Cygwin,
    Msys2,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum CopyFormat {
    Boolean(bool),
    Array(Vec<CopyFormatType>),
    String(CopyFormatType),
}

impl_untagged_default!(CopyFormat, String, CopyFormatType::None);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "lowercase")]
pub enum CopyFormatType {
    Html,
    Rtf,
    All,
    #[default]
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "camelCase")]
pub enum FontWeightString {
    Thin,
    ExtraLight,
    Light,
    SemiLight,
    #[default]
    Normal,
    Medium,
    SemiBold,
    Bold,
    ExtraBold,
    Black,
    ExtraBlack,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "lowercase")]
pub enum BellStyleType {
    #[default]
    Audible,
    Window,
    Taskbar,
    All,
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "camelCase")]
pub enum SpecialThemeColor {
    #[default]
    Accent,
    TerminalBackground,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "camelCase")]
pub enum ShowCloseButton {
    #[default]
    Always,
    Hover,
    Never,
    ActiveOnly,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "camelCase")]
pub enum IconStyle {
    #[default]
    Default,
    Hidden,
    Monochrome,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "camelCase")]
pub enum ApplicationTheme {
    Light,
    Dark,
    #[default]
    System,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "camelCase")]
pub enum IterateOn {
    #[default]
    Profiles,
    Schemes,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "camelCase")]
pub enum FolderEntryInlining {
    Never,
    #[default]
    Auto,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
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
    #[default]
    Unbound,
}

/// Command value: either a named shortcut action or a raw JSON object
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum CommandValue {
    String(ShortcutActionName),
    Object(serde_json::Value),
}

impl_untagged_default!(CommandValue, String, ShortcutActionName::Unbound);

/// Command name: either a plain string or a keyed object
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum CommandName {
    String(String),
    Object(CommandNameObject),
}

impl_untagged_default!(CommandName, String, String::new());

/// Command name keyed object
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CommandNameObject {
    pub key: String,
}

/// Key chord value: single key string or multiple key strings
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum KeyChordValue {
    Single(String),
    Multiple(Vec<String>),
}

impl_untagged_default!(KeyChordValue, Single, String::new());

/// Identifies which dynamic profile generator produced a profile
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum DynamicProfileSource {
    #[serde(rename = "Windows.Terminal.Wsl")]
    Wsl,
    #[serde(rename = "Windows.Terminal.Azure")]
    Azure,
    #[serde(rename = "Windows.Terminal.PowershellCore")]
    PowershellCore,
    #[serde(rename = "Windows.Terminal.VisualStudio")]
    VisualStudio,
}

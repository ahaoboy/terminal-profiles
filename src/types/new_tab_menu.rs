use serde::{Deserialize, Serialize};

use super::enums::FolderEntryInlining;

/// New-tab dropdown menu entry (one of several variants)
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

/// Folder (nested submenu) entry in the new-tab menu
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
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

/// Separator entry in the new-tab menu
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct SeparatorEntry {
    #[serde(rename = "type")]
    pub entry_type: String,
}

/// Profile entry in the new-tab menu (references a profile by name or GUID)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ProfileEntry {
    #[serde(rename = "type")]
    pub entry_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<String>,
}

/// Match-profiles entry: dynamically matches profiles by source, name, or commandline
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
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

/// Remaining-profiles entry: auto-populates with all unmatched profiles
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RemainingProfilesEntry {
    #[serde(rename = "type")]
    pub entry_type: String,
}

/// Action entry in the new-tab menu (triggers a global action by id)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ActionEntry {
    #[serde(rename = "type")]
    pub entry_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

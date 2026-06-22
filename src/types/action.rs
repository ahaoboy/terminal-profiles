use serde::{Deserialize, Serialize};

use super::enums::{CommandName, CommandValue, IterateOn, KeyChordValue};

/// Global action definition (command palette / keybinding target)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
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

/// Nested sub-command (used inside iterable or compound actions)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct SubCommand {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<CommandValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<CommandName>,
}

/// Keyboard shortcut binding
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Keybinding {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub keys: KeyChordValue,
}

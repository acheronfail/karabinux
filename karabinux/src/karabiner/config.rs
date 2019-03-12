use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

/// Karabiner configuration file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KBConfig {
    pub profiles: Vec<KBProfile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KBProfile {
    pub name: String,
    pub selected: bool,
    pub simple_modifications: Vec<KBSimpleModification>,
    pub complex_modifications: KBComplexModifications,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KBSimpleModification {
    pub from: KBSimpleRule,
    pub to: KBSimpleRule,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KBSimpleRule {
    pub key_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KBComplexModifications {
    pub parameters: Option<HashMap<String, Value>>,
    pub rules: Vec<KBComplexRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KBComplexRule {
    pub manipulators: Vec<KBManipulator>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KBManipulator {
    pub description: Option<String>,
    #[serde(rename = "type")]
    pub kind: KBManipulatorKind,
    pub from: KBFromDefinition,
    pub to: Option<Vec<KBToDefinition>>,
    pub conditions: Option<Vec<KBCondition>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KBManipulatorKind {
    Basic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KBFromDefinition {
    pub key_code: Option<String>,
    pub modifiers: Option<KBFromModifiers>,
    pub simultaneous: Option<Vec<KBFromDefinition>>,
    pub simultaneous_options: Option<KBSimultaneousOptions>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KBToDefinition {
    pub key_code: Option<String>,
    pub modifiers: Option<Vec<String>>,
    pub shell_command: Option<String>,
    pub repeat: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KBCondition {
    #[serde(rename = "type")]
    pub kind: KBConditionKind,
    pub bundle_identifiers: Option<Vec<String>>, // regexp
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KBConditionKind {
    FrontmostApplicationIf,
    FrontmostApplicationUnless,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KBFromModifiers {
    pub mandatory: Option<Vec<String>>,
    pub optional: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KBSimultaneousOptions {
    pub to_after_key_up: Option<Vec<KBToDefinition>>,
}

impl KBConfig {
    pub fn from_path<T: AsRef<Path>>(path: T) -> serde_json::Result<KBConfig> {
        let mut file = File::open(path).expect("failed to open config file");
        serde_json::from_reader(&mut file)
    }
}

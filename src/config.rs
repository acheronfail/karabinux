use std::path::Path;
use std::collections::HashMap;
use std::fs::File;
use serde_derive::{Deserialize, Serialize};
use serde_json::{Value};

/// Karabiner configuration file.
#[derive(Debug, Serialize, Deserialize)]
pub struct KBConfig {
    pub profiles: Vec<KBProfile>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KBProfile {
    pub name: String,
    pub selected: bool,
    pub simple_modifications: Vec<KBSimpleModification>,
    pub complex_modifications: KBComplexModifications,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KBSimpleModification {
    pub from: KBSimpleRule,
    pub to: KBSimpleRule
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KBSimpleRule {
    pub key_code: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KBComplexModifications {
    pub parameters: Option<HashMap<String, Value>>,
    pub rules: Vec<KBComplexRule>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KBComplexRule {
    pub manipulators: Vec<KBManipulator>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KBManipulator {
    pub description: Option<String>,
    #[serde(rename = "type")]
    pub kind: KBManipulatorType,
    pub from: KBFromDefinition,
    pub to: Option<Vec<KBToDefinition>>,
    pub conditions: Option<Vec<KBCondition>>
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KBManipulatorType {
    Basic
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KBFromDefinition {
    pub key_code: Option<String>,
    pub modifiers: Option<KBFromModifiers>,
    pub simultaneous: Option<Vec<KBFromDefinition>>,
    pub simultaneous_options: Option<KBSimultaneousOptions>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KBToDefinition {
    pub key_code: Option<String>,
    pub modifiers: Option<Vec<String>>,
    pub shell_command: Option<String>,
    pub repeat: Option<bool>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KBCondition {
    #[serde(rename = "type")]
    pub kind: KBConditionType,
    pub bundle_identifiers: Option<Vec<String>> // regexp
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KBConditionType {
    FrontmostApplicationIf,
    FrontmostApplicationUnless,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KBFromModifiers {
    pub mandatory: Vec<String>,
    pub optional: Option<Vec<String>>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KBSimultaneousOptions {
    pub to_after_key_up: Option<Vec<KBToDefinition>>
}

impl KBConfig {
    pub fn from_path<T: AsRef<Path>>(path: T) -> serde_json::Result<KBConfig> {
        let mut file = File::open(path).expect("failed to open config file");
        serde_json::from_reader(&mut file)
    }
}

use crate::karabiner::{FromKBKeyCode, KBFromModifiers, KBModifier};
use std::collections::HashSet;

#[derive(Debug, Default)]
pub struct FromModifiers {
    pub mandatory: HashSet<KBModifier>,
    pub optional: HashSet<KBModifier>,
}

impl FromModifiers {
    pub fn new() -> FromModifiers {
        FromModifiers {
            mandatory: HashSet::new(),
            optional: HashSet::new(),
        }
    }

    pub fn from_config(kb_from_modifiers: &KBFromModifiers) -> FromModifiers {
        let mut mandatory = HashSet::new();
        if let Some(mandatory_key_codes) = &kb_from_modifiers.mandatory {
            for key_code in mandatory_key_codes {
                if let Some(kb_modifier) = KBModifier::from_kb_key_code(&key_code) {
                    mandatory.insert(kb_modifier);
                }
            }
        }

        let mut optional = HashSet::new();
        if let Some(optional_key_codes) = &kb_from_modifiers.optional {
            for key_code in optional_key_codes {
                if let Some(kb_modifier) = KBModifier::from_kb_key_code(&key_code) {
                    optional.insert(kb_modifier);
                }
            }
        }

        FromModifiers {
            mandatory,
            optional,
        }
    }
}

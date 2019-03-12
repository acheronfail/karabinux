use crate::karabiner::{FromKBKeyCode, KBFromModifiers, Modifier};
use std::collections::hash_map::Iter;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum FromModifier {
    Absent,
    Optional,
    Mandatory,
}

#[derive(Debug)]
pub struct FromModifiers {
    inner: HashMap<Modifier, FromModifier>,
}

impl FromModifiers {
    pub fn from_config(kb_from_modifiers: &KBFromModifiers) -> FromModifiers {
        let mut inner = HashMap::new();
        let mut parse_key_code = |key_code: &str, fm: FromModifier| {
            if let Some(modifier) = Modifier::from_kb_key_code(&key_code) {
                inner.insert(modifier, fm);
            }
        };

        if let Some(mandatory_key_codes) = &kb_from_modifiers.mandatory {
            for key_code in mandatory_key_codes {
                parse_key_code(&key_code, FromModifier::Mandatory);
            }
        }

        if let Some(optional_key_codes) = &kb_from_modifiers.optional {
            for key_code in optional_key_codes {
                parse_key_code(&key_code, FromModifier::Optional);
            }
        }

        FromModifiers { inner: inner }
    }

    pub fn get(&self, key: &Modifier) -> Option<&FromModifier> {
        self.inner.get(key)
    }

    pub fn set(&mut self, key: Modifier, value: FromModifier) {
        self.inner.insert(key, value);
    }

    pub fn has(&self, key: &Modifier) -> bool {
        self.inner.contains_key(key)
    }
}

impl<'a> IntoIterator for &'a FromModifiers {
    type Item = (&'a Modifier, &'a FromModifier);
    type IntoIter = Iter<'a, Modifier, FromModifier>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}

impl Default for FromModifiers {
    fn default() -> FromModifiers {
        FromModifiers {
            inner: HashMap::new(),
        }
    }
}

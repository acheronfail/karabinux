use crate::event::KeyEvent;
use crate::karabiner::KBModifier;
use crate::key_state::KeyState;
use crate::state::{FromModifiers, ModifierKey};
use linked_hash_set::LinkedHashSet;
use std::collections::HashSet;

#[derive(Debug, Default)]
pub struct ModifierState {
    inner: LinkedHashSet<ModifierKey>,
}

impl ModifierState {
    /// Creates a new `ModifierState` struct.
    pub fn new() -> ModifierState {
        ModifierState {
            inner: LinkedHashSet::new(),
        }
    }

    /// Updates the internal state to reflect the passed events.
    /// The `ModifierState` will keep an internal representation of active and
    /// inactive modifiers.
    pub fn update(&mut self, key_event: &KeyEvent) {
        let is_pressed = match key_event.state {
            KeyState::Pressed | KeyState::Autorepeat => true,
            KeyState::Released | KeyState::Unknown(_) => false,
        };

        if let Some(modifier) = ModifierKey::from_key(&key_event.key) {
            if is_pressed {
                self.inner.insert(modifier);
            } else {
                self.inner.remove(&modifier);
            }
        }
    }

    /// Check whether the given modifier is currently active.
    pub fn is_pressed(&self, modifier: ModifierKey) -> bool {
        match modifier {
            modifier => self.inner.contains(&modifier),
        }
    }

    /// If "modifiers" is not present:
    ///     - only fire when NO modifiers are pressed
    /// If "modifiers.mandatory" is present:
    ///     - needs modifiers to be pressed
    ///     - fire keys without the mandatory modifiers
    /// If "modifiers.optional" is present:
    ///     - keys matched independently of modifiers
    ///     - modifiers fire independently of keys
    /// See: https://pqrs.org/osx/karabiner/json.html#from-event-definition-modifiers
    pub fn test_modifiers(&self, fm: &FromModifiers) -> Option<HashSet<ModifierKey>> {
        let mut modifier_keys = HashSet::new();

        // If Modifier::Any is mandatory, return all active modifiers.
        if fm.mandatory.contains(&KBModifier::Any) {
            for modifier_key in &ModifierKey::VARIANTS {
                if self.is_pressed(*modifier_key) {
                    modifier_keys.insert(modifier_key.clone());
                }
            }

            return Some(modifier_keys);
        }


        // Mandatory modifiers.
        for kb_modifier in &KBModifier::VARIANTS {
            if fm.mandatory.contains(kb_modifier) {
                match self.test_modifier(*kb_modifier) {
                    (true, Some(modifier_key)) => modifier_keys.insert(modifier_key),
                    _ => return None,
                };
            }
        }

        // Optional modifiers.
        if !fm.optional.contains(&KBModifier::Any) {
            let mut extra_modifier_keys = HashSet::new();
            for modifier_key in &ModifierKey::VARIANTS {
                extra_modifier_keys.insert(modifier_key);
            }

            for kb_modifier in &KBModifier::VARIANTS {
                if fm.mandatory.contains(kb_modifier) || fm.optional.contains(kb_modifier) {
                    for modifier_key in kb_modifier.as_modifiers() {
                        extra_modifier_keys.remove(&modifier_key);
                    }
                }
            }

            for modifier_key in extra_modifier_keys {
                if self.is_pressed(*modifier_key) {
                    return None;
                }
            }
        }

        Some(modifier_keys)
    }

    fn test_modifier(&self, kb_modifier: KBModifier) -> (bool, Option<ModifierKey>) {
        if kb_modifier == KBModifier::Any {
            return (true, None);
        }

        for modifier_key in kb_modifier.as_modifiers() {
            if self.is_pressed(modifier_key) {
                return (true, Some(modifier_key));
            }
        }

        (false, None)
    }

    // /// Return a sorted (in the order they were activated) list of modifier keys
    // /// that match the passed `Modifier`.
    // pub fn keys_for_modifier(&self, for_modifier: KBModifier) -> Vec<EV_KEY> {
    //     self.inner
    //         .iter()
    //         .filter(|&&m| match for_modifier {
    //             KBModifier::Any => true,
    //             KBModifier::Alt => m == KBModifier::LeftAlt || m == KBModifier::RightAlt,
    //             KBModifier::Meta => m == KBModifier::LeftMeta || m == KBModifier::RightMeta,
    //             KBModifier::Shift => m == KBModifier::LeftShift || m == KBModifier::RightShift,
    //             KBModifier::Control => m == KBModifier::LeftControl || m == KBModifier::RightControl,
    //             modifier => m == modifier,
    //         })
    //         .filter_map(|m| m.as_key())
    //         .collect()
    // }

    // // Checks if either of the "control" modifier keys are active.
    // pub fn control(&self) -> bool {
    //     self.inner.contains(&KBModifier::LeftControl) || self.inner.contains(&KBModifier::RightControl)
    // }

    // // Checks if either of the "shift" modifier keys are active.
    // pub fn shift(&self) -> bool {
    //     self.inner.contains(&KBModifier::LeftShift) || self.inner.contains(&KBModifier::RightShift)
    // }

    // // Checks if either of the "alt" modifier keys are active.
    // pub fn alt(&self) -> bool {
    //     self.inner.contains(&KBModifier::LeftAlt) || self.inner.contains(&KBModifier::RightAlt)
    // }

    // // Checks if either of the "meta" modifier keys are active.
    // pub fn meta(&self) -> bool {
    //     self.inner.contains(&KBModifier::LeftMeta) || self.inner.contains(&KBModifier::RightMeta)
    // }

    // // Check if any modifier key is active.
    // pub fn any(&self) -> bool {
    //     self.inner.contains(&KBModifier::Capslock)
    //         || self.control()
    //         || self.shift()
    //         || self.alt()
    //         || self.meta()
    // }
}

impl<'a> IntoIterator for &'a ModifierState {
    type Item = &'a ModifierKey;
    type IntoIter = linked_hash_set::Iter<'a, ModifierKey>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}

// #[cfg(test)]
// mod tests {

//     use crate::karabiner::KBModifier;
//     use crate::state::{FromModifier, FromModifiers, ModifierState};
//     use pretty_assertions::assert_eq;

//     #[test]
//     fn matches_allows_absent_modifiers() {
//         let empty_state = ModifierState::new();
//         let mut from_modifiers = FromModifiers::default();

//         for modifier in KBModifier::VARIANTS.iter() {
//             from_modifiers.set(KBmodifier.clone(), FromModifier::Absent);
//         }

//         assert_eq!(empty_state.test_modifiers(&from_modifiers, None), true);
//     }

//     #[test]
//     fn matches_allows_optional_modifiers() {
//         let empty_state = ModifierState::new();
//         let mut from_modifiers = FromModifiers::default();

//         for modifier in Modifier::VARIANTS.iter() {
//             from_modifiers.set(KBmodifier.clone(), FromModifier::Optional);
//         }

//         assert_eq!(empty_state.test_modifiers(&from_modifiers, None), true);
//     }

//     #[test]
//     fn matches_disallows_mandatory_modifiers() {
//         let empty_state = ModifierState::new();
//         let mut from_modifiers = FromModifiers::default();

//         for modifier in KBModifier::VARIANTS.iter() {
//             from_modifiers.set(KBmodifier.clone(), FromModifier::Mandatory);
//         }

//         assert_eq!(empty_state.test_modifiers(&from_modifiers, None), false);
//     }
// }

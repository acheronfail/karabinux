use crate::event::KeyEvent;
use crate::karabiner::Modifier;
use crate::key_state::KeyState;
use crate::state::{FromModifier, FromModifiers};
use evdev_rs::enums::EV_KEY;
use linked_hash_set::LinkedHashSet;

#[derive(Debug, Default)]
pub struct ModifierState {
    inner: LinkedHashSet<Modifier>,
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
        let is_active = match key_event.state {
            KeyState::Pressed | KeyState::Autorepeat => true,
            KeyState::Released | KeyState::Unknown(_) => false,
        };

        if let Some(modifier) = Modifier::from_key(&key_event.key) {
            if is_active {
                self.inner.insert(modifier);
            } else {
                self.inner.remove(&modifier);
            }
        }
    }

    /// Check whether the given modifier is currently active.
    pub fn is_active(&self, modifier: Modifier) -> bool {
        match modifier {
            Modifier::Any => self.any(),
            Modifier::Alt => self.alt(),
            Modifier::Meta => self.meta(),
            Modifier::Shift => self.shift(),
            Modifier::Control => self.control(),
            modifier => self.inner.contains(&modifier),
        }
    }

    /// Return a sorted (in the order they were activated) list of modifier keys
    /// that match the passed `Modifier`.
    pub fn keys_for_modifier(&self, for_modifier: Modifier) -> Vec<EV_KEY> {
        self.inner
            .iter()
            .filter(|&&m| match for_modifier {
                Modifier::Any => true,
                Modifier::Alt => m == Modifier::LeftAlt || m == Modifier::RightAlt,
                Modifier::Meta => m == Modifier::LeftMeta || m == Modifier::RightMeta,
                Modifier::Shift => m == Modifier::LeftShift || m == Modifier::RightShift,
                Modifier::Control => m == Modifier::LeftControl || m == Modifier::RightControl,
                modifier => m == modifier,
            })
            .filter_map(|m| m.as_key())
            .collect()
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
    pub fn test_modifiers(&self, fm: &FromModifiers, event_modifier: Option<Modifier>) -> bool {
        // If "any" modifier exists, check that first.
        if let Some(condition) = fm.get(Modifier::Any) {
            return ModifierState::check_condition_pair((*condition, self.any()));
        }

        let mut pairs = vec![];
        let mut try_check = |modifier: Modifier| {
            if let Some(condition) = fm.get(modifier) {
                // Filter out the current event if it's a modifier.
                if Some(modifier) != event_modifier {
                    pairs.push((*condition, self.is_active(modifier)));
                }
                return true;
            }

            false
        };

        if !try_check(Modifier::Alt) {
            try_check(Modifier::LeftAlt);
            try_check(Modifier::RightAlt);
        }

        if !try_check(Modifier::Meta) {
            try_check(Modifier::LeftMeta);
            try_check(Modifier::RightMeta);
        }

        if !try_check(Modifier::Shift) {
            try_check(Modifier::LeftShift);
            try_check(Modifier::RightShift);
        }

        if !try_check(Modifier::Control) {
            try_check(Modifier::LeftControl);
            try_check(Modifier::RightControl);
        }

        // No from modifiers specified, but we have a modifier -> do not match.
        if pairs.is_empty() && self.any() {
            return false;
        }

        pairs
            .iter()
            .all(|pair| ModifierState::check_condition_pair(*pair))
    }

    fn check_condition_pair(pair: (FromModifier, bool)) -> bool {
        let (cond, state) = pair;
        match cond {
            FromModifier::Absent => !state,
            FromModifier::Optional => true,
            FromModifier::Mandatory => state,
        }
    }

    // Checks if either of the "control" modifier keys are active.
    pub fn control(&self) -> bool {
        self.inner.contains(&Modifier::LeftControl) || self.inner.contains(&Modifier::RightControl)
    }

    // Checks if either of the "shift" modifier keys are active.
    pub fn shift(&self) -> bool {
        self.inner.contains(&Modifier::LeftShift) || self.inner.contains(&Modifier::RightShift)
    }

    // Checks if either of the "alt" modifier keys are active.
    pub fn alt(&self) -> bool {
        self.inner.contains(&Modifier::LeftAlt) || self.inner.contains(&Modifier::RightAlt)
    }

    // Checks if either of the "meta" modifier keys are active.
    pub fn meta(&self) -> bool {
        self.inner.contains(&Modifier::LeftMeta) || self.inner.contains(&Modifier::RightMeta)
    }

    // Check if any modifier key is active.
    pub fn any(&self) -> bool {
        self.inner.contains(&Modifier::Capslock)
            || self.control()
            || self.shift()
            || self.alt()
            || self.meta()
    }
}

impl<'a> IntoIterator for &'a ModifierState {
    type Item = &'a Modifier;
    type IntoIter = linked_hash_set::Iter<'a, Modifier>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}

#[cfg(test)]
mod tests {

    use crate::karabiner::Modifier;
    use crate::state::{FromModifier, FromModifiers, ModifierState};
    use pretty_assertions::assert_eq;

    #[test]
    fn matches_allows_absent_modifiers() {
        let empty_state = ModifierState::new();
        let mut from_modifiers = FromModifiers::default();

        for modifier in Modifier::VARIANTS.iter() {
            from_modifiers.set(modifier.clone(), FromModifier::Absent);
        }

        assert_eq!(empty_state.test_modifiers(&from_modifiers, None), true);
    }

    #[test]
    fn matches_allows_optional_modifiers() {
        let empty_state = ModifierState::new();
        let mut from_modifiers = FromModifiers::default();

        for modifier in Modifier::VARIANTS.iter() {
            from_modifiers.set(modifier.clone(), FromModifier::Optional);
        }

        assert_eq!(empty_state.test_modifiers(&from_modifiers, None), true);
    }

    #[test]
    fn matches_disallows_mandatory_modifiers() {
        let empty_state = ModifierState::new();
        let mut from_modifiers = FromModifiers::default();

        for modifier in Modifier::VARIANTS.iter() {
            from_modifiers.set(modifier.clone(), FromModifier::Mandatory);
        }

        assert_eq!(empty_state.test_modifiers(&from_modifiers, None), false);
    }
}

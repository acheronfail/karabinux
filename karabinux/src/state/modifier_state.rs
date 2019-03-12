use crate::karabiner::Modifier;
use crate::key_state::KeyState;
use crate::state::{FromModifier, FromModifiers};
use evdev_rs::enums::{EventCode, EV_KEY};
use evdev_rs::InputEvent;
use linked_hash_set::LinkedHashSet;

#[derive(Debug)]
pub struct ModifierState {
    inner: LinkedHashSet<Modifier>,
}

impl ModifierState {
    pub fn new() -> ModifierState {
        ModifierState {
            inner: LinkedHashSet::new(),
        }
    }

    pub fn update(&mut self, ev: &InputEvent) {
        let is_active = match KeyState::from(ev.value) {
            KeyState::Pressed | KeyState::Autorepeat => true,
            KeyState::Released | KeyState::Unknown(_) => false,
        };

        if let EventCode::EV_KEY(key) = &ev.event_code {
            if let Some(modifier) = Modifier::from_key(&key) {
                if is_active {
                    self.inner.insert(modifier);
                } else {
                    self.inner.remove(&modifier);
                }
            }
        }
    }

    pub fn is_active(&self, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Any => self.any(),
            Modifier::Alt => self.alt(),
            Modifier::Meta => self.meta(),
            Modifier::Shift => self.shift(),
            Modifier::Control => self.control(),
            modifier => self.inner.contains(&modifier),
        }
    }

    // TODO: rename this method
    // TODO: doc that order matters
    pub fn get_keys(&self, for_modifier: &Modifier) -> Vec<EV_KEY> {
        self.inner
            .iter()
            .filter(|&&m| match for_modifier {
                Modifier::Any => true,
                Modifier::Alt => m == Modifier::LeftAlt || m == Modifier::RightAlt,
                Modifier::Meta => m == Modifier::LeftMeta || m == Modifier::RightMeta,
                Modifier::Shift => m == Modifier::LeftShift || m == Modifier::RightShift,
                Modifier::Control => m == Modifier::LeftControl || m == Modifier::RightControl,
                modifier => m == *modifier,
            })
            .filter_map(|m| m.as_key())
            .collect()
    }

    pub fn matches(&self, fm: &FromModifiers) -> bool {
        // If "any" modifier exists, check that first.
        if let Some(condition) = fm.get(&Modifier::Any) {
            return ModifierState::check_condition_pair(&(*condition, self.any()));
        }

        let mut pairs = vec![];
        let mut try_check = |modifier: Modifier| {
            if let Some(condition) = fm.get(&modifier) {
                pairs.push((*condition, self.is_active(&modifier)));
                true
            } else {
                false
            }
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
        if pairs.len() == 0 && self.any() {
            return false;
        }

        pairs
            .iter()
            .all(|pair| ModifierState::check_condition_pair(pair))
    }

    fn check_condition_pair(pair: &(FromModifier, bool)) -> bool {
        let (cond, state) = pair;
        match cond {
            FromModifier::Absent => *state == false,
            FromModifier::Optional => true,
            FromModifier::Mandatory => *state == true,
        }
    }

    pub fn control(&self) -> bool {
        self.inner.contains(&Modifier::LeftControl) || self.inner.contains(&Modifier::RightControl)
    }

    pub fn shift(&self) -> bool {
        self.inner.contains(&Modifier::LeftShift) || self.inner.contains(&Modifier::RightShift)
    }

    pub fn alt(&self) -> bool {
        self.inner.contains(&Modifier::LeftAlt) || self.inner.contains(&Modifier::RightAlt)
    }

    pub fn meta(&self) -> bool {
        self.inner.contains(&Modifier::LeftMeta) || self.inner.contains(&Modifier::RightMeta)
    }

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

        assert_eq!(empty_state.matches(&from_modifiers), true);
    }

    #[test]
    fn matches_allows_optional_modifiers() {
        let empty_state = ModifierState::new();
        let mut from_modifiers = FromModifiers::default();

        for modifier in Modifier::VARIANTS.iter() {
            from_modifiers.set(modifier.clone(), FromModifier::Optional);
        }

        assert_eq!(empty_state.matches(&from_modifiers), true);
    }

    #[test]
    fn matches_disallows_mandatory_modifiers() {
        let empty_state = ModifierState::new();
        let mut from_modifiers = FromModifiers::default();

        for modifier in Modifier::VARIANTS.iter() {
            from_modifiers.set(modifier.clone(), FromModifier::Mandatory);
        }

        assert_eq!(empty_state.matches(&from_modifiers), false);
    }
}

//! AI Planner - Action selection and planning

use simcore::{Action, PlayerId, State};

/// Select next action for a player
pub fn select_action(_state: &State, _player: PlayerId) -> Option<Action> {
    // Placeholder: returns None (no action)
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select_action_compiles() {
        let state = State::new();
        let action = select_action(&state, PlayerId(0));
        assert!(action.is_none());
    }
}


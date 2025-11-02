//! AI Evaluation - State evaluation and scoring

use simcore::{PlayerId, State};

/// Evaluate state for a given player
pub fn evaluate_state(_state: &State, _player: PlayerId) -> f64 {
    // Placeholder: returns neutral score
    0.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate_returns_score() {
        let state = State::new();
        let score = evaluate_state(&state, PlayerId(0));
        assert!(score.is_finite());
    }
}


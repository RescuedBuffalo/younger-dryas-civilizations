//! SimCore - Deterministic strategy simulation core

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

/// Opaque player identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PlayerId(pub u64);

/// Opaque city identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CityId(pub u64);

/// Opaque unit identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UnitId(pub u64);

/// Tile coordinate
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TileCoord {
    pub x: i32,
    pub y: i32,
}

/// 128-bit deterministic hash
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Hash128(pub u128);

/// Game action enum per SimCore Contract
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Action {
    EndTurn,
    MoveUnit {
        unit: UnitId,
        path: Vec<TileCoord>,
        ap: i32,
    },
    Attack {
        attacker: UnitId,
        target: UnitId,
    },
    Fortify {
        unit: UnitId,
    },
    BuildUnit {
        city: CityId,
        kind: String,
    },
    BuildDistrict {
        city: CityId,
        kind: String,
        tile: TileCoord,
    },
    SetPolicy {
        slot: i32,
        id: String,
    },
    ChooseTech {
        id: String,
    },
    OfferDeal {
        json: String,
    },
    AcceptDeal {
        id: String,
    },
    DeclineDeal {
        id: String,
    },
}

/// Simulation error
#[derive(Error, Debug)]
pub enum SimError {
    #[error("Invalid action: {0}")]
    InvalidAction(String),
    #[error("Invariant violation: {0}")]
    InvariantViolation(String),
}

/// Game state (placeholder)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct State {
    pub turn: i32,
    pub rules_ver: String,
    // Placeholder fields
    _players: HashMap<PlayerId, ()>,
    _cities: HashMap<CityId, ()>,
    _units: HashMap<UnitId, ()>,
}

impl State {
    pub fn new() -> Self {
        Self {
            turn: 0,
            rules_ver: "0.1.0".to_string(),
            _players: HashMap::new(),
            _cities: HashMap::new(),
            _units: HashMap::new(),
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

/// Effects from applying an action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Effects {
    pub deltas: Vec<String>,
    pub events: Vec<String>,
}

/// Enumerate all legal actions for a player
pub fn enumerate_legal_actions(_state: &State, _player: PlayerId) -> Vec<Action> {
    // Placeholder: returns empty vector
    Vec::new()
}

/// Validate an action against current state
pub fn validate_action(_state: &State, _action: &Action) -> Result<(), SimError> {
    // Placeholder: always succeeds
    Ok(())
}

/// Apply an action to state, returning effects
pub fn apply_action(_state: &mut State, _action: Action) -> Result<Effects, SimError> {
    // Placeholder: no-op
    Ok(Effects {
        deltas: Vec::new(),
        events: Vec::new(),
    })
}

/// Execute end-of-turn processing
pub fn end_turn(state: &mut State) -> Result<(), SimError> {
    // Placeholder: increment turn
    state.turn += 1;
    Ok(())
}

/// Compute deterministic state hash
pub fn state_hash(state: &State) -> Hash128 {
    // Placeholder: deterministic hash based on turn number
    // In production, this would hash all state fields in a stable order
    let hash = state.turn as u128;
    Hash128(hash)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_hash_deterministic() {
        let state = State::new();
        let hash1 = state_hash(&state);
        let hash2 = state_hash(&state);
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_end_turn_increments() {
        let mut state = State::new();
        assert_eq!(state.turn, 0);
        end_turn(&mut state).unwrap();
        assert_eq!(state.turn, 1);
    }

    // ========== PROPTEST SCAFFOLDS ==========

    mod proptests {
        use super::*;
        use proptest::prelude::*;

        // Arbitrary generator for TileCoord
        impl Arbitrary for TileCoord {
            type Parameters = ();
            type Strategy = BoxedStrategy<Self>;

            fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
                (any::<i32>(), any::<i32>())
                    .prop_map(|(x, y)| TileCoord { x, y })
                    .boxed()
            }
        }

        // Arbitrary generator for Action
        impl Arbitrary for Action {
            type Parameters = ();
            type Strategy = BoxedStrategy<Self>;

            fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
                prop_oneof![
                    // EndTurn
                    Just(Action::EndTurn),
                    // MoveUnit
                    (any::<u64>(), prop::collection::vec(any::<TileCoord>(), 0..10), any::<i32>())
                        .prop_map(|(unit, path, ap)| Action::MoveUnit {
                            unit: UnitId(unit),
                            path,
                            ap,
                        }),
                    // Attack
                    (any::<u64>(), any::<u64>()).prop_map(|(attacker, target)| Action::Attack {
                        attacker: UnitId(attacker),
                        target: UnitId(target),
                    }),
                    // Fortify
                    any::<u64>().prop_map(|unit| Action::Fortify {
                        unit: UnitId(unit),
                    }),
                    // BuildUnit
                    (any::<u64>(), "[a-z]{3,8}").prop_map(|(city, kind)| Action::BuildUnit {
                        city: CityId(city),
                        kind,
                    }),
                    // BuildDistrict
                    (any::<u64>(), "[a-z]{3,8}", any::<TileCoord>()).prop_map(
                        |(city, kind, tile)| Action::BuildDistrict {
                            city: CityId(city),
                            kind,
                            tile,
                        }
                    ),
                    // SetPolicy
                    (any::<i32>(), "[a-z]{3,8}").prop_map(|(slot, id)| Action::SetPolicy {
                        slot,
                        id,
                    }),
                    // ChooseTech
                    "[a-z]{3,8}".prop_map(|id| Action::ChooseTech { id }),
                    // OfferDeal
                    "\\{.*\\}".prop_map(|json| Action::OfferDeal { json }),
                    // AcceptDeal
                    "[a-z0-9]{4,8}".prop_map(|id| Action::AcceptDeal { id }),
                    // DeclineDeal
                    "[a-z0-9]{4,8}".prop_map(|id| Action::DeclineDeal { id }),
                ]
                .boxed()
            }
        }

        /// Invariant check: 1UPT (one unit per tile) for combat units
        fn check_1upt_invariant(_state: &State) -> Result<(), String> {
            // TODO(spec): Implement 1UPT check
            // - Collect all combat unit positions
            // - Verify no two combat units occupy same tile
            // - Allow civilian co-stack with 1 combat unit
            Ok(())
        }

        /// Invariant check: No units on out-of-bounds or impassable tiles
        fn check_oob_invariant(_state: &State) -> Result<(), String> {
            // TODO(spec): Implement OOB check
            // - Verify all unit positions are within map bounds
            // - Verify all unit positions are on passable tiles
            Ok(())
        }

        /// Invariant check: Cost constraints honored
        fn check_cost_invariant(_state: &State) -> Result<(), String> {
            // TODO(spec): Implement cost constraint check
            // - Verify no player has negative resources
            // - Verify all built units/districts were paid for
            // - Verify action points (AP) are non-negative
            Ok(())
        }

        /// Check all simulation invariants
        fn check_all_invariants(state: &State) -> Result<(), String> {
            check_1upt_invariant(state)?;
            check_oob_invariant(state)?;
            check_cost_invariant(state)?;
            Ok(())
        }

        proptest! {
            /// Invariant fuzz test: random action streams never break constraints
            /// 
            /// Contract acceptance criterion: 10M random actions → 0 invariant breaks
            #[test]
            fn invariant_fuzz_random_actions(actions in prop::collection::vec(any::<Action>(), 0..100)) {
                let mut state = State::new();
                
                // Apply random action stream
                for action in actions {
                    // Attempt to apply action (may fail if invalid, which is fine)
                    let _ = apply_action(&mut state, action);
                    
                    // After each action, invariants must hold
                    check_all_invariants(&state)
                        .expect("Invariant violated after applying action");
                }
                
                // Final invariant check
                check_all_invariants(&state)
                    .expect("Invariant violated at end of action stream");
            }
        }

        proptest! {
            /// Determinism smoke test: fixed seed + action script produces same state_hash
            /// 
            /// Contract acceptance criterion: 100 random seeds → identical state_hash on replay
            #[test]
            fn determinism_smoke_fixed_seed(
                seed in any::<u64>(),
                actions in prop::collection::vec(any::<Action>(), 0..50)
            ) {
                // First run: apply actions with given seed
                let mut state1 = State::new();
                // TODO(spec): Initialize state with seed
                
                for action in actions.clone() {
                    let _ = apply_action(&mut state1, action);
                }
                let hash1 = state_hash(&state1);
                
                // Second run: replay with same seed and actions
                let mut state2 = State::new();
                // TODO(spec): Initialize state with same seed
                
                for action in actions {
                    let _ = apply_action(&mut state2, action);
                }
                let hash2 = state_hash(&state2);
                
                // Hashes must be identical (determinism)
                prop_assert_eq!(
                    hash1,
                    hash2,
                    "State hash differs on replay with seed={}", seed
                );
            }
        }
    }
}


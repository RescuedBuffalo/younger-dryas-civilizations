//! AI Negotiator - Deal generation and evaluation

use jsonschema::{Draft, JSONSchema};
use serde_json::Value;
use simcore::{Action, PlayerId, State};
use std::sync::OnceLock;
use thiserror::Error;

/// Deal validation error
#[derive(Error, Debug)]
pub enum DealValidationError {
    #[error("JSON parse error: {0}")]
    JsonParse(#[from] serde_json::Error),
    #[error("Schema validation failed: {0}")]
    SchemaValidation(String),
    #[error("Schema load error: {0}")]
    SchemaLoad(String),
}

/// Lazily-loaded JSON schema validator
static DEAL_SCHEMA: OnceLock<JSONSchema> = OnceLock::new();

/// Load and compile the deal JSON schema
fn get_deal_schema() -> Result<&'static JSONSchema, DealValidationError> {
    DEAL_SCHEMA.get_or_try_init(|| {
        // Load schema from embedded file
        let schema_json = include_str!("../../../schemas/deal.schema.json");
        let schema_value: Value = serde_json::from_str(schema_json)
            .map_err(|e| DealValidationError::SchemaLoad(e.to_string()))?;

        // Compile schema
        JSONSchema::options()
            .with_draft(Draft::Draft202012)
            .compile(&schema_value)
            .map_err(|e| DealValidationError::SchemaLoad(e.to_string()))
    })
}

/// Validate deal JSON against schema
///
/// Contract: Per Deal_DSL, validation should complete in ≤2ms
pub fn validate_deal_json(deal_json: &str) -> Result<(), DealValidationError> {
    // Parse JSON
    let deal_value: Value = serde_json::from_str(deal_json)?;

    // Get compiled schema
    let schema = get_deal_schema()?;

    // Validate against schema
    if let Err(errors) = schema.validate(&deal_value) {
        let error_messages: Vec<String> = errors.map(|e| e.to_string()).collect();
        return Err(DealValidationError::SchemaValidation(
            error_messages.join("; "),
        ));
    }

    Ok(())
}

/// Generate deal offer for negotiation
pub fn generate_deal(_state: &State, _from: PlayerId, _to: PlayerId) -> Option<Action> {
    // Placeholder: returns None (no deal)
    None
}

/// Evaluate incoming deal
///
/// Returns true if deal is accepted, false otherwise
pub fn evaluate_deal(_state: &State, _player: PlayerId, deal_json: &str) -> bool {
    // First, validate schema
    if validate_deal_json(deal_json).is_err() {
        return false;
    }

    // Placeholder: rejects all deals after validation
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_minimal_deal() {
        let deal = r#"{"give":{},"take":{}}"#;
        assert!(validate_deal_json(deal).is_ok());
    }

    #[test]
    fn test_validate_gold_exchange() {
        let deal = r#"{
            "give": {"gold": 100},
            "take": {"open_borders": true}
        }"#;
        assert!(validate_deal_json(deal).is_ok());
    }

    #[test]
    fn test_validate_research_agreement() {
        let deal = r#"{
            "give": {"gold": 50},
            "take": {"research_agreement": {"field": "Science", "turns": 10}}
        }"#;
        assert!(validate_deal_json(deal).is_ok());
    }

    #[test]
    fn test_validate_iron_license() {
        let deal = r#"{
            "give": {"iron_license": {"tech": "iron_working", "turns": 15}},
            "take": {"gold": 200},
            "duration": 15
        }"#;
        assert!(validate_deal_json(deal).is_ok());
    }

    #[test]
    fn test_reject_missing_required_fields() {
        let deal = r#"{"give": {}}"#; // Missing "take"
        assert!(validate_deal_json(deal).is_err());
    }

    #[test]
    fn test_reject_negative_gold() {
        let deal = r#"{"give": {"gold": -50}, "take": {}}"#;
        assert!(validate_deal_json(deal).is_err());
    }

    #[test]
    fn test_reject_excessive_duration() {
        let deal = r#"{
            "give": {},
            "take": {},
            "duration": 31
        }"#;
        assert!(validate_deal_json(deal).is_err());
    }

    #[test]
    fn test_reject_invalid_research_field() {
        let deal = r#"{
            "give": {},
            "take": {"research_agreement": {"field": "Magic", "turns": 5}}
        }"#;
        assert!(validate_deal_json(deal).is_err());
    }

    #[test]
    fn test_reject_additional_properties() {
        let deal = r#"{
            "give": {},
            "take": {},
            "unknown_field": "should_fail"
        }"#;
        assert!(validate_deal_json(deal).is_err());
    }

    #[test]
    fn test_validate_with_threat() {
        let deal = r#"{
            "give": {},
            "take": {"gold": 500},
            "threat": {"casus_belli": "border_dispute"}
        }"#;
        assert!(validate_deal_json(deal).is_ok());
    }

    #[test]
    fn test_validate_with_conditions() {
        let deal = r#"{
            "give": {"open_borders": true},
            "take": {"gold": 100},
            "conditions": ["player_alive", "not_at_war"]
        }"#;
        assert!(validate_deal_json(deal).is_ok());
    }

    #[test]
    fn test_evaluate_deal_rejects_invalid_json() {
        let state = State::new();
        let invalid_deal = r#"{"give": {}}"#; // Missing "take"
        assert!(!evaluate_deal(&state, PlayerId(0), invalid_deal));
    }

    #[test]
    fn test_evaluate_deal_validates_before_rejecting() {
        let state = State::new();
        let valid_deal = r#"{"give": {}, "take": {}}"#;
        // Should validate successfully but still reject (placeholder logic)
        assert!(!evaluate_deal(&state, PlayerId(0), valid_deal));
    }
}


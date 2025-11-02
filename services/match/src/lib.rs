//! Match service - gRPC interface for game matches

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use tonic::{Request, Response, Status};

// Generated protobuf code
pub mod proto {
    tonic::include_proto!("match.v1");
}

use proto::match_server::{Match, MatchServer};
use proto::*;

/// In-memory match state store (production: persist to DB)
#[derive(Debug, Clone)]
struct MatchState {
    match_id: String,
    turn: i32,
    state_hash: Vec<u8>,
    players: Vec<String>,
    seed: u64,
    // Idempotency: track processed action_ids
    processed_actions: HashMap<String, Acknowledgement>,
}

/// Match service implementation
#[derive(Debug, Clone)]
pub struct MatchService {
    matches: Arc<RwLock<HashMap<String, MatchState>>>,
}

impl MatchService {
    pub fn new() -> Self {
        Self {
            matches: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Generate action_id hash for idempotency
    /// Formula: sha256(match_id|turn|player|action_bytes|prev_state_hash[:8])
    fn compute_action_id(
        match_id: &str,
        turn: i32,
        player_id: &str,
        action_bytes: &[u8],
        prev_state_hash_prefix: &[u8],
    ) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        // Simple hash for stub (production: use SHA256)
        let mut hasher = DefaultHasher::new();
        match_id.hash(&mut hasher);
        turn.hash(&mut hasher);
        player_id.hash(&mut hasher);
        action_bytes.hash(&mut hasher);
        prev_state_hash_prefix.hash(&mut hasher);

        format!("{:x}", hasher.finish())
    }
}

impl Default for MatchService {
    fn default() -> Self {
        Self::new()
    }
}

#[tonic::async_trait]
impl Match for MatchService {
    async fn create_match(
        &self,
        request: Request<CreateMatchRequest>,
    ) -> Result<Response<CreateMatchResponse>, Status> {
        let req = request.into_inner();

        // Generate match_id
        let match_id = format!("match_{}", req.seed);

        // Mock initial state
        let initial_state = simcore::State::new();
        let state_hash = simcore::state_hash(&initial_state);
        let hash_bytes = state_hash.0.to_le_bytes().to_vec();

        // Store match state
        let match_state = MatchState {
            match_id: match_id.clone(),
            turn: 0,
            state_hash: hash_bytes.clone(),
            players: req.players.iter().map(|p| p.player_id.clone()).collect(),
            seed: req.seed,
            processed_actions: HashMap::new(),
        };

        self.matches
            .write()
            .unwrap()
            .insert(match_id.clone(), match_state);

        Ok(Response::new(CreateMatchResponse {
            match_id,
            initial_state_hash: hash_bytes,
            turn: 0,
        }))
    }

    async fn get_observation(
        &self,
        request: Request<ObservationRequest>,
    ) -> Result<Response<Observation>, Status> {
        let req = request.into_inner();

        // Retrieve match state
        let matches = self.matches.read().unwrap();
        let match_state = matches
            .get(&req.match_id)
            .ok_or_else(|| Status::not_found("Match not found"))?;

        // Mock observation from current state
        let observation = Observation {
            turn: match_state.turn,
            player_id: req.player_id.clone(),
            view: Some(View {
                tiles: vec![
                    TileInfo {
                        x: 0,
                        y: 0,
                        terrain: "plains".to_string(),
                        features: vec![],
                        visible: true,
                    },
                ],
                cities: vec![],
                units: vec![],
            }),
            yields: Some(Yields {
                food: 2,
                production: 1,
                gold: 3,
                science: 1,
                culture: 1,
                influence: 0,
            }),
            tech: Some(TechState {
                known: vec!["agriculture".to_string()],
                available: vec!["mining".to_string(), "pottery".to_string()],
                frozen: vec![],
            }),
            diplomacy: Some(DiplomacyState {
                relations: vec![],
                open_offers: vec![],
            }),
            legal_actions: vec![
                ActionLite {
                    action_type: "EndTurn".to_string(),
                    payload: b"{}".to_vec(),
                },
            ],
            state_hash: match_state.state_hash.clone(),
        };

        Ok(Response::new(observation))
    }

    async fn submit_action(
        &self,
        request: Request<ActionRequest>,
    ) -> Result<Response<Acknowledgement>, Status> {
        let req = request.into_inner();

        // Verify action_id for idempotency
        let computed_id = Self::compute_action_id(
            &req.match_id,
            req.turn,
            &req.player_id,
            &req.action_bytes,
            &req.prev_state_hash_prefix,
        );

        if computed_id != req.action_id {
            return Err(Status::invalid_argument(
                "action_id does not match computed hash",
            ));
        }

        let mut matches = self.matches.write().unwrap();
        let match_state = matches
            .get_mut(&req.match_id)
            .ok_or_else(|| Status::not_found("Match not found"))?;

        // Check if action already processed (idempotency)
        if let Some(ack) = match_state.processed_actions.get(&req.action_id) {
            return Ok(Response::new(ack.clone()));
        }

        // Deserialize and validate action (stub: always accept)
        let action: Result<simcore::Action, _> = serde_json::from_slice(&req.action_bytes);
        if action.is_err() {
            return Ok(Response::new(Acknowledgement {
                accepted: false,
                error: "Invalid action JSON".to_string(),
                action_id: req.action_id,
                new_state_hash: vec![],
            }));
        }

        // Mock state update
        let new_hash = (match_state.state_hash[0] as u128 + 1).to_le_bytes().to_vec();
        match_state.state_hash = new_hash.clone();

        // Create acknowledgement
        let ack = Acknowledgement {
            accepted: true,
            error: String::new(),
            action_id: req.action_id.clone(),
            new_state_hash: new_hash,
        };

        // Store for idempotency
        match_state.processed_actions.insert(req.action_id, ack.clone());

        Ok(Response::new(ack))
    }

    async fn advance(
        &self,
        request: Request<AdvanceRequest>,
    ) -> Result<Response<EventBatch>, Status> {
        let req = request.into_inner();

        let mut matches = self.matches.write().unwrap();
        let match_state = matches
            .get_mut(&req.match_id)
            .ok_or_else(|| Status::not_found("Match not found"))?;

        // Advance turn
        match_state.turn += 1;

        // Mock events
        let events = vec![
            GameEvent {
                event_type: "TurnAdvanced".to_string(),
                description: format!("Turn {} begins", match_state.turn),
                payload: vec![],
            },
        ];

        Ok(Response::new(EventBatch {
            turn: match_state.turn,
            events,
            state_hash: match_state.state_hash.clone(),
        }))
    }

    async fn negotiate(
        &self,
        request: Request<DealRequest>,
    ) -> Result<Response<DealResponse>, Status> {
        let _req = request.into_inner();

        // Mock negotiation: reject all deals
        Ok(Response::new(DealResponse {
            accepted: false,
            reason: "Not implemented yet".to_string(),
            counter_offer_json: String::new(),
        }))
    }
}

/// Helper to create server
pub fn create_server() -> MatchServer<MatchService> {
    MatchServer::new(MatchService::new())
}


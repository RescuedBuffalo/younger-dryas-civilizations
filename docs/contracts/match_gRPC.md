Contract — Match gRPC (Service & Idempotency)

problem: Headless clients (human/AI) must create/observe/act/advance a deterministic match via a narrow, idempotent API.
perf_budget: Server handles 6 AI clients @ ≤0.6s think/opponent; per‑call latency targets noted below.

interface (conceptual)

RPCs

CreateMatch(CreateMatchReq) -> CreateMatchRes (≤100ms)

GetObservation(ObsReq) -> Observation (≤50ms)

SubmitAction(ActionReq) -> Ack (≤25ms)

Advance(AdvanceReq) -> EventBatch (≤100ms)

Negotiate(DealReq) -> DealRes (≤80ms budget for negotiator path)

idempotency & safety

action_id = sha256(match_id|turn|player|serialized_action|prev_state_hash[:8]); server dedups.

Rate limits & backpressure; invalid actions return typed errors.

Observation (shape)
```
Observation {
  turn: i32,
  player_id: string,
  view: { tiles[], cities[], units[] },
  yields: { food, prod, gold, science, culture, influence },
  tech: { known[], available[], frozen[] },
  diplomacy: { relations, open_offers[] },
  legal_actions: [ ActionLite ... ]
}
```

acceptance_tests

Two headless clients can complete a toy match deterministically.

Replaying the same (seed, actions) produces the same event sequence and state_hash.

Idempotent resubmits do not duplicate effects.

machine‑readable (JSON)
```
{
  "service": "Match",
  "rpcs": [
    {"name":"CreateMatch","latency_budget_ms":100},
    {"name":"GetObservation","latency_budget_ms":50},
    {"name":"SubmitAction","latency_budget_ms":25},
    {"name":"Advance","latency_budget_ms":100},
    {"name":"Negotiate","latency_budget_ms":80}
  ],
  "idempotency": "sha256(match_id|turn|player|serialized_action|prev_state_hash[:8])",
  "acceptance": [
    "Deterministic completion by two headless clients",
    "Replays reproduce event sequence and state_hash",
    "Resubmits are deduped"
  ]
}
```

# Contract — SimCore (State, Actions, Determinism)

**problem:** Provide a deterministic strategy simulation core with strict action validation, legal‑action enumeration, and inter‑turn processing.  
**non_goals:** UI, content breadth, async multiplayer, deep naval/air (v0).  
**perf_budget:** On Standard map @ 6 AIs, end‑turn p50 ≤ **6s**, p95 ≤ **12s**; per‑opponent AI think ≤ **0.6s** (parallelizable). :contentReference[oaicite:3]{index=3}

## invariants
- 1UPT for combat units (civilian may co‑stack with 1 combat).
- No unit occupies OOB or impassable tiles; ZOC respected.
- City capacity/slots and costs honored; yields ≥ 0; storage caps enforced.
- Deterministic fixed‑tick order of systems (Upkeep → Yields → Events → AI Think → Digest).
- State transitions are pure given `(state, action)` and seeded RNG stream.
- No hidden mutations; replay re‑applies the same actions to produce identical hashes.

## types (public surface)
- **Ids:** `PlayerId`, `CityId`, `UnitId` = opaque newtypes (u64).  
- **Coords:** `TileCoord { x:i32, y:i32 }`  
- **State:** `{ turn:i32, map:Map, players:[Player], cities:[City], units:[Unit], tech:TechTree, policies:Policies, diplomacy:Diplomacy, rng:Seed, rules_ver:String }`  
- **Action (enum):** `EndTurn, MoveUnit{unit,path[],ap}, Attack{attacker,target}, Fortify{unit}, BuildUnit{city,kind}, BuildDistrict{city,kind,tile}, SetPolicy{slot,id}, ChooseTech{id}, OfferDeal{json}, AcceptDeal{id}, DeclineDeal{id}`  
- **Effects:** `{ deltas:[], events:[] }` (event‑sourced)

## functions (must exist)
- `enumerate_legal_actions(state:&State, player:PlayerId) -> Vec<Action>`
  - **pre:** state sane; player alive  
  - **post:** every returned action passes `validate_action`
- `validate_action(state:&State, action:&Action) -> Result<(), Error>`
  - **post:** failure reasons are specific & machine‑readable
- `apply_action(state:&mut State, action:Action) -> Result<Effects, Error>`
  - **post:** invariants hold; `Effects` contains emitted events only
- `end_turn(state:&mut State) -> Result<(), Error>`
  - **post:** runs inter‑turn pipeline in canonical order
- `state_hash(state:&State) -> Hash128`
  - **post:** identical on replay across OS/CPU for same seed/build

## acceptance_tests
- **Invariants:** 10M random action fuzz → 0 invariant breaks.
- **Determinism:** 100 seeds replay to identical `state_hash`.
- **Completeness:** `enumerate_legal_actions` contains any action later accepted by `validate_action`.
- **Illegal filters:** invalid moves (OOB, 1UPT clash, cost deficits) are rejected with precise codes.

## risks
- Floating‑point nondeterminism → use fixed‑point where order matters.
- Hidden iteration order → use stable maps/vectors; sort before fold.

### machine‑readable (JSON)
```json
{
  "problem": "Deterministic sim core with validated actions and canonical inter-turn order.",
  "invariants": [
    "1UPT combat; civilians may co-stack",
    "No OOB/impassable occupancy; ZOC obeyed",
    "Costs/slots enforced; yields non-negative",
    "Fixed pipeline order: Upkeep>Yields>Events>AI>Digest",
    "Pure transitions given (state, action, seed)",
    "No hidden mutations; event-sourced effects"
  ],
  "types": {
    "Id": ["PlayerId","CityId","UnitId"],
    "TileCoord": {"x":"i32","y":"i32"},
    "Action": ["EndTurn","MoveUnit","Attack","Fortify","BuildUnit","BuildDistrict","SetPolicy","ChooseTech","OfferDeal","AcceptDeal","DeclineDeal"]
  },
  "functions": [
    {"name":"enumerate_legal_actions","sig":"(&State, PlayerId) -> Vec<Action>"},
    {"name":"validate_action","sig":"(&State, &Action) -> Result<(), Error>"},
    {"name":"apply_action","sig":"(&mut State, Action) -> Result<Effects, Error>"},
    {"name":"end_turn","sig":"(&mut State) -> Result<(), Error>"},
    {"name":"state_hash","sig":"(&State) -> Hash128"}
  ],
  "acceptance_tests": [
    "10M random actions → 0 invariant breaks",
    "100 random seeds → identical state_hash on replay",
    "Any applied action must have been legal at enumeration time"
  ],
  "perf_budget": {"end_turn_p50_ms":6000,"end_turn_p95_ms":12000,"ai_think_per_opponent_ms":600}
}
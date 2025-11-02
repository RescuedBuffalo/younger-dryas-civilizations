# Civ Remake — Headless‑First 4X (Repo Bootstrap)

**Status:** Kickoff • **Mode:** Headless‑first sim with gRPC services • **Principles:** Determinism, legibility, fast turns

This repo ships a deterministic SimCore and a headless Match service first, then layers on AI and a minimal debug UI. Budgets, loops, and acceptance tests follow the Living GDD. :contentReference[oaicite:1]{index=1}

## What’s here (today)
- `ENGINEERING_LOG.md` — our running plan & request queue
- `docs/contracts/` — Contract Cards the code must obey
- `schemas/` & `protos/` — will hold JSON Schemas & .proto (filled by Coder)
- `tools/` — perf & replay tools (later)

## Next actions (copy these into Cursor)
1. **Reasoner is done** for phase‑1: contracts below.  
2. **Coder:** scaffold the Rust workspace + crates (see _Scaffold spec_).  
3. **Test‑Author:** generate property tests from the contracts.

### Scaffold spec (for the Coder)
Create a Cargo workspace with crates:
- simcore/ # deterministic ECS, rules engine, legal-action generator
- services/match/ # gRPC service (tonic) exposing CreateMatch/Observe/Act/Advance/Negotiate
- ai/eval/ # value functions & feature extraction
- ai/planner/ # planner/search harness (stubs at first)
- ai/negotiator/ # LLM-facing adapter (stubs + schema validation)
- client/cli/ # headless runner for local matches, replays
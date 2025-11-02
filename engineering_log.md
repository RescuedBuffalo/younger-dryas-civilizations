# Civ Remake — Engineering Log (Running)
**Owner:** <you> • **Manager:** ChatGPT EM • **Last Updated:** <YYYY‑MM‑DD>  
**Source GDD:** ./docs/civ_style_strategy_living_game_design_doc_gdd_template_v_0.md  <!-- Budget & loop anchor --> :contentReference[oaicite:2]{index=2}

## 0) Project Status Snapshot
- **Current Milestone:** M0 — SimCore skeleton + deterministic replay
- **Build Hash:** (fill after first commit)
- **Perf:** inter‑turn p50/p95 TBD
- **Risks (Top 3):** empty repo; scope creep; determinism drift
- **Next Decision Date:** <YYYY‑MM‑DD>

## 1) This Week’s Objectives
- [ ] Workspace scaffolding builds & tests (CI: fmt, clippy, unit)
- [ ] Implement `state_hash`, legal‑action enumerators (stubs ok)
- [ ] gRPC `.proto` drafted from Match contract

**Exit Criteria / Acceptance**
- `cargo test` green on clean machine; `state_hash` exists
- Contract Cards copied into `docs/contracts/`

## 2) Request Queue (from EM)
- [ ] Paste workspace tree after scaffolding
- [ ] Paste `.proto` for Match service
- [ ] Paste JSON Schema draft for Deal DSL
- [ ] Paste `simcore/src/lib.rs` and `simcore/src/actions.rs` after stubbing

## 3) Decisions Log
| Date | Decision | Options | Rationale | Impact |
|---|---|---|---|---|
| YYYY‑MM‑DD | Rust+Bevy core; tonic gRPC | Unity/Godot+Rust | Determinism, headless perf, control | Self‑play & testing first‑class |

## 4) Telemetry & Perf (to fill later)
- p50 turn time:  
- p95 turn time:  
- determinism (state_hash): pass/fail
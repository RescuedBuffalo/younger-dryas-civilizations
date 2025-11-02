Contract — Combat v1 (Deterministic)

problem: Resolve tactical exchanges without coin‑flip RNG; emphasize terrain, flanks, supply, and morale.
perf_budget: Per exchange ≤ 0.2ms in microbench on Standard units.

invariants

Simultaneous exchange; ranged counter‑fire only if in range.

Fractional damage is banked as “wounds”; apply when ≥1.

River, elevation, forts, terrain apply as modifiers; caps enforced.

constants (initial)

K_melee=22, K_ranged=18, alpha=0.5

Flank +12% per extra adjacent attacker (cap +36%)

River ‑15% to attacker; High‑ground +10% defender

Fortify +10%/turn (cap +25%); Out‑of‑supply ‑10%/turn (cap ‑30%)

Terrain DEF: Forest +25%, Hills +20%, Marsh +10%, City +30% (terrain+fort cap +45%)

functions (interface)

resolve_melee(attacker, defender, context) -> {to_def:int, to_att:int, banks:{a:fp,d:fp}}

resolve_ranged(attacker, defender, context) -> {to_def:int, to_att:int, banks:{a:fp,d:fp}}

acceptance_tests

Symmetric matchup yields equal damage.

Each +12% flank step increases EV monotonically.

Siege reduces lethality until breach (city/fort modifier active).

machine‑readable (JSON)
```
{
  "constants": {
    "K_melee": 22, "K_ranged": 18, "alpha": 0.5,
    "flank_step": 0.12, "flank_cap": 0.36,
    "river_penalty": -0.15, "high_ground_def": 0.10,
    "fortify_per_turn": 0.10, "fortify_cap": 0.25,
    "oos_per_turn": -0.10, "oos_cap": -0.30,
    "terrain_caps": 0.45
  },
  "terrain_def": {"Forest":0.25,"Hills":0.20,"Marsh":0.10,"City":0.30}
}
```
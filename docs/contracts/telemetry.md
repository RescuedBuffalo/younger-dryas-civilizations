Contract — Telemetry (Event‑Sourced, Determinism & Perf)

problem: Capture events for replay verification, perf, AI quality, and balance without exceeding budgets.
perf_budget: ≤ 5% of turn time in release builds; deterministic state_hash every tick.

streams (Parquet/Arrow)

match_meta{match_id,seed,map,players,build_hash,rules_ver}

turn_summary{turn,player,yields,army_power,fame,tech_tier,constraints[]}

state_hash{tick,hash,platform}

action_event{turn,player,legal_set,action_json,ev,latency_ms,action_id}

search_trace{turn,phase,nodes,depth,score,cutoff,best_line}

plan_event{candidates[],adopted,why}

deal_event{proposal,response,trust_delta}

battle_record{loc,attacker,defender,mods,damage,result,ler}

perf_counter{interturn:{upkeep_ms,yields_ms,events_ms,ai_ms,digest_ms}}

crash_event{panic,backtrace,last_replay_ptr}

acceptance_tests

100 seeds replay: 0 hash mismatches.

1M events/hr stress: ≤1% latency overage; no loss.

machine‑readable (JSON)
```
{
  "streams": [
    "match_meta","turn_summary","state_hash","action_event","search_trace",
    "plan_event","deal_event","battle_record","perf_counter","crash_event"
  ],
  "budgets": {"overhead_fraction_max": 0.05},
  "determinism_check": "xxh3_128 hash per tick; must match on replay"
}
```